use crate::config::CONFIG;
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
    use super::redis_conn;
    use pretty_assertions::assert_eq;
    use redis::Commands;

    #[test]
    fn connect() {
        assert!(redis_conn().is_ok());
        assert!(redis_conn().is_ok());
    }

    #[test]
    fn access() {
        let mut redis = redis_conn().unwrap();

        let key = "CARGO_UNIT_TEST_KEY";
        let value = "CARGO_UNIT_TEST_VALUE";

        assert_eq!(redis.set::<&str, &str, String>(key, value).unwrap(), "OK");
        assert_eq!(redis.get::<&str, String>(key).unwrap(), value);
        assert_eq!(redis.del::<&str, u32>(key).unwrap(), 1);
    }
}
