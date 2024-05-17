use crate::database::{redis_conn, redis_key};
use crate::federation::acct::Acct;
use crate::misc::check_hit_antenna::{check_hit_antenna, AntennaCheckError};
use crate::model::entity::{antenna, note};
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
    #[error("Failed to check if the note should be added to antenna: {0}")]
    AntennaCheckErr(#[from] AntennaCheckError),
}

// https://github.com/napi-rs/napi-rs/issues/2060
type Antenna = antenna::Model;
type Note = note::Model;

#[crate::export]
pub async fn update_antenna_on_create_note(
    antenna: &Antenna,
    note: Note,
    note_author: &Acct,
) -> Result<(), Error> {
    if check_hit_antenna(antenna, note.clone(), note_author).await? {
        add_note_to_antenna(&antenna.id, &note)?;
    }
    Ok(())
}

pub fn add_note_to_antenna(antenna_id: &str, note: &Note) -> Result<(), Error> {
    // for timeline API
    redis_conn()?.xadd_maxlen(
        redis_key(format!("antennaTimeline:{}", antenna_id)),
        StreamMaxlen::Approx(200),
        format!("{}-*", get_timestamp(&note.id)?),
        &[("note", &note.id)],
    )?;

    // for streaming API
    Ok(stream::antenna::publish(antenna_id.to_string(), note)?)
}
