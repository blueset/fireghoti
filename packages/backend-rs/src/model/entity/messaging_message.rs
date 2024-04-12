//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "messaging_message")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "MessagingMessage", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "userId")]
    pub user_id: String,
    #[sea_orm(column_name = "recipientId")]
    pub recipient_id: Option<String>,
    pub text: Option<String>,
    #[sea_orm(column_name = "isRead")]
    pub is_read: bool,
    #[sea_orm(column_name = "fileId")]
    pub file_id: Option<String>,
    #[sea_orm(column_name = "groupId")]
    pub group_id: Option<String>,
    pub reads: Vec<String>,
    pub uri: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::drive_file::Entity",
        from = "Column::FileId",
        to = "super::drive_file::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    DriveFile,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::RecipientId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User1,
    #[sea_orm(
        belongs_to = "super::user_group::Entity",
        from = "Column::GroupId",
        to = "super::user_group::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    UserGroup,
}

impl Related<super::drive_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DriveFile.def()
    }
}

impl Related<super::user_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
