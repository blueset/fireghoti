use super::*;
use crate::config::CONFIG;
use uuid::Uuid;

#[macros::export(object)]
pub struct ApAccept {
    pub id: String,
    pub r#type: ApObject,
    pub actor: String,
    pub object: follow::ApFollow,
}

impl ActivityPubObject for ApAccept {}

impl ApAccept {
    #[allow(dead_code)] // TODO: remove this line
    fn new(user_id: String, follow_object: follow::ApFollow) -> Self {
        Self {
            id: format!("{}/{}", CONFIG.url, Uuid::new_v4()),
            r#type: ApObject::Accept,
            actor: format!("{}/users/{}", CONFIG.url, user_id),
            object: follow_object,
        }
    }
}

#[cfg(any(test, doctest, feature = "napi"))]
type Follow = follow::ApFollow;

#[macros::ts_export]
pub fn render_accept(user_id: String, follow_object: Follow) -> ApAccept {
    ApAccept::new(user_id, follow_object)
}
