//! In-memory read store stub.
//!
//! # Examples
//! ```
//! use acu_adapters::readstore::MemoryReadStore;
//! let mut store = MemoryReadStore::default();
//! store.insert("key".into(), "value".into());
//! assert_eq!(store.get("key"), Some(&"value".to_string()));
//! ```

use std::collections::HashMap;

/// Simple in-memory read store.
#[derive(Default)]
pub struct MemoryReadStore {
    items: HashMap<String, String>,
}

impl MemoryReadStore {
    /// Insert an item.
    pub fn insert(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    /// Retrieve an item.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.items.get(key)
    }
}
