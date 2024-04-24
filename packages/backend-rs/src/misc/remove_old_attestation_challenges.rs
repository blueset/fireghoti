// TODO: We want to get rid of this

use crate::database::db_conn;
use crate::model::entity::attestation_challenge;
use chrono::{Duration, Local};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

/// Delete all entries in the "attestation_challenge" table created at more than 5 minutes ago
#[crate::export]
pub async fn remove_old_attestation_challenges() -> Result<(), DbErr> {
    attestation_challenge::Entity::delete_many()
        .filter(attestation_challenge::Column::CreatedAt.lt(Local::now() - Duration::minutes(5)))
        .exec(db_conn().await?)
        .await?;

    Ok(())
}
