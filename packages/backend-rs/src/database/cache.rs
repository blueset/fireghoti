use crate::database::{redis_conn, redis_key};
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

#[derive(strum::Display)]
pub enum Category {
    #[strum(serialize = "fetchUrl")]
    FetchUrl,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("Data serialization error: {0}")]
    SerializeError(#[from] rmp_serde::encode::Error),
    #[error("Data deserialization error: {0}")]
    DeserializeError(#[from] rmp_serde::decode::Error),
}

fn categorize(category: Category, key: &str) -> String {
    format!("{}:{}", category, key)
}

fn prefix_key(key: &str) -> String {
    redis_key(format!("cache:{}", key))
}

pub fn set<V: for<'a> Deserialize<'a> + Serialize>(
    key: &str,
    value: &V,
    expire_seconds: u64,
) -> Result<(), Error> {
    redis_conn()?.set_ex(
        prefix_key(key),
        rmp_serde::encode::to_vec(&value)?,
        expire_seconds,
    )?;
    Ok(())
}

pub fn get<V: for<'a> Deserialize<'a> + Serialize>(key: &str) -> Result<Option<V>, Error> {
    let serialized_value: Option<Vec<u8>> = redis_conn()?.get(prefix_key(key))?;
    Ok(match serialized_value {
        Some(v) => Some(rmp_serde::from_slice::<V>(v.as_ref())?),
        None => None,
    })
}

pub fn delete(key: &str) -> Result<(), Error> {
    Ok(redis_conn()?.del(prefix_key(key))?)
}

pub fn set_one<V: for<'a> Deserialize<'a> + Serialize>(
    category: Category,
    key: &str,
    value: &V,
    expire_seconds: u64,
) -> Result<(), Error> {
    set(&categorize(category, key), value, expire_seconds)
}

pub fn get_one<V: for<'a> Deserialize<'a> + Serialize>(
    category: Category,
    key: &str,
) -> Result<Option<V>, Error> {
    get(&categorize(category, key))
}

pub fn delete_one(category: Category, key: &str) -> Result<(), Error> {
    delete(&categorize(category, key))
}

// TODO: set_all(), get_all(), delete_all()

#[cfg(test)]
mod unit_test {
    use super::{get, set};
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

        set(key_1, &value_1, 1).unwrap();
        set(key_2, &value_2, 1).unwrap();
        set(key_3, &value_3, 1).unwrap();

        let cached_value_1: Vec<i32> = get(key_1).unwrap().unwrap();
        let cached_value_2: String = get(key_2).unwrap().unwrap();
        let cached_value_3: Data = get(key_3).unwrap().unwrap();

        assert_eq!(value_1, cached_value_1);
        assert_eq!(value_2, cached_value_2);
        assert_eq!(value_3, cached_value_3);

        // wait for the cache to expire
        std::thread::sleep(std::time::Duration::from_millis(1100));

        let expired_value_1: Option<Vec<i32>> = get(key_1).unwrap();
        let expired_value_2: Option<Vec<i32>> = get(key_2).unwrap();
        let expired_value_3: Option<Vec<i32>> = get(key_3).unwrap();

        assert!(expired_value_1.is_none());
        assert!(expired_value_2.is_none());
        assert!(expired_value_3.is_none());
    }
}
