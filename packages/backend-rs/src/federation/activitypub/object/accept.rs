use super::*;
use crate::config::CONFIG;
use uuid::Uuid;

#[macros::export(object)]
pub struct Accept {
    pub id: String,
    pub r#type: Activity,
    pub actor: String,
    pub object: follow::Follow,
}

impl ActivityPubObject for Accept {}

impl Accept {
    #[allow(dead_code)] // TODO: remove this line
    fn new(user_id: String, follow_object: follow::Follow) -> Self {
        Self {
            id: format!("{}/{}", CONFIG.url, Uuid::new_v4()),
            r#type: Activity::Accept,
            actor: format!("{}/users/{}", CONFIG.url, user_id),
            object: follow_object,
        }
    }
}

#[cfg(any(test, doctest, feature = "napi"))]
type Follow = follow::Follow;

#[macros::ts_export]
pub fn render_accept(user_id: String, follow_object: Follow) -> Accept {
    Accept::new(user_id, follow_object)
}
