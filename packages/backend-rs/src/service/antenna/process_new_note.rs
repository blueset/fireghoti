use crate::database::{cache, db_conn, redis_conn, redis_key, RedisConnError};
use crate::federation::acct::Acct;
use crate::misc::get_note_all_texts::{all_texts, PartialNoteToElaborate};
use crate::model::entity::{antenna, note};
use crate::service::antenna::check_hit::{check_hit_antenna, AntennaCheckError};
use crate::service::stream;
use crate::util::id::{get_timestamp, InvalidIdError};
use redis::{streams::StreamMaxlen, AsyncCommands, RedisError};
use sea_orm::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Db(#[from] DbErr),
    #[error("Cache error: {0}")]
    Cache(#[from] cache::Error),
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),
    #[error("Redis connection error: {0}")]
    RedisConn(#[from] RedisConnError),
    #[error("Invalid ID: {0}")]
    InvalidId(#[from] InvalidIdError),
    #[error("Stream error: {0}")]
    Stream(#[from] stream::Error),
    #[error("Failed to check if the note should be added to antenna: {0}")]
    AntennaCheck(#[from] AntennaCheckError),
}

// for napi export
// https://github.com/napi-rs/napi-rs/issues/2060
type Antenna = antenna::Model;
type Note = note::Model;

// TODO?: it might be better to store this directly in memory
// (like fetch_meta) instead of Redis as it's used so much
async fn antennas() -> Result<Vec<Antenna>, Error> {
    const CACHE_KEY: &str = "antennas";

    if let Some(antennas) = cache::get::<Vec<Antenna>>(CACHE_KEY).await? {
        Ok(antennas)
    } else {
        let antennas = antenna::Entity::find().all(db_conn().await?).await?;
        cache::set(CACHE_KEY, &antennas, 5 * 60).await?;
        Ok(antennas)
    }
}

#[crate::export]
pub async fn update_antennas_on_new_note(
    note: Note,
    note_author: &Acct,
    note_muted_users: &[String],
) -> Result<(), Error> {
    let note_cloned = note.clone();
    let note_all_texts = all_texts(
        PartialNoteToElaborate {
            file_ids: note.file_ids,
            user_id: note.user_id,
            text: note.text,
            cw: note.cw,
            renote_id: note.renote_id,
            reply_id: note.reply_id,
        },
        false,
    )
    .await?;

    // TODO: do this in parallel
    for antenna in antennas().await?.iter() {
        if note_muted_users.contains(&antenna.user_id) {
            continue;
        }
        if check_hit_antenna(antenna, &note_cloned, &note_all_texts, note_author).await? {
            add_note_to_antenna(&antenna.id, &note_cloned).await?;
        }
    }

    Ok(())
}

async fn add_note_to_antenna(antenna_id: &str, note: &Note) -> Result<(), Error> {
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
