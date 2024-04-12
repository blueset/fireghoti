//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "abuse_user_report")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "AbuseUserReport", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTime,
    #[sea_orm(column_name = "targetUserId")]
    pub target_user_id: String,
    #[sea_orm(column_name = "reporterId")]
    pub reporter_id: String,
    #[sea_orm(column_name = "assigneeId")]
    pub assignee_id: Option<String>,
    pub resolved: bool,
    pub comment: String,
    #[sea_orm(column_name = "targetUserHost")]
    pub target_user_host: Option<String>,
    #[sea_orm(column_name = "reporterHost")]
    pub reporter_host: Option<String>,
    pub forwarded: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::ReporterId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User3,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AssigneeId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::TargetUserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User1,
}

impl ActiveModelBehavior for ActiveModel {}
