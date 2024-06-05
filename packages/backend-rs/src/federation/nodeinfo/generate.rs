//! NodeInfo generator

use crate::config::CONFIG;
use crate::database::{cache, db_conn};
use crate::federation::nodeinfo::schema::*;
use crate::misc::meta::fetch_meta;
use crate::model::entity::{note, user};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use serde_json::json;
use std::collections::HashMap;

/// Errors that can occur while generating NodeInfo of the local server
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Db(#[from] DbErr),
    #[error("Cache error: {0}")]
    Cache(#[from] cache::Error),
    #[error("Failed to serialize nodeinfo to JSON: {0}")]
    Json(#[from] serde_json::Error),
}

/// Fetches the number of total/active local users and local posts.
///
/// # Return value
/// A tuple containing the following information in this order:
/// * the total number of local users
/// * the total number of local users active in the last 6 months
/// * the total number of local users active in the last month (MAU)
/// * the total number of posts from local users
async fn statistics() -> Result<(u64, u64, u64, u64), DbErr> {
    let db = db_conn().await?;

    let now = chrono::Utc::now();
    const MONTH: chrono::TimeDelta = chrono::Duration::days(30);
    const HALF_YEAR: chrono::TimeDelta = chrono::Duration::days(183);

    let local_users = user::Entity::find()
        .filter(user::Column::Host.is_null())
        .count(db);
    let local_active_halfyear = user::Entity::find()
        .filter(user::Column::Host.is_null())
        .filter(user::Column::LastActiveDate.gt(now - HALF_YEAR))
        .count(db);
    let local_active_month = user::Entity::find()
        .filter(user::Column::Host.is_null())
        .filter(user::Column::LastActiveDate.gt(now - MONTH))
        .count(db);
    let local_posts = note::Entity::find()
        .filter(note::Column::UserHost.is_null())
        .count(db);

    tokio::try_join!(
        local_users,
        local_active_halfyear,
        local_active_month,
        local_posts
    )
}

/// Generates NodeInfo (version 2.1) of the local server.
/// This function doesn't use caches and returns the latest information.
async fn generate_nodeinfo_2_1() -> Result<Nodeinfo21, Error> {
    let (local_users, local_active_halfyear, local_active_month, local_posts) =
        statistics().await?;
    let meta = fetch_meta().await?;
    let metadata = HashMap::from([
        (
            "nodeName".to_string(),
            json!(meta.name.unwrap_or(CONFIG.host.clone())),
        ),
        ("nodeDescription".to_string(), json!(meta.description)),
        ("repositoryUrl".to_string(), json!(meta.repository_url)),
        (
            "enableLocalTimeline".to_string(),
            json!(!meta.disable_local_timeline),
        ),
        (
            "enableRecommendedTimeline".to_string(),
            json!(!meta.disable_recommended_timeline),
        ),
        (
            "enableGlobalTimeline".to_string(),
            json!(!meta.disable_global_timeline),
        ),
        (
            "enableGuestTimeline".to_string(),
            json!(meta.enable_guest_timeline),
        ),
        (
            "maintainer".to_string(),
            json!({"name":meta.maintainer_name,"email":meta.maintainer_email}),
        ),
        ("proxyAccountName".to_string(), json!(meta.proxy_account_id)),
        (
            "themeColor".to_string(),
            json!(meta.theme_color.unwrap_or("#31748f".to_string())),
        ),
    ]);

    Ok(Nodeinfo21 {
        version: "2.1".to_string(),
        software: Software21 {
            name: "firefish".to_string(),
            version: CONFIG.version.clone(),
            repository: Some(meta.repository_url),
            homepage: Some("https://firefish.dev/firefish/firefish".to_string()),
        },
        protocols: vec![Protocol::Activitypub],
        services: Services {
            inbound: vec![],
            outbound: vec![Outbound::Atom1, Outbound::Rss2],
        },
        open_registrations: !meta.disable_registration,
        usage: Usage {
            users: Users {
                total: Some(local_users as u32),
                active_halfyear: Some(local_active_halfyear as u32),
                active_month: Some(local_active_month as u32),
            },
            local_posts: Some(local_posts as u32),
            local_comments: None,
        },
        metadata,
    })
}

/// Returns NodeInfo (version 2.1) of the local server.
pub async fn nodeinfo_2_1() -> Result<Nodeinfo21, Error> {
    const NODEINFO_2_1_CACHE_KEY: &str = "nodeinfo_2_1";

    let cached = cache::get::<Nodeinfo21>(NODEINFO_2_1_CACHE_KEY).await?;

    if let Some(nodeinfo) = cached {
        Ok(nodeinfo)
    } else {
        let nodeinfo = generate_nodeinfo_2_1().await?;
        cache::set(NODEINFO_2_1_CACHE_KEY, &nodeinfo, 60 * 60).await?;
        Ok(nodeinfo)
    }
}

/// Returns NodeInfo (version 2.0) of the local server.
pub async fn nodeinfo_2_0() -> Result<Nodeinfo20, Error> {
    Ok(nodeinfo_2_1().await?.into())
}

#[crate::ts_export(js_name = "nodeinfo_2_1")]
pub async fn nodeinfo_2_1_as_json() -> Result<serde_json::Value, Error> {
    Ok(serde_json::to_value(nodeinfo_2_1().await?)?)
}

#[crate::ts_export(js_name = "nodeinfo_2_0")]
pub async fn nodeinfo_2_0_as_json() -> Result<serde_json::Value, Error> {
    Ok(serde_json::to_value(nodeinfo_2_0().await?)?)
}
