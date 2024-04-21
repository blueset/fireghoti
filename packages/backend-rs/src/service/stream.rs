use crate::config::server::CONFIG;
use crate::database::redis_conn;
use redis::{Commands, RedisError};

#[derive(strum::Display, serde::Serialize)]
pub enum Stream {
    #[strum(serialize = "internal")]
    Internal,
    #[strum(serialize = "broadcast")]
    Broadcast,
    #[strum(to_string = "adminStream:{id}")]
    Admin { id: String },
    #[strum(to_string = "user:{id}")]
    User { id: String },
    #[strum(to_string = "channelStream:{id}")]
    Channel { id: String },
    #[strum(to_string = "noteStream:{id}")]
    Note { id: String },
    #[strum(serialize = "notesStream")]
    Notes,
    #[strum(to_string = "userListStream:{id}")]
    UserList { id: String },
    #[strum(to_string = "mainStream:{id}")]
    Main { id: String },
    #[strum(to_string = "driveStream:{id}")]
    Drive { id: String },
    #[strum(to_string = "antennaStream:{id}")]
    Antenna { id: String },
    #[strum(to_string = "messagingStream:{id_1}-{id_2}")]
    Chat { id_1: String, id_2: String },
    #[strum(to_string = "messagingStream:{id}")]
    GroupChat { id: String },
    #[strum(to_string = "messagingIndexStream:{id}")]
    MessagingIndex { id: String },
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("Json (de)serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Value error: {0}")]
    ValueError(String),
}

pub fn publish_to_stream(
    stream: &Stream,
    kind: Option<&str>,
    value: Option<String>,
) -> Result<(), Error> {
    let message = if let Some(kind) = kind {
        format!(
            "{{ \"type\": \"{}\", \"body\": {} }}",
            kind,
            match value {
                Some(v) => v,
                None => "null".to_string(),
            }
        )
    } else {
        value.ok_or(Error::ValueError("Invalid streaming message".to_string()))?
    };

    redis_conn()?.publish(
        &CONFIG.host,
        format!(
            "{{ \"channel\": \"{}\", \"message\": {} }}",
            stream, message,
        ),
    )?;

    Ok(())
}

#[cfg(test)]
mod unit_test {
    use super::Stream;
    use pretty_assertions::assert_eq;

    #[test]
    fn channel_to_string() {
        assert_eq!(Stream::Internal.to_string(), "internal");
        assert_eq!(Stream::Broadcast.to_string(), "broadcast");
        assert_eq!(
            Stream::Admin {
                id: "9tb42br63g5apjcq".to_string()
            }
            .to_string(),
            "adminStream:9tb42br63g5apjcq"
        );
    }
}
