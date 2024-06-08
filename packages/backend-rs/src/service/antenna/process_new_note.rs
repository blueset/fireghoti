use crate::{
    database::{cache, redis_conn, redis_key, RedisConnError},
    federation::acct::Acct,
    misc::get_note_all_texts::all_texts,
    model::entity::note,
    service::{
        antenna,
        antenna::check_hit::{check_hit_antenna, AntennaCheckError},
        stream,
    },
    util::id::{get_timestamp, InvalidIdError},
};
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
type Note = note::Model;

#[crate::export]
pub async fn update_antennas_on_new_note(
    note: &Note,
    note_author: &Acct,
    note_muted_users: &[String],
) -> Result<(), Error> {
    let note_all_texts = all_texts(
        note.file_ids.to_owned(),
        note.text.to_owned(),
        note.cw.to_owned(),
        note.renote_id.to_owned(),
        note.reply_id.to_owned(),
        false,
    )
    .await?;

    // TODO: do this in parallel
    for antenna in antenna::cache::get().await?.iter() {
        if note_muted_users.contains(&antenna.user_id) {
            continue;
        }
        if check_hit_antenna(antenna, note, &note_all_texts, note_author).await? {
            add_note_to_antenna(&antenna.id, note).await?;
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
    stream::antenna::publish(antenna_id.to_string(), note).await?;

    Ok(())
}
