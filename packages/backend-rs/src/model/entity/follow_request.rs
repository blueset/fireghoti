//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "follow_request")]
#[macros::export(object, js_name = "FollowRequest")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "followeeId")]
    pub followee_id: String,
    #[sea_orm(column_name = "followerId")]
    pub follower_id: String,
    #[sea_orm(column_name = "requestId")]
    pub request_id: Option<String>,
    #[sea_orm(column_name = "followerHost")]
    pub follower_host: Option<String>,
    #[sea_orm(column_name = "followerInbox")]
    pub follower_inbox: Option<String>,
    #[sea_orm(column_name = "followerSharedInbox")]
    pub follower_shared_inbox: Option<String>,
    #[sea_orm(column_name = "followeeHost")]
    pub followee_host: Option<String>,
    #[sea_orm(column_name = "followeeInbox")]
    pub followee_inbox: Option<String>,
    #[sea_orm(column_name = "followeeSharedInbox")]
    pub followee_shared_inbox: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::notification::Entity")]
    Notification,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FolloweeId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FollowerId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User1,
}

impl Related<super::notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notification.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
