use crate::database::{redis_conn, redis_key};
use crate::model::entity::note;
use crate::service::stream;
use crate::util::id::get_timestamp;
use redis::{streams::StreamMaxlen, Commands};

type Note = note::Model;

#[crate::export]
pub fn add_note_to_antenna(antenna_id: String, note: &Note) -> Result<(), stream::Error> {
    // for timeline API
    redis_conn()?.xadd_maxlen(
        redis_key(format!("antennaTimeline:{}", antenna_id)),
        StreamMaxlen::Approx(200),
        format!("{}-*", get_timestamp(&note.id)),
        &[("note", &note.id)],
    )?;

    // for streaming API
    stream::antenna::publish(antenna_id, note)
}
