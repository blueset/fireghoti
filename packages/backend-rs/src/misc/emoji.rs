#[inline]
#[crate::export]
pub fn is_unicode_emoji(s: &str) -> bool {
    emojis::get(s).is_some()
}

#[cfg(test)]
mod unit_test {
    use super::is_unicode_emoji;

    #[test]
    fn test_unicode_emoji_check() {
        assert!(is_unicode_emoji("⭐"));
        assert!(is_unicode_emoji("👍"));
        assert!(is_unicode_emoji("❤"));
        assert!(is_unicode_emoji("♥️"));
        assert!(is_unicode_emoji("❤️"));
        assert!(is_unicode_emoji("💙"));
        assert!(is_unicode_emoji("🩷"));
        assert!(is_unicode_emoji("🖖🏿"));
        assert!(is_unicode_emoji("🏃‍➡️"));
        assert!(is_unicode_emoji("👩‍❤️‍👨"));
        assert!(is_unicode_emoji("👩‍👦‍👦"));
        assert!(is_unicode_emoji("🏳️‍🌈"));

        assert!(!is_unicode_emoji("⭐⭐"));
        assert!(!is_unicode_emoji("x"));
        assert!(!is_unicode_emoji("\t"));
        assert!(!is_unicode_emoji(":meow_aww:"));
    }
}
