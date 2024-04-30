use crate::config::CONFIG;
use once_cell::sync::OnceCell;
use reqwest::{Client, Error, NoProxy, Proxy};
use std::time::Duration;

static CLIENT: OnceCell<Client> = OnceCell::new();

pub fn http_client() -> Result<Client, Error> {
    CLIENT
        .get_or_try_init(|| {
            let mut builder = Client::builder().timeout(Duration::from_secs(5));

            if let Some(proxy_url) = &CONFIG.proxy {
                let mut proxy = Proxy::all(proxy_url)?;
                if let Some(proxy_bypass_hosts) = &CONFIG.proxy_bypass_hosts {
                    proxy = proxy.no_proxy(NoProxy::from_string(&proxy_bypass_hosts.join(",")));
                }
                builder = builder.proxy(proxy);
            }

            builder.build()
        })
        .cloned()
}
