use crate::database::db_conn;
use crate::misc::convert_host::to_puny;
use crate::misc::meta::fetch_meta;
use crate::model::entity::emoji;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
#[crate::export(object)]
pub struct DecodedReaction {
    pub reaction: String,
    pub name: Option<String>,
    pub host: Option<String>,
}

#[crate::export]
pub fn decode_reaction(reaction: &str) -> DecodedReaction {
    // Misskey allows you to include "+" and "-" in emoji shortcodes
    // MFM spec: https://github.com/misskey-dev/mfm.js/blob/6aaf68089023c6adebe44123eebbc4dcd75955e0/docs/syntax.md?plain=1#L583
    // Misskey's implementation: https://github.com/misskey-dev/misskey/blob/bba3097765317cbf95d09627961b5b5dce16a972/packages/backend/src/core/ReactionService.ts#L68
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^:([0-9A-Za-z_+-]+)(?:@([0-9A-Za-z_.-]+))?:$").unwrap());

    if let Some(captures) = RE.captures(reaction) {
        let name = &captures[1];
        let host = captures.get(2).map(|s| s.as_str());

        DecodedReaction {
            reaction: format!(":{}@{}:", name, host.unwrap_or(".")),
            name: Some(name.to_owned()),
            host: host.map(|s| s.to_owned()),
        }
    } else {
        DecodedReaction {
            reaction: reaction.to_owned(),
            name: None,
            host: None,
        }
    }
}

#[crate::export]
pub fn count_reactions(reactions: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut res = HashMap::<String, u32>::new();

    for (reaction, count) in reactions.iter() {
        if count > &0 {
            let decoded = decode_reaction(reaction).reaction;
            let total = res.entry(decoded).or_insert(0);
            *total += count;
        }
    }

    res
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Idna error: {0}")]
    IdnaErr(#[from] idna::Errors),
    #[error("Database error: {0}")]
    DbErr(#[from] DbErr),
}

#[crate::export]
pub async fn to_db_reaction(reaction: Option<&str>, host: Option<&str>) -> Result<String, Error> {
    if let Some(reaction) = reaction {
        // FIXME: Is it okay to do this only here?
        // This was introduced in https://firefish.dev/firefish/firefish/-/commit/af730e75b6fc1a57ca680ce83459d7e433b130cf
        if reaction.contains('❤') || reaction.contains("♥️") {
            return Ok("❤️".to_owned());
        }

        // check if the reaction is an Unicode emoji
        if emojis::get(reaction).is_some() {
            return Ok(reaction.to_owned());
        }

        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^:([0-9A-Za-z_+-]+)(?:@\.)?:$").unwrap());

        if let Some(captures) = RE.captures(reaction) {
            let name = &captures[1];
            let db = db_conn().await?;

            if let Some(host) = host {
                // remote emoji
                let ascii_host = to_puny(host)?;

                // TODO: Does SeaORM have the `exists` method?
                if emoji::Entity::find()
                    .filter(emoji::Column::Name.eq(name))
                    .filter(emoji::Column::Host.eq(&ascii_host))
                    .one(db)
                    .await?
                    .is_some()
                {
                    return Ok(format!(":{name}@{ascii_host}:"));
                }

                tracing::info!("nonexistent remote custom emoji: :{name}@{ascii_host}:");
            } else {
                // local emoji
                // TODO: Does SeaORM have the `exists` method?
                if emoji::Entity::find()
                    .filter(emoji::Column::Name.eq(name))
                    .filter(emoji::Column::Host.is_null())
                    .one(db)
                    .await?
                    .is_some()
                {
                    return Ok(format!(":{name}:"));
                }

                tracing::info!("nonexistent local custom emoji: :{name}:");
            }
        };
    };

    Ok(fetch_meta(true).await?.default_reaction)
}

#[cfg(test)]
mod unit_test {
    use super::{decode_reaction, DecodedReaction};
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_decode_reaction() {
        let unicode_emoji_1 = DecodedReaction {
            reaction: "⭐".to_string(),
            name: None,
            host: None,
        };
        let unicode_emoji_2 = DecodedReaction {
            reaction: "🩷".to_string(),
            name: None,
            host: None,
        };

        assert_eq!(decode_reaction("⭐"), unicode_emoji_1);
        assert_eq!(decode_reaction("🩷"), unicode_emoji_2);

        assert_ne!(decode_reaction("⭐"), unicode_emoji_2);
        assert_ne!(decode_reaction("🩷"), unicode_emoji_1);

        let unicode_emoji_3 = DecodedReaction {
            reaction: "🖖🏿".to_string(),
            name: None,
            host: None,
        };
        assert_eq!(decode_reaction("🖖🏿"), unicode_emoji_3);

        let local_emoji = DecodedReaction {
            reaction: ":meow_melt_tears@.:".to_string(),
            name: Some("meow_melt_tears".to_string()),
            host: None,
        };
        assert_eq!(decode_reaction(":meow_melt_tears:"), local_emoji);

        let remote_emoji_1 = DecodedReaction {
            reaction: ":meow_uwu@some-domain.example.org:".to_string(),
            name: Some("meow_uwu".to_string()),
            host: Some("some-domain.example.org".to_string()),
        };
        assert_eq!(
            decode_reaction(":meow_uwu@some-domain.example.org:"),
            remote_emoji_1
        );

        let remote_emoji_2 = DecodedReaction {
            reaction: ":C++23@xn--eckwd4c7c.example.org:".to_string(),
            name: Some("C++23".to_string()),
            host: Some("xn--eckwd4c7c.example.org".to_string()),
        };
        assert_eq!(
            decode_reaction(":C++23@xn--eckwd4c7c.example.org:"),
            remote_emoji_2
        );

        let invalid_reaction_1 = DecodedReaction {
            reaction: ":foo".to_string(),
            name: None,
            host: None,
        };
        assert_eq!(decode_reaction(":foo"), invalid_reaction_1);

        let invalid_reaction_2 = DecodedReaction {
            reaction: ":foo&@example.com:".to_string(),
            name: None,
            host: None,
        };
        assert_eq!(decode_reaction(":foo&@example.com:"), invalid_reaction_2);
    }
}
