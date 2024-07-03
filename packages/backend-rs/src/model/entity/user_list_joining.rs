//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "user_list_joining")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "UserListJoining", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "userId")]
    pub user_id: String,
    #[sea_orm(column_name = "userListId")]
    pub user_list_id: String,
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
    #[sea_orm(
        belongs_to = "super::user_list::Entity",
        from = "Column::UserListId",
        to = "super::user_list::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    UserList,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::user_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserList.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
