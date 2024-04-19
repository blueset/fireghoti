use crate::database::{redis_conn, redis_key};
use crate::model::entity::note;
use crate::service::stream::{publish, Error, Stream};
use crate::util::id::get_timestamp;
use redis::{streams::StreamMaxlen, Commands};

#[crate::export]
pub fn add_note_to_antenna(antenna_id: &str, note: &note::Model) -> Result<(), Error> {
    redis_conn()?.xadd_maxlen(
        redis_key(format!("antennaTimeline:{}", antenna_id)),
        StreamMaxlen::Approx(200),
        format!("{}-*", get_timestamp(&note.id)),
        &[("note", &note.id)],
    )?;

    let stream = Stream::Antenna {
        id: antenna_id.to_string(),
    };
    publish(&stream, Some("note"), Some(serde_json::to_value(note)?))
}
