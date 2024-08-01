//! In-memory cache handler

use chrono::{DateTime, Duration, Utc};
use std::sync::Mutex;

pub struct Cache<T: Clone> {
    cache: Mutex<TimedData<T>>,
    ttl: Option<Duration>,
}

struct TimedData<T: Clone> {
    value: Option<T>,
    last_updated: DateTime<Utc>,
}

impl<T: Clone> Cache<T> {
    pub const fn new(ttl: Option<Duration>) -> Self {
        Self {
            cache: Mutex::new(TimedData {
                value: None,
                last_updated: DateTime::UNIX_EPOCH,
            }),
            ttl,
        }
    }

    pub fn set(&self, value: T) {
        if self.ttl.is_none() {
            let _ = self
                .cache
                .lock()
                .map(|mut cache| (*cache).value = Some(value));
        } else {
            let _ = self.cache.lock().map(|mut cache| {
                *cache = TimedData {
                    value: Some(value),
                    last_updated: Utc::now(),
                }
            });
        }
    }

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
