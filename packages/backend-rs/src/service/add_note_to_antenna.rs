use crate::database::{redis_conn, redis_key};
use crate::service::stream::{publish_to_stream, Error, Stream};
use crate::util::id::get_timestamp;
use redis::{streams::StreamMaxlen, Commands};

#[crate::export]
pub fn add_note_to_antenna(antenna_id: &str, note_id: &str) -> Result<(), Error> {
    redis_conn()?.xadd_maxlen(
        redis_key(format!("antennaTimeline:{}", antenna_id)),
        StreamMaxlen::Approx(200),
        format!("{}-*", get_timestamp(note_id)),
        &[("note", note_id)],
    )?;

    let stream = Stream::Antenna {
        id: antenna_id.to_string(),
    };
    publish_to_stream(
        &stream,
        Some("note"),
        Some(format!("{{ \"id\": \"{}\" }}", note_id)),
    )
}
