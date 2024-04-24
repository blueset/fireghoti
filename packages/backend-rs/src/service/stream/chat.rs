use crate::service::stream::{publish_to_stream, Error, Stream};

#[derive(strum::Display)]
#[crate::export(string_enum = "camelCase")]
pub enum ChatEvent {
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "read")]
    Read,
    #[strum(serialize = "deleted")]
    Deleted,
    #[strum(serialize = "typing")]
    Typing,
}

#[crate::export(js_name = "publishToChatStream")]
pub fn publish(
    sender_user_id: String,
    receiver_user_id: String,
    kind: ChatEvent,
    object: &serde_json::Value, // TODO?: change this to enum
) -> Result<(), Error> {
    publish_to_stream(
        &Stream::Chat {
            sender_user_id,
            receiver_user_id,
        },
        Some(kind.to_string()),
        Some(serde_json::to_string(object)?),
    )
}
