use crate::service::stream::{publish_to_stream, Error, Stream};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[crate::export(object)]
pub struct AbuseUserReportLike {
    pub id: String,
    pub target_user_id: String,
    pub reporter_id: String,
    pub comment: String,
}

#[crate::export(js_name = "publishToModerationStream")]
pub fn publish(moderator_id: String, report: &AbuseUserReportLike) -> Result<(), Error> {
    publish_to_stream(
        &Stream::Moderation { moderator_id },
        Some("newAbuseUserReport".to_string()),
        Some(serde_json::to_string(report)?),
    )
}
