//! This module is used in the TypeScript backend only.
// We may want to (re)implement these functions in the `federation` module
// in a Rusty way (e.g., traits of actor type) if needed.

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Idna error: {0}")]
    Idna(#[from] idna::Errors),
    #[error("Url parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Hostname is missing")]
    NoHostname,
}

#[crate::ts_export]
pub fn get_full_ap_account(username: &str, host: Option<&str>) -> Result<String, Error> {
    Ok(match host {
        Some(host) => format!("{}@{}", username, to_puny(host)?),
        None => format!("{}@{}", username, extract_host(&crate::config::CONFIG.url)?),
    })
}

#[crate::ts_export]
pub fn is_self_host(host: Option<&str>) -> Result<bool, Error> {
    Ok(match host {
        Some(host) => extract_host(&crate::config::CONFIG.url)? == to_puny(host)?,
        None => true,
    })
}

#[crate::ts_export]
pub fn is_same_origin(uri: &str) -> Result<bool, Error> {
    Ok(url::Url::parse(uri)?.origin().ascii_serialization() == crate::config::CONFIG.url)
}

#[crate::ts_export]
pub fn extract_host(uri: &str) -> Result<String, Error> {
    url::Url::parse(uri)?
        .host_str()
        .ok_or(Error::NoHostname)
        .and_then(|v| Ok(to_puny(v)?))
}

#[crate::ts_export]
pub fn to_puny(host: &str) -> Result<String, idna::Errors> {
    idna::domain_to_ascii(host)
}
