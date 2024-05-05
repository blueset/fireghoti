use crate::config::CONFIG;
use isahc::{config::*, HttpClient};
use once_cell::sync::OnceCell;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Isahc error: {0}")]
    IsahcErr(#[from] isahc::Error),
    #[error("Url parse error: {0}")]
    UrlParseErr(#[from] isahc::http::uri::InvalidUri),
}

static CLIENT: OnceCell<HttpClient> = OnceCell::new();

pub fn client() -> Result<HttpClient, Error> {
    CLIENT
        .get_or_try_init(|| {
            let mut builder = HttpClient::builder()
                .timeout(Duration::from_secs(10))
                .default_header("user-agent", &CONFIG.user_agent)
                .dns_cache(DnsCache::Timeout(Duration::from_secs(60 * 60)));

            if let Some(proxy_url) = &CONFIG.proxy {
                builder = builder.proxy(Some(proxy_url.parse()?));
                if let Some(proxy_bypass_hosts) = &CONFIG.proxy_bypass_hosts {
                    builder = builder.proxy_blacklist(proxy_bypass_hosts);
                }
            }

            Ok(builder.build()?)
        })
        .cloned()
}
