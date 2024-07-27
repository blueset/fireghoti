use super::*;
use crate::{config::CONFIG, federation::internal_actor};
use serde::Serialize;

#[derive(Serialize)]
#[macros::export(object)]
pub struct FollowRelay {
    pub id: String,
    pub r#type: Activity,
    pub actor: String,
    pub object: String,
}

impl ActivityPubObject for FollowRelay {}

#[macros::export(js_name = "renderFollowRelay")]
pub async fn follow(relay_id: &str) -> Result<FollowRelay, internal_actor::relay::Error> {
    Ok(FollowRelay {
        id: format!("{}/activities/follow-relay/{}", CONFIG.url, relay_id),
        r#type: Activity::Follow,
        actor: format!(
            "{}/users/{}",
            CONFIG.url,
            internal_actor::relay::get_id().await?
        ),
        object: "https://www.w3.org/ns/activitystreams#Public".to_owned(),
    })
}
