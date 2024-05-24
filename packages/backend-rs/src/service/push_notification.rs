use crate::database::db_conn;
use crate::misc::get_note_summary::{get_note_summary, NoteLike};
use crate::misc::meta::fetch_meta;
use crate::model::entity::{access_token, sw_subscription};
use crate::util::http_client;
use once_cell::sync::OnceCell;
use sea_orm::{prelude::*, DbErr};
use web_push::{
    ContentEncoding, IsahcWebPushClient, SubscriptionInfo, SubscriptionKeys, VapidSignatureBuilder,
    WebPushClient, WebPushError, WebPushMessageBuilder,
};

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

#[derive(strum::Display, PartialEq)]
#[crate::export(string_enum = "camelCase")]
pub enum PushNotificationKind {
    #[strum(serialize = "notification")]
    Generic,
    #[strum(serialize = "unreadMessagingMessage")]
    Chat,
    #[strum(serialize = "readAllMessagingMessages")]
    ReadAllChats,
    #[strum(serialize = "readAllMessagingMessagesOfARoom")]
    ReadAllChatsInTheRoom,
    #[strum(serialize = "readNotifications")]
    ReadNotifications,
    #[strum(serialize = "readAllNotifications")]
    ReadAllNotifications,
    Mastodon,
}

fn compact_content(
    kind: &PushNotificationKind,
    mut content: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    if kind != &PushNotificationKind::Generic {
        return Ok(content);
    }

    let object = content
        .as_object_mut()
        .ok_or(Error::InvalidContent("not a JSON object".to_string()))?;

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

    let note_like: NoteLike = serde_json::from_value(note.clone())?;
    let text = get_note_summary(note_like);

    let note_object = note.as_object_mut().unwrap();

    note_object.remove("reply");
    note_object.remove("renote");
    note_object.remove("user");
    note_object.insert("text".to_string(), text.into());
    object.insert("note".to_string(), note);

    Ok(serde_json::from_value(Json::Object(object.clone()))?)
}

async fn encode_mastodon_payload(
    mut content: serde_json::Value,
    db: &DatabaseConnection,
    subscription: &sw_subscription::Model,
) -> Result<String, Error> {
    let object = content
        .as_object_mut()
        .ok_or(Error::InvalidContent("not a JSON object".to_string()))?;

    let token_id = subscription
        .app_access_token_id
        .as_ref()
        .ok_or(Error::InvalidContent("no access token".to_string()))?;
    let token = access_token::Entity::find()
        .filter(access_token::Column::Id.eq(token_id))
        .one(db)
        .await?
        .ok_or(Error::InvalidContent("access token not found".to_string()))?;

    if token.app_id.is_none() {
        return Err(Error::InvalidContent("no app ID".to_string()));
    }

    object.insert(
        "access_token".to_string(),
        serde_json::to_value(token.token)?,
    );

    Ok(serde_json::to_string(&content)?)
}

async fn handle_web_push_failure(
    db: &DatabaseConnection,
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
    let meta = fetch_meta(true).await?;

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

    // TODO: refactoring
    let mut payload = if kind == PushNotificationKind::Mastodon {
        // Content generated per subscription
        "".to_string()
    } else {
        // Format the `content` passed from the TypeScript backend
        // for Firefish push notifications
        format!(
            "{{\"type\":\"{}\",\"userId\":\"{}\",\"dateTime\":{},\"body\":{}}}",
            kind,
            receiver_user_id,
            chrono::Utc::now().timestamp_millis(),
            serde_json::to_string(&compact_content(&kind, content.clone())?)?
        )
    };
    tracing::trace!("payload: {}", payload);

    let encoding = if kind == PushNotificationKind::Mastodon {
        ContentEncoding::AesGcm
    } else {
        ContentEncoding::Aes128Gcm
    };

    for subscription in subscriptions.iter() {
        if !subscription.send_read_message
            && [
                PushNotificationKind::ReadAllChats,
                PushNotificationKind::ReadAllChatsInTheRoom,
                PushNotificationKind::ReadAllNotifications,
                PushNotificationKind::ReadNotifications,
            ]
            .contains(&kind)
        {
            continue;
        }

        if kind == PushNotificationKind::Mastodon {
            if subscription.app_access_token_id.is_none() {
                continue;
            }
            payload = encode_mastodon_payload(content.clone(), db, subscription).await?;
        } else if subscription.app_access_token_id.is_some() {
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
