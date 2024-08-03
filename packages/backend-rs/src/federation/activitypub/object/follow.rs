use super::*;
use crate::{config::CONFIG, federation::internal_actor, misc::user};

#[macros::export(object)]
pub struct UserLike {
    pub id: String,
    pub host: Option<String>,
    pub uri: Option<String>,
}

#[macros::export(object)]
pub struct ApFollow {
    pub id: String,
    pub r#type: ApObject,
    pub actor: String,
    pub object: String,
}

impl ActivityPubObject for ApFollow {}

#[macros::errors]
pub enum Error {
    #[error("follower uri is missing")]
    MissingFollowerUri,
    #[error("followee uri is missing")]
    MissingFolloweeUri,
}

impl ApFollow {
    #[allow(dead_code)] // TODO: remove this line
    fn new(
        follower: UserLike,
        followee: UserLike,
        request_id: Option<String>,
    ) -> Result<Self, Error> {
        Ok(Self {
            id: request_id.unwrap_or_else(|| {
                format!("{}/follows/{}/{}", CONFIG.url, follower.id, followee.id)
            }),
            r#type: ApObject::Follow,
            actor: match user::is_local!(follower) {
                true => user::local_uri(follower.id),
                false => follower.uri.ok_or(Error::MissingFollowerUri)?,
            },
            object: match user::is_local!(followee) {
                true => user::local_uri(followee.id),
                false => followee.uri.ok_or(Error::MissingFolloweeUri)?,
            },
        })
    }

    #[allow(dead_code)] // TODO: remove this line
    async fn new_relay(relay_id: String) -> Result<Self, internal_actor::relay::Error> {
        Ok(Self {
            id: format!("{}/activities/follow-relay/{}", CONFIG.url, relay_id),
            r#type: ApObject::Follow,
            actor: user::local_uri(internal_actor::relay::get_id().await?),
            object: AS_PUBLIC_URL.to_owned(),
        })
    }
}

#[macros::ts_export]
pub fn render_follow(
    follower: UserLike,
    followee: UserLike,
    request_id: Option<String>,
) -> Result<ApFollow, Error> {
    ApFollow::new(follower, followee, request_id)
}

#[macros::ts_export]
pub async fn render_follow_relay(
    relay_id: String,
) -> Result<ApFollow, internal_actor::relay::Error> {
    ApFollow::new_relay(relay_id).await
}
