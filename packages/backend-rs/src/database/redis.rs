use crate::config::CONFIG;
use async_trait::async_trait;
use bb8::{ManageConnection, Pool, PooledConnection, RunError};
use redis::{aio::MultiplexedConnection, Client, ErrorKind, IntoConnectionInfo, RedisError};
use tokio::sync::OnceCell;

/// A `bb8::ManageConnection` for `redis::Client::get_multiplexed_async_connection`.
#[derive(Clone, Debug)]
pub struct RedisConnectionManager {
    client: Client,
}

impl RedisConnectionManager {
    /// Create a new `RedisConnectionManager`.
    /// See `redis::Client::open` for a description of the parameter types.
    pub fn new<T: IntoConnectionInfo>(info: T) -> Result<Self, RedisError> {
        Ok(Self {
            client: Client::open(info.into_connection_info()?)?,
        })
    }
}

#[async_trait]
impl ManageConnection for RedisConnectionManager {
    type Connection = MultiplexedConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_multiplexed_async_connection().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

static CONN_POOL: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

async fn init_conn_pool() -> Result<(), RedisError> {
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

    tracing::info!("Initializing connection manager");
    let manager = RedisConnectionManager::new(redis_url)?;

    tracing::info!("Creating connection pool");
    let pool = Pool::builder().build(manager).await?;

    CONN_POOL.get_or_init(|| async { pool }).await;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum RedisConnError {
    #[error("Failed to initialize Redis connection pool: {0}")]
    RedisErr(RedisError),
    #[error("Redis connection pool error: {0}")]
    Bb8PoolErr(RunError<RedisError>),
}

pub async fn redis_conn(
) -> Result<PooledConnection<'static, RedisConnectionManager>, RedisConnError> {
    if !CONN_POOL.initialized() {
        let init_res = init_conn_pool().await;

        if let Err(err) = init_res {
            return Err(RedisConnError::RedisErr(err));
        }
    }

    CONN_POOL
        .get()
        .unwrap()
        .get()
        .await
        .map_err(RedisConnError::Bb8PoolErr)
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
