use crate::config::CONFIG;
use crate::database::cache;
use crate::database::db_conn;
use crate::misc::meta::fetch_meta;
use crate::model::entity::{note, user};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// TODO: I want to use these macros but they don't work with rmp_serde
// - #[serde(skip_serializing_if = "Option::is_none")] (https://github.com/3Hren/msgpack-rust/issues/86)
// - #[serde(tag = "version", rename = "2.1")] (https://github.com/3Hren/msgpack-rust/issues/318)

/// NodeInfo schema version 2.1. https://nodeinfo.diaspora.software/docson/index.html#/ns/schema/2.1
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Nodeinfo21 {
    /// The schema version, must be 2.1.
    pub version: String,
    /// Metadata about server software in use.
    pub software: Software21,
    /// The protocols supported on this server.
    pub protocols: Vec<Protocol>,
    /// The third party sites this server can connect to via their application API.
    pub services: Services,
    /// Whether this server allows open self-registration.
    pub open_registrations: bool,
    /// Usage statistics for this server.
    pub usage: Usage,
    /// Free form key value pairs for software specific values. Clients should not rely on any specific key present.
    pub metadata: HashMap<String, serde_json::Value>,
}

/// NodeInfo schema version 2.0. https://nodeinfo.diaspora.software/docson/index.html#/ns/schema/2.0
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Nodeinfo20 {
    /// The schema version, must be 2.0.
    pub version: String,
    /// Metadata about server software in use.
    pub software: Software20,
    /// The protocols supported on this server.
    pub protocols: Vec<Protocol>,
    /// The third party sites this server can connect to via their application API.
    pub services: Services,
    /// Whether this server allows open self-registration.
    pub open_registrations: bool,
    /// Usage statistics for this server.
    pub usage: Usage,
    /// Free form key value pairs for software specific values. Clients should not rely on any specific key present.
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Metadata about server software in use (version 2.1).
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Software21 {
    /// The canonical name of this server software.
    pub name: String,
    /// The version of this server software.
    pub version: String,
    /// The url of the source code repository of this server software.
    pub repository: Option<String>,
    /// The url of the homepage of this server software.
    pub homepage: Option<String>,
}

/// Metadata about server software in use (version 2.0).
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Software20 {
    /// The canonical name of this server software.
    pub name: String,
    /// The version of this server software.
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Activitypub,
    Buddycloud,
    Dfrn,
    Diaspora,
    Libertree,
    Ostatus,
    Pumpio,
    Tent,
    Xmpp,
    Zot,
}

/// The third party sites this server can connect to via their application API.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    /// The third party sites this server can retrieve messages from for combined display with regular traffic.
    pub inbound: Vec<Inbound>,
    /// The third party sites this server can publish messages to on the behalf of a user.
    pub outbound: Vec<Outbound>,
}

/// The third party sites this server can retrieve messages from for combined display with regular traffic.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Inbound {
    #[serde(rename = "atom1.0")]
    Atom1,
    Gnusocial,
    Imap,
    Pnut,
    #[serde(rename = "pop3")]
    Pop3,
    Pumpio,
    #[serde(rename = "rss2.0")]
    Rss2,
    Twitter,
}

/// The third party sites this server can publish messages to on the behalf of a user.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Outbound {
    #[serde(rename = "atom1.0")]
    Atom1,
    Blogger,
    Buddycloud,
    Diaspora,
    Dreamwidth,
    Drupal,
    Facebook,
    Friendica,
    Gnusocial,
    Google,
    Insanejournal,
    Libertree,
    Linkedin,
    Livejournal,
    Mediagoblin,
    Myspace,
    Pinterest,
    Pnut,
    Posterous,
    Pumpio,
    Redmatrix,
    #[serde(rename = "rss2.0")]
    Rss2,
    Smtp,
    Tent,
    Tumblr,
    Twitter,
    Wordpress,
    Xmpp,
}

/// Usage statistics for this server.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub users: Users,
    pub local_posts: Option<u64>,
    pub local_comments: Option<u64>,
}

/// statistics about the users of this server.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    pub total: Option<u64>,
    pub active_halfyear: Option<u64>,
    pub active_month: Option<u64>,
}

impl From<Software21> for Software20 {
    fn from(software: Software21) -> Self {
        Self {
            name: software.name,
            version: software.version,
        }
    }
}

impl From<Nodeinfo21> for Nodeinfo20 {
    fn from(nodeinfo: Nodeinfo21) -> Self {
        Self {
            version: "2.0".to_string(),
            software: nodeinfo.software.into(),
            protocols: nodeinfo.protocols,
            services: nodeinfo.services,
            open_registrations: nodeinfo.open_registrations,
            usage: nodeinfo.usage,
            metadata: nodeinfo.metadata,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DbErr(#[from] DbErr),
    #[error("Cache error: {0}")]
    CacheErr(#[from] cache::Error),
    #[error("Failed to serialize nodeinfo to JSON: {0}")]
    JsonErr(#[from] serde_json::Error),
}

async fn statistics() -> Result<(u64, u64, u64, u64), DbErr> {
    let db = db_conn().await?;

    let now = chrono::Local::now().naive_local();
    const MONTH: chrono::TimeDelta = chrono::Duration::seconds(2592000000);
    const HALF_YEAR: chrono::TimeDelta = chrono::Duration::seconds(15552000000);

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

async fn get_new_nodeinfo_2_1() -> Result<Nodeinfo21, Error> {
    let (local_users, local_active_halfyear, local_active_month, local_posts) =
        statistics().await?;
    let meta = fetch_meta(true).await?;
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
        ("maintainerName".to_string(), json!(meta.maintainer_name)),
        ("maintainerEmail".to_string(), json!(meta.maintainer_email)),
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
                total: Some(local_users),
                active_halfyear: Some(local_active_halfyear),
                active_month: Some(local_active_month),
            },
            local_posts: Some(local_posts),
            local_comments: None,
        },
        metadata,
    })
}

pub async fn nodeinfo_2_1() -> Result<Nodeinfo21, Error> {
    const NODEINFO_2_1_CACHE_KEY: &str = "nodeinfo_2_1";

    let cached = cache::get::<Nodeinfo21>(NODEINFO_2_1_CACHE_KEY)?;

    if let Some(nodeinfo) = cached {
        Ok(nodeinfo)
    } else {
        let nodeinfo = get_new_nodeinfo_2_1().await?;
        cache::set(NODEINFO_2_1_CACHE_KEY, &nodeinfo, 60 * 60)?;
        Ok(nodeinfo)
    }
}

pub async fn nodeinfo_2_0() -> Result<Nodeinfo20, Error> {
    Ok(nodeinfo_2_1().await?.into())
}

#[crate::export(js_name = "nodeinfo_2_1")]
pub async fn nodeinfo_2_1_as_json() -> Result<serde_json::Value, Error> {
    Ok(serde_json::to_value(nodeinfo_2_1().await?)?)
}

#[crate::export(js_name = "nodeinfo_2_0")]
pub async fn nodeinfo_2_0_as_json() -> Result<serde_json::Value, Error> {
    Ok(serde_json::to_value(nodeinfo_2_0().await?)?)
}
