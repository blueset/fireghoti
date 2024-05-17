use crate::config::CONFIG;
use crate::database::{cache, db_conn};
use crate::federation::acct::Acct;
use crate::misc::get_note_all_texts::{all_texts, NoteLike};
use crate::model::entity::{antenna, blocking, following, note, sea_orm_active_enums::*};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect};

#[derive(thiserror::Error, Debug)]
pub enum AntennaCheckError {
    #[error("Database error: {0}")]
    DbErr(#[from] DbErr),
    #[error("Cache error: {0}")]
    CacheErr(#[from] cache::Error),
    #[error("User profile not found: {0}")]
    UserProfileNotFoundErr(String),
}

fn match_all(space_separated_words: &str, text: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        space_separated_words
            .split_whitespace()
            .all(|word| text.contains(word))
    } else {
        space_separated_words
            .to_lowercase()
            .split_whitespace()
            .all(|word| text.to_lowercase().contains(word))
    }
}

pub async fn check_hit_antenna(
    antenna: &antenna::Model,
    note: note::Model,
    note_author: &Acct,
) -> Result<bool, AntennaCheckError> {
    if note.visibility == NoteVisibilityEnum::Specified {
        return Ok(false);
    }

    if antenna.with_file && note.file_ids.is_empty() {
        return Ok(false);
    }

    if !antenna.with_replies && note.reply_id.is_some() {
        return Ok(false);
    }

    if antenna.src == AntennaSrcEnum::Users {
        let is_from_one_of_specified_authors = antenna
            .users
            .iter()
            .map(|s| s.parse::<Acct>().unwrap())
            .any(|acct| acct.username == note_author.username && acct.host == note_author.host);

        if !is_from_one_of_specified_authors {
            return Ok(false);
        }
    } else if antenna.src == AntennaSrcEnum::Instances {
        let is_from_one_of_specified_servers = antenna.instances.iter().any(|host| {
            host.to_ascii_lowercase()
                == note_author
                    .host
                    .clone()
                    .unwrap_or(CONFIG.host.clone())
                    .to_ascii_lowercase()
        });

        if !is_from_one_of_specified_servers {
            return Ok(false);
        }
    }

    // "Home", "Group", "List" sources are currently disabled

    let note_texts = all_texts(NoteLike {
        file_ids: note.file_ids,
        user_id: note.user_id.clone(),
        text: note.text,
        cw: note.cw,
        renote_id: note.renote_id,
        reply_id: note.reply_id,
    })
    .await?;

    let has_keyword = antenna.keywords.iter().any(|words| {
        note_texts
            .iter()
            .any(|text| match_all(words, text, antenna.case_sensitive))
    });

    if !has_keyword {
        return Ok(false);
    }

    let has_excluded_word = antenna.exclude_keywords.iter().any(|words| {
        note_texts
            .iter()
            .any(|text| match_all(words, text, antenna.case_sensitive))
    });

    if has_excluded_word {
        return Ok(false);
    }

    let db = db_conn().await?;

    let blocked_user_ids: Vec<String> = cache::get_one(cache::Category::Block, &note.user_id)?
        .unwrap_or({
            // cache miss
            let blocks = blocking::Entity::find()
                .select_only()
                .column(blocking::Column::BlockeeId)
                .filter(blocking::Column::BlockerId.eq(&note.user_id))
                .into_tuple::<String>()
                .all(db)
                .await?;
            cache::set_one(cache::Category::Block, &note.user_id, &blocks, 10 * 60)?;
            blocks
        });

    // if the antenna owner is blocked by the note author, return false
    if blocked_user_ids.contains(&antenna.user_id) {
        return Ok(false);
    }

    if [NoteVisibilityEnum::Home, NoteVisibilityEnum::Followers].contains(&note.visibility) {
        let following_user_ids: Vec<String> =
            cache::get_one(cache::Category::Follow, &antenna.user_id)?.unwrap_or({
                // cache miss
                let following = following::Entity::find()
                    .select_only()
                    .column(following::Column::FolloweeId)
                    .filter(following::Column::FollowerId.eq(&antenna.user_id))
                    .into_tuple::<String>()
                    .all(db)
                    .await?;
                cache::set_one(
                    cache::Category::Follow,
                    &antenna.user_id,
                    &following,
                    10 * 60,
                )?;
                following
            });

        // if the antenna owner is not following the note author, return false
        if !following_user_ids.contains(&note.user_id) {
            return Ok(false);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod unit_test {
    use super::match_all;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_match_all() {
        assert_eq!(match_all("Apple", "apple and banana", false), true);
        assert_eq!(match_all("Apple", "apple and banana", true), false);
        assert_eq!(match_all("Apple Banana", "apple and banana", false), true);
        assert_eq!(match_all("Apple Banana", "apple and cinnamon", true), false);
        assert_eq!(
            match_all("Apple Banana", "apple and cinnamon", false),
            false
        );
    }
}
