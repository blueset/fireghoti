//! Fetch latest Firefish version from the Firefish repository

use crate::database::cache;
use crate::util::http_client;
use isahc::ReadResponseExt;
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cache error: {0}")]
    Cache(#[from] cache::Error),
    #[error("Isahc error: {0}")]
    Isahc(#[from] isahc::Error),
    #[error("HTTP client aquisition error: {0}")]
    HttpClient(#[from] http_client::Error),
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Response parsing error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to deserialize JSON: {0}")]
    Json(#[from] serde_json::Error),
}

const UPSTREAM_PACKAGE_JSON_URL: &str =
    "https://firefish.dev/firefish/firefish/-/raw/main/package.json";

async fn get_latest_version() -> Result<String, Error> {
    #[derive(Debug, Deserialize)]
    struct Response {
        version: String,
    }

    let mut response = http_client::client()?.get(UPSTREAM_PACKAGE_JSON_URL)?;

    if !response.status().is_success() {
        tracing::info!("status: {}", response.status());
        tracing::debug!("response body: {:#?}", response.body());
        return Err(Error::Http(
            "Failed to fetch version from Firefish GitLab".to_string(),
        ));
    }

    let res_parsed: Response = serde_json::from_str(&response.text()?)?;

    Ok(res_parsed.version)
}

/// Returns the latest Firefish version.
#[crate::export]
pub async fn latest_version() -> Result<String, Error> {
    let version: Option<String> =
        cache::get_one(cache::Category::FetchUrl, UPSTREAM_PACKAGE_JSON_URL).await?;

    if let Some(v) = version {
        tracing::trace!("use cached value: {}", v);
        Ok(v)
    } else {
        tracing::trace!("cache is expired, fetching the latest version");
        let fetched_version = get_latest_version().await?;
        tracing::trace!("fetched value: {}", fetched_version);

        cache::set_one(
            cache::Category::FetchUrl,
            UPSTREAM_PACKAGE_JSON_URL,
            &fetched_version,
            3 * 60 * 60,
        )
        .await?;
        Ok(fetched_version)
    }
}

#[cfg(test)]
mod unit_test {
    use super::{latest_version, UPSTREAM_PACKAGE_JSON_URL};
    use crate::database::cache;

    fn validate_version(version: String) {
        // version: YYYYMMDD or YYYYMMDD-X
        assert!(version.len() >= 8);
        assert!(version[..8].chars().all(|c| c.is_ascii_digit()));

        // YYYY
        assert!(&version[..4] >= "2024");

        // MM
        assert!(&version[4..6] >= "01");
        assert!(&version[4..6] <= "12");

        // DD
        assert!(&version[6..8] >= "01");
        assert!(&version[6..8] <= "31");

        // -X
        if version.len() > 8 {
            assert!(version.chars().nth(8).unwrap() == '-');
            assert!(version[9..].chars().all(|c| c.is_ascii_digit()));
        }
    }

    #[tokio::test]
    async fn check_version() {
        // delete caches in case you run this test multiple times
        cache::delete_one(cache::Category::FetchUrl, UPSTREAM_PACKAGE_JSON_URL)
            .await
            .unwrap();

        // fetch from firefish.dev
        validate_version(latest_version().await.unwrap());

        // use cache
        validate_version(latest_version().await.unwrap());
    }
}
