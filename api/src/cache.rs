use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Cache item. Stores a string for a certain time.
struct CacheItem {
    value: String,
    creation: Instant,
    lifetime: Duration,
}

impl CacheItem {
    /// Create a new cache item.
    fn new(value: &str, lifetime: Duration) -> Self {
        CacheItem {
            value: value.to_string(),
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
pub struct Cache {
    safe: Arc<Mutex<HashMap<String, CacheItem>>>,
}

impl Cache {
    /// Create a new instance of the Cache structure.
    pub fn new() -> Self {
        Cache {
            safe: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set a key-value pair in the cache. Returns the old value if a value
    /// already existed for the given key. The lifetime is the duration of
    /// storage inside the Cache. Passed it the data will be deleted.
    pub fn set(
        &self,
        key: &str,
        value: &str,
        lifetime: Duration,
    ) -> Option<String> {
        let safe = Arc::clone(&self.safe);
        let mut map = safe.lock().unwrap();
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
    pub fn get(&self, key: &str) -> Option<String> {
        let safe = Arc::clone(&self.safe);
        let mut map = safe.lock().unwrap();
        match map.get(key) {
            Some(item) => {
                if item.is_expired() {
                    map.remove(key);
                    None
                } else {
                    Some(item.value.to_string())
                }
            }
            None => None,
        }
    }

    /// Delete a value from the Cache. Returns the old value if it was still
    /// valid. So this method can basically be used as a 'pop' function.
    pub fn del(&self, key: &str) -> Option<String> {
        let safe = Arc::clone(&self.safe);
        let mut map = safe.lock().unwrap();
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
        let safe = Arc::clone(&self.safe);
        let map = safe.lock().unwrap();
		map.contains_key(key)
	}
}
