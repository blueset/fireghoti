use crate::database::cache;
use crate::util::http_client::http_client;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cache error: {0}")]
    CacheErr(#[from] cache::Error),
    #[error("Reqwest error: {0}")]
    ReqwestErr(#[from] reqwest::Error),
    #[error("Failed to deserialize JSON: {0}")]
    JsonErr(#[from] serde_json::Error),
}

const UPSTREAM_PACKAGE_JSON_URL: &'static str =
    "https://firefish.dev/firefish/firefish/-/raw/main/package.json";

async fn get_latest_version() -> Result<String, Error> {
    #[derive(Debug, Deserialize, Serialize)]
    struct Response {
        version: String,
    }

    let res = http_client()?
        .get(UPSTREAM_PACKAGE_JSON_URL)
        .send()
        .await?
        .text()
        .await?;
    let res_parsed: Response = serde_json::from_str(&res)?;

    Ok(res_parsed.version)
}

#[crate::export]
pub async fn latest_version() -> Result<String, Error> {
    let version: Option<String> =
        cache::get_one(cache::Category::FetchUrl, UPSTREAM_PACKAGE_JSON_URL)?;

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
        )?;
        Ok(fetched_version)
    }
}

#[cfg(test)]
mod unit_test {
    use super::{latest_version, UPSTREAM_PACKAGE_JSON_URL};
    use crate::database::cache;

    fn validate_version(version: String) {
        // version: YYYYMMDD
        assert!(version.len() == 8);
        assert!(version.chars().all(|c| c.is_ascii_digit()));

        // YYYY
        assert!(&version[..4] >= "2024");

        // MM
        assert!(&version[4..6] >= "01");
        assert!(&version[4..6] <= "12");

        // DD
        assert!(&version[6..] >= "01");
        assert!(&version[6..] <= "31");
    }

    #[tokio::test]
    async fn check_version() {
        // TODO: don't need to do this in CI tasks
        cache::delete_one(cache::Category::FetchUrl, UPSTREAM_PACKAGE_JSON_URL).unwrap();

        // fetch from firefish.dev
        validate_version(latest_version().await.unwrap());

        // use cache
        validate_version(latest_version().await.unwrap());
    }
}
