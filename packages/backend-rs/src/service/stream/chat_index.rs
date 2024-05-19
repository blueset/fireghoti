use crate::service::stream::{publish_to_stream, Error, Stream};

#[derive(strum::Display)]
#[crate::export(string_enum = "camelCase")]
pub enum ChatIndexEvent {
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "read")]
    Read,
}

// We want to merge `kind` and `object` into a single enum
// https://github.com/napi-rs/napi-rs/issues/2036

#[crate::export(js_name = "publishToChatIndexStream")]
pub async fn publish(
    user_id: String,
    kind: ChatIndexEvent,
    object: &serde_json::Value,
) -> Result<(), Error> {
    publish_to_stream(
        &Stream::ChatIndex { user_id },
        Some(kind.to_string()),
        Some(serde_json::to_string(object)?),
    )
    .await
}
