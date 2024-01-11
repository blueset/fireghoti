//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "renote_muting")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "muteeId")]
    pub mutee_id: String,
    #[sea_orm(column_name = "muterId")]
    pub muter_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
