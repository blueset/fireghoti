//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "note_edit")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "NoteEdit", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "noteId")]
    pub note_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub text: Option<String>,
    pub cw: Option<String>,
    #[sea_orm(column_name = "fileIds")]
    pub file_ids: Vec<String>,
    #[sea_orm(column_name = "updatedAt")]
    pub updated_at: DateTimeWithTimeZone,
    pub emojis: Vec<String>,
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
