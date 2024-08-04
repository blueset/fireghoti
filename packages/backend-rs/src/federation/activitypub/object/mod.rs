pub mod accept;
pub mod emoji;
pub mod flag;
pub mod follow;
pub mod mention;
pub mod read;
pub mod tombstone;

pub trait ActivityPubObject {}

#[macros::export(string_enum)]
pub enum ApObject {
    Accept,
    Emoji,
    Flag,
    Follow,
    Mention,
    Image,
    Read,
    Tombstone,
}

const AS_PUBLIC_URL: &str = "https://www.w3.org/ns/activitystreams#Public";

#[macros::export(object)]
pub struct UserLike {
    pub id: String,
    pub username: String,
    pub host: Option<String>,
    pub uri: Option<String>,
}
