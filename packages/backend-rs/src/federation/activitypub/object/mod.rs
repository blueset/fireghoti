pub mod accept;
pub mod emoji;
pub mod follow;

pub trait ActivityPubObject {}

#[macros::export(string_enum)]
pub enum ApObject {
    Accept,
    Emoji,
    Follow,
    Image,
}

const AS_PUBLIC_URL: &str = "https://www.w3.org/ns/activitystreams#Public";
