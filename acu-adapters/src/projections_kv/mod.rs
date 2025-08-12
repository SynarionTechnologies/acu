//! Key/value stores for projections.

mod errors;
pub use errors::KvError;

#[cfg(feature = "rocksdb")]
pub mod rocks;
pub mod sqlite;

/// Basic operations for a key/value store.
pub trait KvStore {
    fn put(&self, table: &str, key: &str, value: &[u8]) -> Result<(), KvError>;
    fn get(&self, table: &str, key: &str) -> Result<Option<Vec<u8>>, KvError>;
    fn delete(&self, table: &str, key: &str) -> Result<(), KvError>;
    fn scan_prefix(&self, table: &str, prefix: &str) -> Result<Vec<(String, Vec<u8>)>, KvError>;
}

#[cfg(feature = "rocksdb")]
pub use rocks::RocksStore;
pub use sqlite::SqliteStore;
