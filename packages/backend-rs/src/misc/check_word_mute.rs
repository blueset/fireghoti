use crate::database::db_conn;
use crate::model::entity::{drive_file, note};
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::{prelude::*, QuerySelect};

#[crate::export(object)]
pub struct NoteLike {
    pub file_ids: Vec<String>,
    pub user_id: Option<String>,
    pub text: Option<String>,
    pub cw: Option<String>,
    pub renote_id: Option<String>,
    pub reply_id: Option<String>,
}

async fn all_texts(note: NoteLike) -> Result<Vec<String>, DbErr> {
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

    if let Some(renote_id) = note.renote_id {
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
        }
    }

    if let Some(reply_id) = note.reply_id {
        if let Some((text, cw)) = note::Entity::find_by_id(reply_id)
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
        }
    }

    Ok(texts)
}

fn convert_regex(js_regex: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^/(.+)/(.*)$").unwrap());
    RE.replace(js_regex, "(?$2)$1").to_string()
}

fn check_word_mute_impl(
    texts: &[String],
    muted_word_lists: &[Vec<String>],
    muted_patterns: &[String],
) -> bool {
    muted_word_lists.iter().any(|muted_word_list| {
        texts.iter().any(|text| {
            let text_lower = text.to_lowercase();
            muted_word_list
                .iter()
                .all(|muted_word| text_lower.contains(&muted_word.to_lowercase()))
        })
    }) || muted_patterns.iter().any(|muted_pattern| {
        Regex::new(convert_regex(muted_pattern).as_str())
            .map(|re| texts.iter().any(|text| re.is_match(text)))
            .unwrap_or(false)
    })
}

#[crate::export]
pub async fn check_word_mute(
    note: NoteLike,
    muted_word_lists: Vec<Vec<String>>,
    muted_patterns: Vec<String>,
) -> Result<bool, DbErr> {
    if muted_word_lists.is_empty() && muted_patterns.is_empty() {
        Ok(false)
    } else {
        Ok(check_word_mute_impl(
            &all_texts(note).await?,
            &muted_word_lists,
            &muted_patterns,
        ))
    }
}
