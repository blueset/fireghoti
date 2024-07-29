pub mod accept;
pub mod follow;

pub trait ActivityPubObject {}

#[macros::export(string_enum)]
pub enum Activity {
    Accept,
    Follow,
}

const AS_PUBLIC_URL: &str = "https://www.w3.org/ns/activitystreams#Public";
