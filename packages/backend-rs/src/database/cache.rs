use crate::database::{redis_conn, redis_key};
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};

#[derive(strum::Display, Debug)]
pub enum Category {
    #[strum(serialize = "fetchUrl")]
    FetchUrl,
    #[strum(serialize = "blocking")]
    Block,
    #[strum(serialize = "following")]
    Follow,
    #[cfg(test)]
    #[strum(serialize = "usedOnlyForTesting")]
    Test,
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

#[inline]
fn prefix_key(key: &str) -> String {
    redis_key(format!("cache:{}", key))
}

#[inline]
fn categorize(category: Category, key: &str) -> String {
    format!("{}:{}", category, key)
}

#[inline]
fn wildcard(category: Category) -> String {
    prefix_key(&categorize(category, "*"))
}

/// Sets a Redis cache.
///
/// This overwrites the exsisting cache with the same key.
///
/// ## Arguments
///
/// * `key` - key (will be prefixed automatically)
/// * `value` - (de)serializable value
/// * `expire_seconds` - TTL
///
/// ## Example
///
/// ```
/// # use backend_rs::database::cache;
/// let key = "apple";
/// let data = "I want to cache this string".to_string();
///
/// // caches the data for 10 seconds
/// cache::set(key, &data, 10);
///
/// // get the cache
/// let cached_data = cache::get::<String>(key).unwrap();
/// assert_eq!(data, cached_data.unwrap());
/// ```
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

/// Gets a Redis cache.
///
/// If the Redis connection is fine, this returns `Ok(data)` where `data`
/// is the cached value. Returns `Ok(None)` if there is no value corresponding to `key`.
///
/// ## Arguments
///
/// * `key` - key (will be prefixed automatically)
///
/// ## Example
///
/// ```
/// # use backend_rs::database::cache;
/// let key = "banana";
/// let data = "I want to cache this string".to_string();
///
/// // set cache
/// cache::set(key, &data, 10).unwrap();
///
/// // get cache
/// let cached_data = cache::get::<String>(key).unwrap();
/// assert_eq!(data, cached_data.unwrap());
///
/// // get nonexistent (or expired) cache
/// let no_cache = cache::get::<String>("nonexistent").unwrap();
/// assert!(no_cache.is_none());
/// ```
pub fn get<V: for<'a> Deserialize<'a> + Serialize>(key: &str) -> Result<Option<V>, Error> {
    let serialized_value: Option<Vec<u8>> = redis_conn()?.get(prefix_key(key))?;
    Ok(match serialized_value {
        Some(v) => Some(rmp_serde::from_slice::<V>(v.as_ref())?),
        None => None,
    })
}

/// Deletes a Redis cache.
///
/// If the Redis connection is fine, this returns `Ok(())`
/// regardless of whether the cache exists.
///
/// ## Arguments
///
/// * `key` - key (will be prefixed automatically)
///
/// ## Example
///
/// ```
/// # use backend_rs::database::cache;
/// let key = "chocolate";
/// let value = "I want to cache this string".to_string();
///
/// // set cache
/// cache::set(key, &value, 10).unwrap();
///
/// // delete the cache
/// cache::delete("foo").unwrap();
/// cache::delete("nonexistent").unwrap(); // this is okay
///
/// // the cache is gone
/// let cached_value = cache::get::<String>("foo").unwrap();
/// assert!(cached_value.is_none());
/// ```
pub fn delete(key: &str) -> Result<(), Error> {
    Ok(redis_conn()?.del(prefix_key(key))?)
}

/// Sets a Redis cache under a `category`.
///
/// The usage is the same as [set], except that you need to
/// use [get_one] and [delete_one] to get/delete the cache.
///
/// ## Arguments
///
/// * `category` - one of [Category]
/// * `key` - key (will be prefixed automatically)
/// * `value` - (de)serializable value
/// * `expire_seconds` - TTL
pub fn set_one<V: for<'a> Deserialize<'a> + Serialize>(
    category: Category,
    key: &str,
    value: &V,
    expire_seconds: u64,
) -> Result<(), Error> {
    set(&categorize(category, key), value, expire_seconds)
}

/// Gets a Redis cache under a `category`.
///
/// The usage is basically the same as [get].
///
/// ## Arguments
///
/// * `category` - one of [Category]
/// * `key` - key (will be prefixed automatically)
pub fn get_one<V: for<'a> Deserialize<'a> + Serialize>(
    category: Category,
    key: &str,
) -> Result<Option<V>, Error> {
    get(&categorize(category, key))
}

/// Deletes a Redis cache under a `category`.
///
/// The usage is basically the same as [delete].
///
/// ## Arguments
///
/// * `category` - one of [Category]
/// * `key` - key (will be prefixed automatically)
pub fn delete_one(category: Category, key: &str) -> Result<(), Error> {
    delete(&categorize(category, key))
}

/// Deletes all Redis caches under a `category`.
///
/// ## Arguments
///
/// * `category` - one of [Category]
pub fn delete_all(category: Category) -> Result<(), Error> {
    let mut redis = redis_conn()?;
    let keys: Vec<Vec<u8>> = redis.keys(wildcard(category))?;

    if !keys.is_empty() {
        redis.del(keys)?
    }

    Ok(())
}

// TODO: set_all(), get_all()

#[cfg(test)]
mod unit_test {
    use crate::database::cache::delete_one;

    use super::{delete_all, get, get_one, set, set_one, Category::Test};
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

    #[test]
    fn use_category() {
        let key_1 = "fire";
        let key_2 = "fish";
        let key_3 = "awawa";

        let value_1 = "hello".to_string();
        let value_2 = 998244353u32;
        let value_3 = 'あ';

        set_one(Test, key_1, &value_1, 5 * 60).unwrap();
        set_one(Test, key_2, &value_2, 5 * 60).unwrap();
        set_one(Test, key_3, &value_3, 5 * 60).unwrap();

        assert_eq!(get_one::<String>(Test, key_1).unwrap().unwrap(), value_1);
        assert_eq!(get_one::<u32>(Test, key_2).unwrap().unwrap(), value_2);
        assert_eq!(get_one::<char>(Test, key_3).unwrap().unwrap(), value_3);

        delete_one(Test, key_1).unwrap();

        assert!(get_one::<String>(Test, key_1).unwrap().is_none());
        assert!(get_one::<u32>(Test, key_2).unwrap().is_some());
        assert!(get_one::<char>(Test, key_3).unwrap().is_some());

        delete_all(Test).unwrap();

        assert!(get_one::<String>(Test, key_1).unwrap().is_none());
        assert!(get_one::<u32>(Test, key_2).unwrap().is_none());
        assert!(get_one::<char>(Test, key_3).unwrap().is_none());
    }
}
