use crate::config::CONFIG;
use once_cell::sync::OnceCell;
use redis::{aio::MultiplexedConnection, Client, RedisError};

static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();

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

    tracing::info!("Initializing Redis client");

    Client::open(redis_url)
}

pub async fn redis_conn() -> Result<MultiplexedConnection, RedisError> {
    match REDIS_CLIENT.get() {
        Some(client) => Ok(client.get_multiplexed_async_connection().await?),
        None => init_redis()?.get_multiplexed_async_connection().await,
    }
}

/// prefix redis key
#[inline]
pub fn key(key: impl ToString) -> String {
    format!("{}:{}", CONFIG.redis_key_prefix, key.to_string())
}

#[cfg(test)]
mod unit_test {
    use super::redis_conn;
    use pretty_assertions::assert_eq;
    use redis::AsyncCommands;

    #[tokio::test]
    async fn connect() {
        assert!(redis_conn().await.is_ok());
        assert!(redis_conn().await.is_ok());
    }

    #[tokio::test]
    async fn access() {
        let mut redis = redis_conn().await.unwrap();

        let key = "CARGO_UNIT_TEST_KEY";
        let value = "CARGO_UNIT_TEST_VALUE";

        assert_eq!(
            redis.set::<&str, &str, String>(key, value).await.unwrap(),
            "OK"
        );
        assert_eq!(redis.get::<&str, String>(key).await.unwrap(), value);
        assert_eq!(redis.del::<&str, u32>(key).await.unwrap(), 1);
    }
}
