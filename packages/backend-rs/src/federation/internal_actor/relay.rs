//! In-memory relay actor id cache

use super::RELAY_ACTOR_USERNAME;
use crate::{database::db_conn, model::entity::user};
use sea_orm::{prelude::*, QuerySelect, SelectColumns};
use tokio::sync::OnceCell;

static RELAY_ACTOR_ID: OnceCell<String> = OnceCell::const_new();

#[macros::errors]
pub enum Error {
    #[error("@relay.actor not found")]
    RelayActorNotFound,
    #[error(transparent)]
    #[doc = "database error"]
    Db(#[from] DbErr),
}

async fn set_id_cache() -> Result<&'static str, Error> {
    let id = RELAY_ACTOR_ID
        .get_or_try_init(|| async {
            tracing::debug!("caching @relay.actor");
            let found_id = user::Entity::find()
                .select_only()
                .select_column(user::Column::Id)
                .filter(user::Column::Username.eq(RELAY_ACTOR_USERNAME))
                .filter(user::Column::Host.is_null())
                .into_tuple::<String>()
                .one(db_conn().await?)
                .await?;

            Ok::<String, Error>(found_id.ok_or(Error::RelayActorNotFound)?)
        })
        .await?;

    Ok(id)
}

#[macros::export(js_name = "getRelayActorId")]
pub async fn get_id() -> Result<&'static str, Error> {
    match RELAY_ACTOR_ID.get() {
        Some(id) => Ok(id),
        None => set_id_cache().await,
    }
}
