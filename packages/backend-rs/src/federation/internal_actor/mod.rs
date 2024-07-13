mod cache;

pub use cache::get;

use super::acct::Acct;

#[derive(Debug)]
#[macros::derive_clone_and_export(string_enum = "lowercase")]
pub enum InternalActor {
    Instance,
    Relay,
}

const INSTANCE_ACTOR_USERNAME: &str = "instance.actor";
const RELAY_ACTOR_USERNAME: &str = "relay.actor";

// TODO: When `std::mem::variant_count` is stabilized, use
// it to count system actors instead of hard coding the magic number
pub const INTERNAL_ACTORS: u64 = 2;

impl From<InternalActor> for Acct {
    fn from(actor: InternalActor) -> Self {
        match actor {
            InternalActor::Instance => Acct {
                username: INSTANCE_ACTOR_USERNAME.to_owned(),
                host: None,
            },
            InternalActor::Relay => Acct {
                username: RELAY_ACTOR_USERNAME.to_owned(),
                host: None,
            },
        }
    }
}
