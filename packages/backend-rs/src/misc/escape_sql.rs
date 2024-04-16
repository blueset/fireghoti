#[crate::export]
pub fn sql_like_escape(src: &str) -> String {
    src.replace('%', r"\%").replace('_', r"\_")
}

#[crate::export]
pub fn safe_for_sql(src: &str) -> bool {
    !src.contains([
        '\0', '\x08', '\x09', '\x1a', '\n', '\r', '"', '\'', '\\', '%',
    ])
}

#[cfg(test)]
mod unit_test {
    use super::{safe_for_sql, sql_like_escape};
    use pretty_assertions::assert_eq;

    #[test]
    fn sql_like_escape_test() {
        assert_eq!(sql_like_escape(""), "");
        assert_eq!(sql_like_escape("abc"), "abc");
        assert_eq!(sql_like_escape("a%bc"), r"a\%bc");
        assert_eq!(sql_like_escape("a呼%吸bc"), r"a呼\%吸bc");
        assert_eq!(sql_like_escape("a呼%吸b%_c"), r"a呼\%吸b\%\_c");
        assert_eq!(sql_like_escape("_اللغة العربية"), r"\_اللغة العربية");
    }

    #[test]
    fn safe_for_sql_test() {
        assert!(safe_for_sql("123"));
        assert!(safe_for_sql("人間"));
        assert!(!safe_for_sql("人間\x09"));
        assert!(!safe_for_sql("abc\ndef"));
        assert!(!safe_for_sql("%something%"));
    }
}
