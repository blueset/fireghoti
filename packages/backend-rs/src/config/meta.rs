//! Server information

use crate::{database::db_conn, model::entity::meta};
use sea_orm::{prelude::*, ActiveValue};
use std::sync::Mutex;

type Meta = meta::Model;

static CACHE: Mutex<Option<Meta>> = Mutex::new(None);
fn set_cache(meta: &Meta) {
    let _ = CACHE.lock().map(|mut cache| *cache = Some(meta.clone()));
}

#[macros::export(js_name = "fetchMeta")]
pub async fn local_server_info() -> Result<Meta, DbErr> {
    local_server_info_impl(true).await
}

#[macros::export(js_name = "updateMetaCache")]
pub async fn update() -> Result<(), DbErr> {
    local_server_info_impl(false).await?;
    Ok(())
}

async fn local_server_info_impl(use_cache: bool) -> Result<Meta, DbErr> {
    // try using cache
    if use_cache {
        if let Some(cache) = CACHE.lock().ok().and_then(|cache| cache.clone()) {
            return Ok(cache);
        }
    }

    // try fetching from db
    let db = db_conn().await?;
    let meta = meta::Entity::find().one(db).await?;
    if let Some(meta) = meta {
        set_cache(&meta);
        return Ok(meta);
    }

    // create a new meta object and insert into db
    let meta = meta::Entity::insert(meta::ActiveModel {
        id: ActiveValue::Set("x".to_owned()),
        ..Default::default()
    })
    .exec_with_returning(db)
    .await?;
    set_cache(&meta);
    Ok(meta)
}

#[macros::export(object)]
pub struct PugArgs {
    pub img: Option<String>,
    pub title: String,
    pub instance_name: String,
    pub desc: Option<String>,
    pub icon: Option<String>,
    pub splash_icon: Option<String>,
    pub theme_color: Option<String>,
    pub random_motd: String,
    pub private_mode: Option<bool>,
}

#[macros::ts_export]
pub fn meta_to_pug_args(meta: Meta) -> PugArgs {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let splash_icon = meta
        .custom_splash_icons
        .choose(&mut rng)
        .map(|s| s.to_owned())
        .or_else(|| meta.icon_url.to_owned());

    let random_motd = meta
        .custom_motd
        .choose(&mut rng)
        .map(|s| s.to_owned())
        .unwrap_or_else(|| "Loading...".to_owned());

    let name = meta.name.unwrap_or_else(|| "Firefish".to_owned());
    PugArgs {
        img: meta.banner_url,
        title: name.clone(),
        instance_name: name.clone(),
        desc: meta.description,
        icon: meta.icon_url,
        splash_icon,
        theme_color: meta.theme_color,
        random_motd,
        private_mode: meta.private_mode,
    }
}
