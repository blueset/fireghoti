//! In-memory instance actor cache

use crate::{database::db_conn, model::entity::user};
use sea_orm::prelude::*;
use tokio::sync::OnceCell;

// for napi export
// https://github.com/napi-rs/napi-rs/issues/2060
type User = user::Model;

pub const USERNAME: &str = "instance.actor";
static INSTANCE_ACTOR: OnceCell<User> = OnceCell::const_new();

#[macros::errors]
pub enum Error {
    #[error("@instance.actor not found")]
    InstanceActorNotFound,
    #[error(transparent)]
    #[doc = "database error"]
    Db(#[from] DbErr),
}

async fn set_cache() -> Result<&'static User, Error> {
    let instance_actor = INSTANCE_ACTOR
        .get_or_try_init(|| async {
            tracing::debug!("caching @instance.actor");
            let found_model = user::Entity::find()
                .filter(user::Column::Username.eq(USERNAME))
                .filter(user::Column::Host.is_null())
                .one(db_conn().await?)
                .await?;

            Ok::<User, Error>(found_model.ok_or(Error::InstanceActorNotFound)?)
        })
        .await?;

    Ok(instance_actor)
}

pub async fn get() -> Result<&'static User, Error> {
    match INSTANCE_ACTOR.get() {
        Some(model) => Ok(model),
        None => set_cache().await,
    }
}

#[macros::ts_export(js_name = "getInstanceActor")]
pub async fn get_js() -> Result<User, Error> {
    Ok(get().await?.to_owned())
}
