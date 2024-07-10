use crate::model::entity::user;
use sea_orm::prelude::*;

// TODO: When `std::mem::variant_count` is stabilized, use
// it to count system actors instead of hard coding the magic number

// @instance.actor and @relay.actor are not real users
const NUMBER_OF_SYSTEM_ACTORS: u64 = 2;

pub async fn local_total(db: &DbConn) -> Result<u64, DbErr> {
    user::Entity::find()
        .filter(user::Column::Host.is_null())
        .count(db)
        .await
        .map(|count| count - NUMBER_OF_SYSTEM_ACTORS)
}

#[macros::ts_export(js_name = "countLocalUsers")]
pub async fn local_total_js() -> Result<u32, DbErr> {
    local_total(crate::database::db_conn().await?)
        .await
        .map(|count| count as u32)
}
