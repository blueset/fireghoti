use crate::misc::get_note_all_texts::{all_texts, NoteLike};
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::DbErr;

fn convert_regex(js_regex: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^/(.+)/(.*)$").unwrap());
    RE.replace(js_regex, "(?$2)$1").to_string()
}

pub fn check_word_mute_bare(
    texts: &[String],
    muted_words: &[String],
    muted_patterns: &[String],
) -> bool {
    muted_words.iter().any(|item| {
        texts.iter().any(|text| {
            let text_lower = text.to_lowercase();
            item.split_whitespace()
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
    muted_words: &[String],
    muted_patterns: &[String],
) -> Result<bool, DbErr> {
    if muted_words.is_empty() && muted_patterns.is_empty() {
        Ok(false)
    } else {
        Ok(check_word_mute_bare(
            &all_texts(note).await?,
            muted_words,
            muted_patterns,
        ))
    }
}
