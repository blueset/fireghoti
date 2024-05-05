use serde::{Deserialize, Serialize};
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
