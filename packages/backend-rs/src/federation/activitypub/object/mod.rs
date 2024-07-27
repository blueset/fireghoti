pub mod relay;

pub trait ActivityPubObject {}

#[derive(serde::Serialize)]
#[macros::export(string_enum)]
pub enum Activity {
    Follow,
}
