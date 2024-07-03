//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "user_keypair")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "UserKeypair", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(column_name = "userId", primary_key, auto_increment = false, unique)]
    pub user_id: String,
    #[sea_orm(column_name = "publicKey")]
    pub public_key: String,
    #[sea_orm(column_name = "privateKey")]
    pub private_key: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
