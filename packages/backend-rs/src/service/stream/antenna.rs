use crate::model::entity::note;
use crate::service::stream::{publish_to_stream, Error, Stream};

pub fn publish(antenna_id: String, note: &note::Model) -> Result<(), Error> {
    publish_to_stream(
        &Stream::Antenna { antenna_id },
        Some("note"),
        Some(serde_json::to_string(note)?),
    )
}
