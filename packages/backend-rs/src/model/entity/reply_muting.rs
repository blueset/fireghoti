//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "reply_muting")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "ReplyMuting", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "muteeId")]
    pub mutee_id: String,
    #[sea_orm(column_name = "muterId")]
    pub muter_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
