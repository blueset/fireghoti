//! In-memory antennas cache handler

use crate::{database::db_conn, model::entity::antenna};
use sea_orm::prelude::*;
use std::sync::{Arc, Mutex};

static CACHE: Mutex<Option<Arc<[antenna::Model]>>> = Mutex::new(None);

fn set(antennas: Arc<[antenna::Model]>) {
    let _ = CACHE.lock().map(|mut cache| *cache = Some(antennas));
}

pub(super) async fn update() -> Result<Arc<[antenna::Model]>, DbErr> {
    tracing::debug!("updating cache");
    let antennas: Arc<[antenna::Model]> =
        antenna::Entity::find().all(db_conn().await?).await?.into();
    set(antennas.clone());
    Ok(antennas)
}

pub(super) async fn get() -> Result<Arc<[antenna::Model]>, DbErr> {
    if let Some(cache) = CACHE.lock().ok().and_then(|cache| cache.clone()) {
        return Ok(cache);
    }
    update().await
}
