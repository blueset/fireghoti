use crate::config::server::CONFIG;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Idna error: {0}")]
    IdnaError(#[from] idna::Errors),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Hostname is missing")]
    NoHostname,
}

#[crate::export]
pub fn get_full_ap_account(username: &str, host: Option<&str>) -> Result<String, Error> {
    Ok(match host {
        Some(host) => format!("{}@{}", username, to_puny(host)?),
        None => format!("{}@{}", username, extract_host(&CONFIG.url)?),
    })
}

#[crate::export]
pub fn is_self_host(host: Option<&str>) -> Result<bool, Error> {
    Ok(match host {
        Some(host) => extract_host(&CONFIG.url)? == to_puny(host)?,
        None => true,
    })
}

#[crate::export]
pub fn is_same_origin(uri: &str) -> Result<bool, Error> {
    Ok(url::Url::parse(uri)?.origin().ascii_serialization() == CONFIG.url)
}

#[crate::export]
pub fn extract_host(uri: &str) -> Result<String, Error> {
    url::Url::parse(uri)?
        .host_str()
        .ok_or(Error::NoHostname)
        .and_then(|v| Ok(to_puny(v)?))
}

#[crate::export]
pub fn to_puny(host: &str) -> Result<String, idna::Errors> {
    idna::domain_to_ascii(host)
}

#[cfg(test)]
mod unit_test {
    use super::{extract_host, to_puny};
    use pretty_assertions::assert_eq;

    #[test]
    fn extract_host_test() {
        assert_eq!(
            extract_host("https://firefish.dev/firefish/firefish.git").unwrap(),
            "firefish.dev"
        );
    }

    #[test]
    fn to_puny_test() {
        assert_eq!(
            to_puny("何もかも.owari.shop").unwrap(),
            "xn--u8jyfb5762a.owari.shop"
        );
    }
}
