//! In-memory antennas cache handler

use crate::{database::db_conn, model::entity::antenna};
use sea_orm::prelude::*;
use std::sync::Mutex;

static CACHE: Mutex<Option<Vec<antenna::Model>>> = Mutex::new(None);

fn set(antennas: &[antenna::Model]) {
    let _ = CACHE
        .lock()
        .map(|mut cache| *cache = Some(antennas.to_owned()));
}

pub(super) async fn update() -> Result<Vec<antenna::Model>, DbErr> {
    tracing::debug!("updating cache");
    let antennas = antenna::Entity::find().all(db_conn().await?).await?;
    set(&antennas);
    Ok(antennas)
}

pub(super) async fn get() -> Result<Vec<antenna::Model>, DbErr> {
    if let Some(cache) = CACHE.lock().ok().and_then(|cache| cache.clone()) {
        return Ok(cache);
    }
    update().await
}
