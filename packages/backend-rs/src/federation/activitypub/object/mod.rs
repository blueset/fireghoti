pub mod follow;

pub trait ActivityPubObject {}

#[derive(serde::Serialize)]
#[macros::export(string_enum)]
pub enum Activity {
    Follow,
}

const AS_PUBLIC_URL: &str = "https://www.w3.org/ns/activitystreams#Public";
