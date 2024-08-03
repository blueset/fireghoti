//! In-memory cache handler

use chrono::{DateTime, Duration, Utc};
use std::sync::Mutex;

/// Cache stored directly in memory
pub struct Cache<T: Clone> {
    cache: Mutex<TimedData<T>>,
    ttl: Option<Duration>,
}

struct TimedData<T: Clone> {
    value: Option<T>,
    last_updated: DateTime<Utc>,
}

impl<T: Clone> Default for Cache<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Cache<T> {
    /// Creates a new cache object with no auto invalidation.
    pub const fn new() -> Self {
        Self {
            cache: Mutex::new(TimedData {
                value: None,
                last_updated: DateTime::UNIX_EPOCH,
            }),
            ttl: None,
        }
    }

    /// Creates a new cache object whose content is invalidated
    /// in the specified duration.
    ///
    /// # Example
    /// ```
    /// # use backend_rs::cache::Cache;
    /// use chrono::Duration;
    /// static CACHE: Cache<i32> = Cache::new_with_ttl(Duration::seconds(1));
    ///
    /// fn use_cache() {
    ///     let data = 998244353;
    ///
    ///     // Set cache
    ///     CACHE.set(data);
    ///
    ///     // wait for the cache to expire
    ///     std::thread::sleep(std::time::Duration::from_millis(1100));
    ///
    ///     // Get cache
    ///     let cache = CACHE.get();
    ///
    ///     assert!(cache.is_none());
    /// }
    /// ```
    pub const fn new_with_ttl(ttl: Duration) -> Self {
        Self {
            cache: Mutex::new(TimedData {
                value: None,
                last_updated: DateTime::UNIX_EPOCH,
            }),
            ttl: Some(ttl),
        }
    }

    /// Sets a cache. This function overwrites the existing data.
    ///
    /// # Example
    /// ```
    /// # use backend_rs::cache::Cache;
    /// static CACHE: Cache<i32> = Cache::new();
    ///
    /// fn use_cache() {
    ///     let data = 998244353;
    ///
    ///     // Set cache
    ///     CACHE.set(data);
    ///
    ///     // Get cache
    ///     let cache = CACHE.get();
    ///
    ///     if let Some(cached_data) = cache {
    ///         println!("found a cached value");
    ///         assert_eq!(data, cached_data)
    ///     } else {
    ///         println!("cache not found");
    ///     }
    /// }
    /// ```
    pub fn set(&self, value: T) {
        if self.ttl.is_none() {
            let _ = self.cache.lock().map(|mut cache| cache.value = Some(value));
        } else {
            let _ = self.cache.lock().map(|mut cache| {
                *cache = TimedData {
                    value: Some(value),
                    last_updated: Utc::now(),
                }
            });
        }
    }

    /// Gets a cache. Returns [`None`] is the cache is not set or expired.
    pub fn get(&self) -> Option<T> {
        let data = self.cache.lock().ok()?;

        if let Some(ttl) = self.ttl {
            if data.last_updated + ttl < Utc::now() {
                return None;
            }
        }
        data.value.to_owned()
    }
}
