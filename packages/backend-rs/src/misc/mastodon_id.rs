#[crate::export]
pub fn to_mastodon_id(firefish_id: &str) -> Option<String> {
    let decoded: [u8; 16] = basen::BASE36.decode_var_len(&firefish_id.to_ascii_lowercase())?;
    Some(basen::BASE10.encode_var_len(&decoded))
}

#[crate::export]
pub fn from_mastodon_id(mastodon_id: &str) -> Option<String> {
    let decoded: [u8; 16] = basen::BASE10.decode_var_len(mastodon_id)?;
    Some(basen::BASE36.encode_var_len(&decoded))
}

#[cfg(test)]
mod unit_test {
    use super::{from_mastodon_id, to_mastodon_id};
    use pretty_assertions::assert_eq;

    #[test]
    fn to_mastodon_id_test() {
        assert_eq!(
            to_mastodon_id("9pdqi3rjl4lxirq3").unwrap(),
            "2145531976185871567229403"
        );
        assert_eq!(to_mastodon_id("9pdqi3r*irq3"), None);
    }

    #[test]
    fn from_mastodon_id_test() {
        assert_eq!(
            from_mastodon_id("2145531976185871567229403").unwrap(),
            "9pdqi3rjl4lxirq3"
        );
        assert_eq!(from_mastodon_id("9pdqi3rjl4lxirq3"), None);
    }
}
