//! In-memory internal actor cache handler

// TODO: refactoring

use super::*;
use crate::{database::db_conn, model::entity::user};
use sea_orm::prelude::*;
use std::sync::Mutex;

#[macros::errors]
pub enum Error {
    #[error(transparent)]
    #[doc = "database error"]
    Db(#[from] DbErr),
    #[error("{} does not exist", Acct::from(.0.to_owned()))]
    #[doc = "internal actor does not exist"]
    InternalActorNotFound(InternalActor),
}

static INSTANCE_ACTOR: Mutex<Option<user::Model>> = Mutex::new(None);
static RELAY_ACTOR: Mutex<Option<user::Model>> = Mutex::new(None);

fn set_instance_actor(value: &user::Model) {
    let _ = INSTANCE_ACTOR
        .lock()
        .map(|mut cache| *cache = Some(value.to_owned()));
}
fn set_relay_actor(value: &user::Model) {
    let _ = RELAY_ACTOR
        .lock()
        .map(|mut cache| *cache = Some(value.to_owned()));
}

async fn cache_instance_actor() -> Result<user::Model, Error> {
    let actor = user::Entity::find()
        .filter(user::Column::Username.eq(INSTANCE_ACTOR_USERNAME))
        .filter(user::Column::Host.is_null())
        .one(db_conn().await?)
        .await?;

    if let Some(actor) = actor {
        set_instance_actor(&actor);
        Ok(actor)
    } else {
        Err(Error::InternalActorNotFound(InternalActor::Instance))
    }
}
async fn cache_relay_actor() -> Result<user::Model, Error> {
    let actor = user::Entity::find()
        .filter(user::Column::Username.eq(RELAY_ACTOR_USERNAME))
        .filter(user::Column::Host.is_null())
        .one(db_conn().await?)
        .await?;

    if let Some(actor) = actor {
        set_relay_actor(&actor);
        Ok(actor)
    } else {
        Err(Error::InternalActorNotFound(InternalActor::Relay))
    }
}

// for napi export
// https://github.com/napi-rs/napi-rs/issues/2060
type User = user::Model;

#[macros::export(js_name = "getInternalActor")]
pub async fn get(actor: InternalActor) -> Result<User, Error> {
    match actor {
        InternalActor::Instance => {
            if let Some(cache) = INSTANCE_ACTOR.lock().ok().and_then(|cache| cache.clone()) {
                tracing::debug!("Using cached instance.actor");
                return Ok(cache);
            }
            tracing::debug!("Caching instance.actor");
            cache_instance_actor().await
        }
        InternalActor::Relay => {
            if let Some(cache) = RELAY_ACTOR.lock().ok().and_then(|cache| cache.clone()) {
                tracing::debug!("Using cached relay.actor");
                return Ok(cache);
            }
            tracing::debug!("Caching relay.actor");
            cache_relay_actor().await
        }
    }
}
