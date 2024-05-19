//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::PageVisibilityEnum;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "page")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "Page", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "updatedAt")]
    pub updated_at: DateTimeWithTimeZone,
    pub title: String,
    pub name: String,
    pub summary: Option<String>,
    #[sea_orm(column_name = "alignCenter")]
    pub align_center: bool,
    pub font: String,
    #[sea_orm(column_name = "userId")]
    pub user_id: String,
    #[sea_orm(column_name = "eyeCatchingImageId")]
    pub eye_catching_image_id: Option<String>,
    #[sea_orm(column_type = "JsonBinary")]
    pub content: Json,
    #[sea_orm(column_type = "JsonBinary")]
    pub variables: Json,
    pub visibility: PageVisibilityEnum,
    #[sea_orm(column_name = "visibleUserIds")]
    pub visible_user_ids: Vec<String>,
    #[sea_orm(column_name = "likedCount")]
    pub liked_count: i32,
    #[sea_orm(column_name = "hideTitleWhenPinned")]
    pub hide_title_when_pinned: bool,
    pub script: String,
    #[sea_orm(column_name = "isPublic")]
    pub is_public: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::drive_file::Entity",
        from = "Column::EyeCatchingImageId",
        to = "super::drive_file::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    DriveFile,
    #[sea_orm(has_many = "super::page_like::Entity")]
    PageLike,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(has_one = "super::user_profile::Entity")]
    UserProfile,
}

impl Related<super::drive_file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DriveFile.def()
    }
}

impl Related<super::page_like::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageLike.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
