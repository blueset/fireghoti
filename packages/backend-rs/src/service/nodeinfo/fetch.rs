use crate::service::nodeinfo::schema::*;
use crate::util::http_client;
use isahc::AsyncReadResponseExt;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Http client aquisition error: {0}")]
    HttpClientErr(#[from] http_client::Error),
    #[error("Http error: {0}")]
    HttpErr(#[from] isahc::Error),
    #[error("Bad status: {0}")]
    BadStatus(String),
    #[error("Failed to parse response body as text: {0}")]
    ResponseErr(#[from] std::io::Error),
    #[error("Failed to parse response body as json: {0}")]
    JsonErr(#[from] serde_json::Error),
    #[error("No nodeinfo provided")]
    MissingNodeinfo,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeinfoLinks {
    links: Vec<NodeinfoLink>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeinfoLink {
    rel: String,
    href: String,
}

#[inline]
fn wellknown_nodeinfo_url(host: &str) -> String {
    format!("https://{}/.well-known/nodeinfo", host)
}

async fn fetch_nodeinfo_links(host: &str) -> Result<NodeinfoLinks, Error> {
    let client = http_client::client()?;
    let wellknown_url = wellknown_nodeinfo_url(host);
    let mut wellknown_response = client.get_async(&wellknown_url).await?;

    if !wellknown_response.status().is_success() {
        tracing::debug!("{:#?}", wellknown_response.body());
        return Err(Error::BadStatus(format!(
            "{} returned {}",
            wellknown_url,
            wellknown_response.status()
        )));
    }

    Ok(serde_json::from_str(&wellknown_response.text().await?)?)
}

fn check_nodeinfo_link(links: NodeinfoLinks) -> Result<String, Error> {
    for link in links.links {
        if link.rel == "http://nodeinfo.diaspora.software/ns/schema/2.1"
            || link.rel == "http://nodeinfo.diaspora.software/ns/schema/2.0"
        {
            return Ok(link.href);
        }
    }

    Err(Error::MissingNodeinfo)
}

async fn fetch_nodeinfo_impl(nodeinfo_link: &str) -> Result<Nodeinfo20, Error> {
    let client = http_client::client()?;
    let mut response = client.get_async(nodeinfo_link).await?;

    if !response.status().is_success() {
        tracing::debug!("{:#?}", response.body());
        return Err(Error::BadStatus(format!(
            "{} returned {}",
            nodeinfo_link,
            response.status()
        )));
    }

    Ok(serde_json::from_str(&response.text().await?)?)
}

// for napi export
type Nodeinfo = Nodeinfo20;

#[crate::export]
pub async fn fetch_nodeinfo(host: &str) -> Result<Nodeinfo, Error> {
    tracing::info!("fetching from {}", host);
    let links = fetch_nodeinfo_links(host).await?;
    let nodeinfo_link = check_nodeinfo_link(links)?;
    fetch_nodeinfo_impl(&nodeinfo_link).await
}

#[cfg(test)]
mod unit_test {
    use super::{check_nodeinfo_link, fetch_nodeinfo, NodeinfoLink, NodeinfoLinks};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_check_nodeinfo_link() {
        let links_1 = NodeinfoLinks {
            links: vec![
                NodeinfoLink {
                    rel: "https://example.com/incorrect/schema/2.0".to_string(),
                    href: "https://example.com/dummy".to_string(),
                },
                NodeinfoLink {
                    rel: "http://nodeinfo.diaspora.software/ns/schema/2.0".to_string(),
                    href: "https://example.com/real".to_string(),
                },
            ],
        };
        assert_eq!(
            check_nodeinfo_link(links_1).unwrap(),
            "https://example.com/real"
        );

        let links_2 = NodeinfoLinks {
            links: vec![
                NodeinfoLink {
                    rel: "https://example.com/incorrect/schema/2.0".to_string(),
                    href: "https://example.com/dummy".to_string(),
                },
                NodeinfoLink {
                    rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string(),
                    href: "https://example.com/real".to_string(),
                },
            ],
        };
        assert_eq!(
            check_nodeinfo_link(links_2).unwrap(),
            "https://example.com/real"
        );

        let links_3 = NodeinfoLinks {
            links: vec![
                NodeinfoLink {
                    rel: "https://example.com/incorrect/schema/2.0".to_string(),
                    href: "https://example.com/dummy/2.0".to_string(),
                },
                NodeinfoLink {
                    rel: "https://example.com/incorrect/schema/2.1".to_string(),
                    href: "https://example.com/dummy/2.1".to_string(),
                },
            ],
        };
        check_nodeinfo_link(links_3).expect_err("No nodeinfo");
    }

    #[tokio::test]
    async fn test_fetch_nodeinfo() {
        assert_eq!(
            fetch_nodeinfo("info.firefish.dev")
                .await
                .unwrap()
                .software
                .name,
            "firefish"
        );
    }
}
