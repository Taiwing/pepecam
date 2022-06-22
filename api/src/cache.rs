//! Cache structure for the api.
//!
//! This is used to store arbitrary data in the api for a limited amount of
//! time. This has to be managed with rocket's `manage` method to be accessed as
//! a rocket `State` by the routes' handlers. It is key-value pair system with
//! setter, getter and remove functions.
//!
//! When creating a Cache for a given type and managing it with rocket, do not
//! forget to add it in the cleanup job of the `main` module. Otherwise the
//! expired values would not necessarily be deleted which could cause a memory
//! leak.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Cache item. Stores a value for a certain time.
struct CacheItem<T: Clone> {
    value: T,
    creation: Instant,
    lifetime: Duration,
}

impl<T: Clone> CacheItem<T> {
    /// Create a new cache item.
    fn new(value: &T, lifetime: Duration) -> Self {
        CacheItem {
            value: value.clone(),
            creation: Instant::now(),
            lifetime,
        }
    }

    /// Is the item expired or not.
    fn is_expired(&self) -> bool {
        self.creation.elapsed() > self.lifetime
    }
}

/// Cache. Thread safe local cache system. This is an equivalent of Redis but
/// without Redis.
#[derive(Clone)]
pub struct Cache<T: Clone> {
    safe: Arc<Mutex<HashMap<String, CacheItem<T>>>>,
}

impl<T: Clone> Cache<T> {
    /// Create a new instance of the Cache structure.
    pub fn new() -> Self {
        Cache {
            safe: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a key-value pair in the cache. Returns the old value if a value
    /// already existed for the given key. The lifetime is the duration of
    /// storage inside the Cache. Passed it the data will be deleted.
    pub fn set(&self, key: &str, value: &T, lifetime: Duration) -> Option<T> {
        let mut map = self.safe.lock().unwrap();
        let old_item =
            map.insert(key.to_string(), CacheItem::new(value, lifetime));
        match old_item {
            Some(item) => {
                if item.is_expired() {
                    None
                } else {
                    Some(item.value)
                }
            }
            None => None,
        }
    }

    /// Get a value from the Cache.
    pub fn get(&self, key: &str) -> Option<T> {
        let mut map = self.safe.lock().unwrap();
        match map.get(key) {
            Some(item) => {
                if item.is_expired() {
                    map.remove(key);
                    None
                } else {
                    Some(item.value.clone())
                }
            }
            None => None,
        }
    }

    /// Delete a value from the Cache. Returns the old value if it was still
    /// valid. So this method can basically be used as a 'pop' function.
    pub fn del(&self, key: &str) -> Option<T> {
        let mut map = self.safe.lock().unwrap();
        match map.remove(key) {
            Some(item) => {
                if item.is_expired() {
                    None
                } else {
                    Some(item.value)
                }
            }
            None => None,
        }
    }

    /// Check if a given key exists in the cache.
    pub fn exists(&self, key: &str) -> bool {
        let map = self.safe.lock().unwrap();
        map.contains_key(key)
    }

    /// Cleanup the cache by removing expired items.
    pub fn cleanup(&self) {
        let mut map = self.safe.lock().unwrap();
        map.retain(|_, item| item.is_expired() == false);
    }
}
