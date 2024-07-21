use mfm::{node, parse, MfmParseError};

#[macros::export]
pub fn extract_hashtags(text: &str) -> Result<Vec<String>, MfmParseError> {
    Ok(parse(text)?
        .into_iter()
        .filter_map(|node| match node {
            node::Node::Inline(node::Inline::Hashtag(node::Hashtag { hashtag })) => Some(hashtag),
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect())
}
