use super::*;
use crate::{config::CONFIG, federation::internal_actor};

#[macros::export(object)]
pub struct ApFlag {
    pub r#type: ApObject,
    pub actor: String,
    pub content: String,
    // TODO: object can be an array of uri's
    pub object: String,
}

impl ActivityPubObject for ApFlag {}

impl ApFlag {
    #[allow(dead_code)] // TODO: remove this line
    async fn new(
        target_user_uri: String,
        comment: String,
    ) -> Result<Self, internal_actor::instance::Error> {
        Ok(Self {
            r#type: ApObject::Flag,
            actor: format!(
                "{}/users/{}",
                CONFIG.url,
                internal_actor::instance::get().await?.id
            ),
            content: comment,
            object: target_user_uri,
        })
    }
}

#[macros::ts_export]
pub async fn render_flag(
    target_user_uri: String,
    comment: String,
) -> Result<ApFlag, internal_actor::instance::Error> {
    ApFlag::new(target_user_uri, comment).await
}
