use crate::database::db_conn;
use crate::model::entity::{drive_file, note};
use sea_orm::{prelude::*, QuerySelect};

// TODO?: handle name collisions
#[crate::export(object, js_name = "NoteLikeForAllTexts")]
pub struct NoteLike {
    pub file_ids: Vec<String>,
    pub user_id: String,
    pub text: Option<String>,
    pub cw: Option<String>,
    pub renote_id: Option<String>,
    pub reply_id: Option<String>,
}

pub async fn all_texts(note: NoteLike, include_in_reply_to: bool) -> Result<Vec<String>, DbErr> {
    let db = db_conn().await?;

    let mut texts: Vec<String> = vec![];

    if let Some(text) = note.text {
        texts.push(text);
    }
    if let Some(cw) = note.cw {
        texts.push(cw);
    }

    texts.extend(
        drive_file::Entity::find()
            .select_only()
            .column(drive_file::Column::Comment)
            .filter(drive_file::Column::Id.is_in(note.file_ids))
            .into_tuple::<Option<String>>()
            .all(db)
            .await?
            .into_iter()
            .flatten(),
    );

    if let Some(renote_id) = &note.renote_id {
        if let Some((text, cw)) = note::Entity::find_by_id(renote_id)
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

    if include_in_reply_to && note.reply_id.is_some() {
        if let Some((text, cw)) = note::Entity::find_by_id(note.reply_id.as_ref().unwrap())
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
            tracing::warn!("nonexistent reply id: {}", note.reply_id.unwrap());
        }
    }

    Ok(texts)
}
