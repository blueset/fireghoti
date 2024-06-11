#[macros::export]
pub fn is_safe_url(url: &str) -> bool {
    if let Ok(url) = url.parse::<url::Url>() {
        if url.host_str().unwrap_or_default() == "unix"
            || !["http", "https"].contains(&url.scheme())
            || ![None, Some(80), Some(443)].contains(&url.port())
        {
            return false;
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn is_safe_url() {
        assert!(super::is_safe_url("http://firefish.dev/firefish/firefish"));
        assert!(super::is_safe_url("https://firefish.dev/firefish/firefish"));
        assert!(super::is_safe_url(
            "http://firefish.dev:80/firefish/firefish"
        ));
        assert!(super::is_safe_url(
            "https://firefish.dev:80/firefish/firefish"
        ));
        assert!(super::is_safe_url(
            "http://firefish.dev:443/firefish/firefish"
        ));
        assert!(super::is_safe_url(
            "https://firefish.dev:443/firefish/firefish"
        ));
        assert!(!super::is_safe_url("https://unix/firefish/firefish"));
        assert!(!super::is_safe_url(
            "https://firefish.dev:35/firefish/firefish"
        ));
        assert!(!super::is_safe_url("ftp://firefish.dev/firefish/firefish"));
        assert!(!super::is_safe_url("nyaa"));
        assert!(!super::is_safe_url(""));
    }
}
