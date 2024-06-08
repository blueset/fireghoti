use crate::{
    database::db_conn,
    model::entity::{drive_file, note},
};
use sea_orm::{prelude::*, QuerySelect};

/// Returns [`Vec<String>`] containing the post text, content warning,
/// those of the "parent" (replied/quoted) posts, and alt texts of attached files.
///
/// # Arguments
///
/// * `file_ids` : IDs of attached files ([`drive_file::Model`])
/// * `text`, `cw`, `renote_id`, `reply_id` : note ([`note::Model`]) fields
/// * `include_parent` : whether to take the reply-to post and quoted post into account
pub async fn all_texts(
    file_ids: Vec<String>,
    text: Option<String>,
    cw: Option<String>,
    renote_id: Option<String>,
    reply_id: Option<String>,
    include_parent: bool,
) -> Result<Vec<String>, DbErr> {
    let db = db_conn().await?;

    let mut texts: Vec<String> = vec![];
    let is_renote = text.is_none();

    text.map(|text| texts.push(text));
    cw.map(|cw| texts.push(cw));

    texts.extend(
        drive_file::Entity::find()
            .select_only()
            .column(drive_file::Column::Comment)
            .filter(drive_file::Column::Id.is_in(file_ids))
            .into_tuple::<Option<String>>()
            .all(db)
            .await?
            .into_iter()
            .flatten(),
    );

    if renote_id.is_some() && (include_parent || is_renote) {
        let renote_id = renote_id.unwrap();

        if let Some((text, cw)) = note::Entity::find_by_id(&renote_id)
            .select_only()
            .columns([note::Column::Text, note::Column::Cw])
            .into_tuple::<(Option<String>, Option<String>)>()
            .one(db)
            .await?
        {
            if let Some(t) = text {
                texts.push(t);
            }
            if let Some(c) = cw {
                texts.push(c);
            }
        } else {
            tracing::warn!("nonexistent renote id: {}", renote_id);
        }
    }

    if include_parent && reply_id.is_some() {
        if let Some((text, cw)) = note::Entity::find_by_id(reply_id.as_ref().unwrap())
            .select_only()
            .columns([note::Column::Text, note::Column::Cw])
            .into_tuple::<(Option<String>, Option<String>)>()
            .one(db)
            .await?
        {
            if let Some(t) = text {
                texts.push(t);
            }
            if let Some(c) = cw {
                texts.push(c);
            }
        } else {
            tracing::warn!("nonexistent reply id: {}", reply_id.unwrap());
        }
    }

    Ok(texts)
}
