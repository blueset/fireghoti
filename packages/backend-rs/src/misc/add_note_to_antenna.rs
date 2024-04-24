use crate::database::{redis_conn, redis_key};
use crate::model::entity::note;
use crate::service::stream;
use crate::util::id::{get_timestamp, InvalidIdErr};
use redis::{streams::StreamMaxlen, Commands, RedisError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Redis error: {0}")]
    RedisErr(#[from] RedisError),
    #[error("Invalid ID: {0}")]
    InvalidIdErr(#[from] InvalidIdErr),
    #[error("Stream error: {0}")]
    StreamErr(#[from] stream::Error),
}

type Note = note::Model;

#[crate::export]
pub fn add_note_to_antenna(antenna_id: String, note: &Note) -> Result<(), Error> {
    // for timeline API
    redis_conn()?.xadd_maxlen(
        redis_key(format!("antennaTimeline:{}", antenna_id)),
        StreamMaxlen::Approx(200),
        format!("{}-*", get_timestamp(&note.id)?),
        &[("note", &note.id)],
    )?;

    // for streaming API
    Ok(stream::antenna::publish(antenna_id, note)?)
}
