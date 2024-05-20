#[crate::ts_only_warn("Use `emojis::get(str).is_some()` instead.")]
#[crate::export]
pub fn is_unicode_emoji(s: &str) -> bool {
    emojis::get(s).is_some()
}

#[cfg(test)]
mod unit_test {
    #[allow(deprecated)]
    use super::is_unicode_emoji;

    #[test]
    #[allow(deprecated)]
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
