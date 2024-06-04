//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::PushSubscriptionType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "sw_subscription")]
#[cfg_attr(
    feature = "napi",
    napi_derive::napi(object, js_name = "SwSubscription", use_nullable = true)
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "userId")]
    pub user_id: String,
    pub endpoint: String,
    pub auth: String,
    pub publickey: String,
    #[sea_orm(column_name = "sendReadMessage")]
    pub send_read_message: bool,
    #[sea_orm(column_name = "appAccessTokenId")]
    pub app_access_token_id: Option<String>,
    #[sea_orm(column_name = "subscriptionTypes")]
    pub subscription_types: Vec<PushSubscriptionType>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::access_token::Entity",
        from = "Column::AppAccessTokenId",
        to = "super::access_token::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    AccessToken,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::access_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccessToken.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
