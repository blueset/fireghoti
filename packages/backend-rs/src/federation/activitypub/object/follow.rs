use super::*;
use crate::{config::CONFIG, federation::internal_actor, misc::user};

#[macros::export(object)]
pub struct UserLike {
    pub id: String,
    pub host: Option<String>,
    pub uri: String,
}

#[macros::export(object)]
pub struct Follow {
    pub id: String,
    pub r#type: Activity,
    pub actor: String,
    pub object: String,
}

impl ActivityPubObject for Follow {}

impl Follow {
    #[allow(dead_code)] // TODO: remove this line
    fn new(follower: UserLike, followee: UserLike, request_id: Option<String>) -> Self {
        Self {
            id: request_id.unwrap_or_else(|| {
                format!("{}/follows/{}/{}", CONFIG.url, follower.id, followee.id)
            }),
            r#type: Activity::Follow,
            actor: match user::is_local!(follower) {
                true => format!("{}/users/{}", CONFIG.url, follower.id),
                false => follower.uri,
            },
            object: match user::is_local!(followee) {
                true => format!("{}/users/{}", CONFIG.url, followee.id),
                false => followee.uri,
            },
        }
    }

    #[allow(dead_code)] // TODO: remove this line
    async fn new_relay(relay_id: String) -> Result<Self, internal_actor::relay::Error> {
        Ok(Self {
            id: format!("{}/activities/follow-relay/{}", CONFIG.url, relay_id),
            r#type: Activity::Follow,
            actor: format!(
                "{}/users/{}",
                CONFIG.url,
                internal_actor::relay::get_id().await?
            ),
            object: AS_PUBLIC_URL.to_owned(),
        })
    }
}

#[macros::ts_export]
pub fn render_follow(follower: UserLike, followee: UserLike, request_id: Option<String>) -> Follow {
    Follow::new(follower, followee, request_id)
}

#[macros::ts_export]
pub async fn render_follow_relay(relay_id: String) -> Result<Follow, internal_actor::relay::Error> {
    Follow::new_relay(relay_id).await
}
