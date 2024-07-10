pub mod antenna;
pub mod channel;
pub mod chat;
pub mod chat_index;
pub mod custom_emoji;
pub mod drive;
pub mod group_chat;
pub mod moderation;
pub mod notes;

use crate::{
    config::CONFIG,
    database::{redis_conn, RedisConnError},
};
use redis::{AsyncCommands, RedisError};

pub enum Stream {
    Internal,
    CustomEmoji,
    Moderation {
        moderator_id: String,
    },
    User {
        user_id: String,
    },
    Channel {
        channel_id: String,
    },
    Note {
        note_id: String,
    },
    Notes,
    UserList {
        list_id: String,
    },
    Main {
        user_id: String,
    },
    Drive {
        user_id: String,
    },
    Antenna {
        antenna_id: String,
    },
    Chat {
        sender_user_id: String,
        receiver_user_id: String,
    },
    GroupChat {
        group_id: String,
    },
    ChatIndex {
        user_id: String,
    },
}

#[macros::export(string_enum = "camelCase")]
pub enum ChatEvent {
    Message,
    Read,
    Deleted,
    Typing,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to execute a Redis command")]
    Redis(#[from] RedisError),
    #[error("bad Redis connection")]
    RedisConn(#[from] RedisConnError),
    #[error("failed to (de)serialize object")]
    Json(#[from] serde_json::Error),
    #[error("invalid content")]
    InvalidContent,
}

pub async fn publish_to_stream(
    stream: &Stream,
    kind: Option<&str>,
    value: Option<String>,
) -> Result<(), Error> {
    let channel = match stream {
        Stream::Internal => "internal".to_string(),
        Stream::CustomEmoji => "broadcast".to_string(),
        Stream::Moderation { moderator_id } => format!("adminStream:{moderator_id}"),
        Stream::User { user_id } => format!("user:{user_id}"),
        Stream::Channel { channel_id } => format!("channelStream:{channel_id}"),
        Stream::Note { note_id } => format!("noteStream:{note_id}"),
        Stream::Notes => "notesStream".to_string(),
        Stream::UserList { list_id } => format!("userListStream:{list_id}"),
        Stream::Main { user_id } => format!("mainStream:{user_id}"),
        Stream::Drive { user_id } => format!("driveStream:{user_id}"),
        Stream::Antenna { antenna_id } => format!("antennaStream:{antenna_id}"),
        Stream::Chat {
            sender_user_id,
            receiver_user_id,
        } => format!("messagingStream:{sender_user_id}-{receiver_user_id}"),
        Stream::GroupChat { group_id } => format!("messagingStream:{group_id}"),
        Stream::ChatIndex { user_id } => format!("messagingIndexStream:{user_id}"),
    };

    let message = if let Some(kind) = kind {
        format!(
            "{{\"type\":\"{}\",\"body\":{}}}",
            kind,
            value.unwrap_or_else(|| "null".to_string()),
        )
    } else {
        value.ok_or(Error::InvalidContent)?
    };

    redis_conn()
        .await?
        .publish(
            &CONFIG.host,
            format!("{{\"channel\":\"{}\",\"message\":{}}}", channel, message),
        )
        .await?;

    Ok(())
}
