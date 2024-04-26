use crate::database::{redis_conn, redis_key};
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("Data serialization error: {0}")]
    SerializeError(#[from] rmp_serde::encode::Error),
    #[error("Data deserialization error: {0}")]
    DeserializeError(#[from] rmp_serde::decode::Error),
}

fn prefix_key(key: &str) -> String {
    redis_key(format!("cache:{}", key))
}

pub fn set_cache<V: for<'a> Deserialize<'a> + Serialize>(
    key: &str,
    value: &V,
    expire_seconds: u64,
) -> Result<(), CacheError> {
    redis_conn()?.set_ex(
        prefix_key(key),
        rmp_serde::encode::to_vec(&value)?,
        expire_seconds,
    )?;
    Ok(())
}

pub fn get_cache<V: for<'a> Deserialize<'a> + Serialize>(
    key: &str,
) -> Result<Option<V>, CacheError> {
    let serialized_value: Option<Vec<u8>> = redis_conn()?.get(prefix_key(key))?;
    Ok(match serialized_value {
        Some(v) => Some(rmp_serde::from_slice::<V>(v.as_ref())?),
        None => None,
    })
}

pub fn delete_cache(key: &str) -> Result<(), CacheError> {
    Ok(redis_conn()?.del(prefix_key(key))?)
}

#[cfg(test)]
mod unit_test {
    use super::{get_cache, set_cache};
    use pretty_assertions::assert_eq;

    #[test]
    fn set_get_expire() {
        #[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug)]
        struct Data {
            id: u32,
            kind: String,
        }

        let key_1 = "CARGO_TEST_CACHE_KEY_1";
        let value_1: Vec<i32> = vec![1, 2, 3, 4, 5];

        let key_2 = "CARGO_TEST_CACHE_KEY_2";
        let value_2 = "Hello fedizens".to_string();

        let key_3 = "CARGO_TEST_CACHE_KEY_3";
        let value_3 = Data {
            id: 1000000007,
            kind: "prime number".to_string(),
        };

        set_cache(key_1, &value_1, 1).unwrap();
        set_cache(key_2, &value_2, 1).unwrap();
        set_cache(key_3, &value_3, 1).unwrap();

        let cached_value_1: Vec<i32> = get_cache(key_1).unwrap().unwrap();
        let cached_value_2: String = get_cache(key_2).unwrap().unwrap();
        let cached_value_3: Data = get_cache(key_3).unwrap().unwrap();

        assert_eq!(value_1, cached_value_1);
        assert_eq!(value_2, cached_value_2);
        assert_eq!(value_3, cached_value_3);

        // wait for the cache to expire
        std::thread::sleep(std::time::Duration::from_millis(1100));

        let expired_value_1: Option<Vec<i32>> = get_cache(key_1).unwrap();
        let expired_value_2: Option<Vec<i32>> = get_cache(key_2).unwrap();
        let expired_value_3: Option<Vec<i32>> = get_cache(key_3).unwrap();

        assert!(expired_value_1.is_none());
        assert!(expired_value_2.is_none());
        assert!(expired_value_3.is_none());
    }
}
