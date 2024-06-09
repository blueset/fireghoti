use crate::{
    config::local_server_info, database::db_conn, misc::note::summarize,
    model::entity::sw_subscription, util::http_client,
};
use once_cell::sync::OnceCell;
use sea_orm::prelude::*;
use serde::Deserialize;
use web_push::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Db(#[from] DbErr),
    #[error("Web Push error: {0}")]
    WebPush(#[from] WebPushError),
    #[error("Failed to (de)serialize an object: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Invalid content: {0}")]
    InvalidContent(String),
    #[error("HTTP client aquisition error: {0}")]
    HttpClient(#[from] http_client::Error),
}

static CLIENT: OnceCell<IsahcWebPushClient> = OnceCell::new();

fn get_client() -> Result<IsahcWebPushClient, Error> {
    Ok(CLIENT
        .get_or_try_init(|| http_client::client().map(IsahcWebPushClient::from))
        .cloned()?)
}

#[crate::export]
pub enum PushNotificationKind {
    Generic,
    Chat,
    ReadAllChats,
    ReadAllChatsInTheRoom,
    ReadNotifications,
    ReadAllNotifications,
    Mastodon,
}

fn compact_content(mut content: serde_json::Value) -> Result<serde_json::Value, Error> {
    if !content.is_object() {
        return Err(Error::InvalidContent("not a JSON object".to_string()));
    }

    let object = content.as_object_mut().unwrap();

    if !object.contains_key("note") {
        return Ok(content);
    }

    let mut note = if object.contains_key("type") && object.get("type").unwrap() == "renote" {
        object
            .get("note")
            .unwrap()
            .get("renote")
            .ok_or(Error::InvalidContent(
                "renote object is missing".to_string(),
            ))?
    } else {
        object.get("note").unwrap()
    }
    .clone();

    if !note.is_object() {
        return Err(Error::InvalidContent(
            "(re)note is not an object".to_string(),
        ));
    }

    // TODO: get rid of this struct
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PartialNote {
        file_ids: Vec<String>,
        text: Option<String>,
        cw: Option<String>,
        has_poll: bool,
    }

    let note_like: PartialNote = serde_json::from_value(note.clone())?;
    let text = summarize!(note_like);

    let note_object = note.as_object_mut().unwrap();

    note_object.remove("reply");
    note_object.remove("renote");
    note_object.remove("user");
    note_object.insert("text".to_string(), text.into());
    object.insert("note".to_string(), note);

    Ok(serde_json::from_value(Json::Object(object.clone()))?)
}

async fn handle_web_push_failure(
    db: &DbConn,
    err: WebPushError,
    subscription_id: &str,
    error_message: &str,
) -> Result<(), DbErr> {
    match err {
        WebPushError::BadRequest(_)
        | WebPushError::ServerError(_)
        | WebPushError::InvalidUri
        | WebPushError::EndpointNotValid
        | WebPushError::EndpointNotFound
        | WebPushError::TlsError
        | WebPushError::SslError
        | WebPushError::InvalidPackageName
        | WebPushError::MissingCryptoKeys
        | WebPushError::InvalidCryptoKeys
        | WebPushError::InvalidResponse => {
            sw_subscription::Entity::delete_by_id(subscription_id)
                .exec(db)
                .await?;
            tracing::info!("{}; {} was unsubscribed", error_message, subscription_id);
            tracing::debug!("reason: {:#?}", err);
        }
        _ => {
            tracing::warn!("{}; subscription id: {}", error_message, subscription_id);
            tracing::info!("reason: {:#?}", err);
        }
    };

    Ok(())
}

#[crate::export]
pub async fn send_push_notification(
    receiver_user_id: &str,
    kind: PushNotificationKind,
    content: &serde_json::Value,
) -> Result<(), Error> {
    let meta = local_server_info().await?;

    if !meta.enable_service_worker || meta.sw_public_key.is_none() || meta.sw_private_key.is_none()
    {
        return Ok(());
    }

    let db = db_conn().await?;

    let signature_builder = VapidSignatureBuilder::from_base64_no_sub(
        meta.sw_private_key.unwrap().as_str(),
        web_push::URL_SAFE_NO_PAD,
    )?;

    let subscriptions = sw_subscription::Entity::find()
        .filter(sw_subscription::Column::UserId.eq(receiver_user_id))
        .all(db)
        .await?;

    let use_mastodon_api = matches!(kind, PushNotificationKind::Mastodon);

    // TODO: refactoring
    let payload = if use_mastodon_api {
        // Leave the `content` as it is
        serde_json::to_string(content)?
    } else {
        // Format the `content` passed from the TypeScript backend
        // for Firefish push notifications
        let label = match kind {
            PushNotificationKind::Generic => "notification",
            PushNotificationKind::Chat => "unreadMessagingMessage",
            PushNotificationKind::ReadAllChats => "readAllMessagingMessages",
            PushNotificationKind::ReadAllChatsInTheRoom => "readAllMessagingMessagesOfARoom",
            PushNotificationKind::ReadNotifications => "readNotifications",
            PushNotificationKind::ReadAllNotifications => "readAllNotifications",
            // unreachable
            _ => "unknown",
        };
        format!(
            "{{\"type\":\"{}\",\"userId\":\"{}\",\"dateTime\":{},\"body\":{}}}",
            label,
            receiver_user_id,
            chrono::Utc::now().timestamp_millis(),
            match kind {
                PushNotificationKind::Generic =>
                    serde_json::to_string(&compact_content(content.to_owned())?)?,
                _ => serde_json::to_string(&content)?,
            }
        )
    };
    tracing::trace!("payload: {}", payload);

    let encoding = if use_mastodon_api {
        ContentEncoding::AesGcm
    } else {
        ContentEncoding::Aes128Gcm
    };

    for subscription in subscriptions.iter() {
        if !subscription.send_read_message
            && matches!(
                kind,
                PushNotificationKind::ReadAllChats
                    | PushNotificationKind::ReadAllChatsInTheRoom
                    | PushNotificationKind::ReadAllNotifications
                    | PushNotificationKind::ReadNotifications
            )
        {
            continue;
        }

        let subscription_info = SubscriptionInfo {
            endpoint: subscription.endpoint.to_owned(),
            keys: SubscriptionKeys {
                // convert standard base64 into base64url
                // https://en.wikipedia.org/wiki/Base64#Variants_summary_table
                p256dh: subscription
                    .publickey
                    .replace('+', "-")
                    .replace('/', "_")
                    .to_owned(),
                auth: subscription
                    .auth
                    .replace('+', "-")
                    .replace('/', "_")
                    .to_owned(),
            },
        };

        let signature = signature_builder
            .clone()
            .add_sub_info(&subscription_info)
            .build();

        if let Err(err) = signature {
            handle_web_push_failure(db, err, &subscription.id, "failed to build a signature")
                .await?;
            continue;
        }

        let mut message_builder = WebPushMessageBuilder::new(&subscription_info);
        message_builder.set_ttl(1000);
        message_builder.set_payload(encoding, payload.as_bytes());
        message_builder.set_vapid_signature(signature.unwrap());

        let message = message_builder.build();

        if let Err(err) = message {
            handle_web_push_failure(db, err, &subscription.id, "failed to build a payload").await?;
            continue;
        }
        if let Err(err) = get_client()?.send(message.unwrap()).await {
            handle_web_push_failure(db, err, &subscription.id, "failed to send").await?;
            continue;
        }

        tracing::debug!("success; subscription id: {}", subscription.id);
    }

    Ok(())
}
