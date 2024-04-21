use crate::database::{redis_conn, redis_key};
use crate::service::stream::{publish_to_stream, Error, Stream};
use crate::model::entity::note;
use crate::util::id::get_timestamp;
use redis::{streams::StreamMaxlen, Commands};

type Note = note::Model;

#[crate::export]
pub fn add_note_to_antenna(antenna_id: String, note: &Note) -> Result<(), Error> {
    redis_conn()?.xadd_maxlen(
        redis_key(format!("antennaTimeline:{}", antenna_id)),
        StreamMaxlen::Approx(200),
        format!("{}-*", get_timestamp(&note.id)),
        &[("note", &note.id)],
    )?;

    publish_to_stream(
        &Stream::Antenna { antenna_id },
        Some("note"),
        Some(serde_json::to_string(note)?),
    )
}
