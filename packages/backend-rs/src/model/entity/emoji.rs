//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "emoji")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "Emoji", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "updatedAt")]
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub name: String,
    pub host: Option<String>,
    #[sea_orm(column_name = "originalUrl")]
    pub original_url: String,
    pub uri: Option<String>,
    pub r#type: Option<String>,
    pub aliases: Vec<String>,
    pub category: Option<String>,
    #[sea_orm(column_name = "publicUrl")]
    pub public_url: String,
    pub license: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
