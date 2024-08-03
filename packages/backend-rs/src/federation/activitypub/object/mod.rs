pub mod accept;
pub mod emoji;
pub mod flag;
pub mod follow;
pub mod tombstone;

pub trait ActivityPubObject {}

#[macros::export(string_enum)]
pub enum ApObject {
    Accept,
    Emoji,
    Flag,
    Follow,
    Image,
    Tombstone,
}

const AS_PUBLIC_URL: &str = "https://www.w3.org/ns/activitystreams#Public";
