use crate::service::stream::{chat::ChatEvent, publish_to_stream, Error, Stream};

// We want to merge `kind` and `object` into a single enum
// https://github.com/napi-rs/napi-rs/issues/2036

#[crate::export(js_name = "publishToGroupChatStream")]
pub fn publish(group_id: String, kind: ChatEvent, object: &serde_json::Value) -> Result<(), Error> {
    publish_to_stream(
        &Stream::GroupChat { group_id },
        Some(kind.to_string()),
        Some(serde_json::to_string(object)?),
    )
}
