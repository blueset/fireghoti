//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize)]
#[sea_orm(table_name = "promo_note")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "PromoNote", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(column_name = "noteId", primary_key, auto_increment = false, unique)]
    pub note_id: String,
    #[sea_orm(column_name = "expiresAt")]
    pub expires_at: DateTime,
    #[sea_orm(column_name = "userId")]
    pub user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::note::Entity",
        from = "Column::NoteId",
        to = "super::note::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Note,
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
