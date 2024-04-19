use crate::config::server::CONFIG;
use redis::{Client, Connection, RedisError};

static REDIS_CLIENT: once_cell::sync::OnceCell<Client> = once_cell::sync::OnceCell::new();

fn init_redis() -> Result<Client, RedisError> {
    let redis_url = {
        let mut params = vec!["redis://".to_owned()];

        let redis = if let Some(cache_server) = &CONFIG.cache_server {
            cache_server
        } else {
            &CONFIG.redis
        };

        if let Some(user) = &redis.user {
            params.push(user.to_string())
        }
        if let Some(pass) = &redis.pass {
            params.push(format!(":{}@", pass))
        }
        params.push(redis.host.to_string());
        params.push(format!(":{}", redis.port));
        params.push(format!("/{}", redis.db));

        params.concat()
    };

    Client::open(redis_url)
}

pub fn redis_conn() -> Result<Connection, RedisError> {
    match REDIS_CLIENT.get() {
        Some(client) => Ok(client.get_connection()?),
        None => init_redis()?.get_connection(),
    }
}

#[inline]
/// prefix redis key
pub fn key(key: impl ToString) -> String {
    format!("{}:{}", CONFIG.redis_key_prefix, key.to_string())
}

#[cfg(test)]
mod unit_test {
    use super::init_redis;

    #[test]
    fn connect_test() {
        assert!(init_redis().is_ok());
    }
}
