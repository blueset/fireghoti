use crate::database::cache;
use crate::database::{db_conn, redis_conn, redis_key};
use crate::federation::acct::Acct;
use crate::misc::check_hit_antenna::{check_hit_antenna, AntennaCheckError};
use crate::model::entity::{antenna, note};
use crate::service::stream;
use crate::util::id::{get_timestamp, InvalidIdErr};
use redis::{streams::StreamMaxlen, AsyncCommands, RedisError};
use sea_orm::{DbErr, EntityTrait};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DbErr(#[from] DbErr),
    #[error("Cache error: {0}")]
    CacheErr(#[from] cache::Error),
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

// TODO?: it might be better to store this directly in memory
// (like fetch_meta) instead of Redis as it's used so much
async fn antennas() -> Result<Vec<Antenna>, Error> {
    const CACHE_KEY: &str = "antennas";

    Ok(cache::get::<Vec<Antenna>>(CACHE_KEY).await?.unwrap_or({
        let antennas = antenna::Entity::find().all(db_conn().await?).await?;
        cache::set(CACHE_KEY, &antennas, 5 * 60).await?;
        antennas
    }))
}

#[crate::export]
pub async fn update_antennas_on_new_note(
    note: Note,
    note_author: &Acct,
    note_muted_users: Vec<String>,
) -> Result<(), Error> {
    // TODO: do this in parallel
    for antenna in antennas().await?.iter() {
        if note_muted_users.contains(&antenna.user_id) {
            continue;
        }
        if check_hit_antenna(antenna, note.clone(), note_author).await? {
            add_note_to_antenna(&antenna.id, &note).await?;
        }
    }

    Ok(())
}

pub async fn add_note_to_antenna(antenna_id: &str, note: &Note) -> Result<(), Error> {
    // for timeline API
    redis_conn()
        .await?
        .xadd_maxlen(
            redis_key(format!("antennaTimeline:{}", antenna_id)),
            StreamMaxlen::Approx(200),
            format!("{}-*", get_timestamp(&note.id)?),
            &[("note", &note.id)],
        )
        .await?;

    // for streaming API
    Ok(stream::antenna::publish(antenna_id.to_string(), note).await?)
}
