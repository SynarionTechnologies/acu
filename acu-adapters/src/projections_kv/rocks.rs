#[cfg(feature = "rocksdb")]
use super::{KvError, KvStore};
#[cfg(feature = "rocksdb")]
use rocksdb::{IteratorMode, Options, DB};
#[cfg(feature = "rocksdb")]
use std::path::Path;

#[cfg(feature = "rocksdb")]
/// RocksDB implementation of [`KvStore`].
pub struct RocksStore {
    db: DB,
}

#[cfg(feature = "rocksdb")]
impl RocksStore {
    /// Open a new RocksDB store.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, KvError> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self { db })
    }
}

#[cfg(feature = "rocksdb")]
impl KvStore for RocksStore {
    fn put(&self, table: &str, key: &str, value: &[u8]) -> Result<(), KvError> {
        let k = format!("{}:{}", table, key);
        self.db.put(k.as_bytes(), value)?;
        Ok(())
    }

    fn get(&self, table: &str, key: &str) -> Result<Option<Vec<u8>>, KvError> {
        let k = format!("{}:{}", table, key);
        Ok(self.db.get(k.as_bytes())?)
    }

    fn delete(&self, table: &str, key: &str) -> Result<(), KvError> {
        let k = format!("{}:{}", table, key);
        self.db.delete(k.as_bytes())?;
        Ok(())
    }

    fn scan_prefix(&self, table: &str, prefix: &str) -> Result<Vec<(String, Vec<u8>)>, KvError> {
        let start = format!("{}:{}", table, prefix);
        let mode = IteratorMode::From(start.as_bytes(), rocksdb::Direction::Forward);
        let mut out = Vec::new();
        for item in self.db.iterator(mode) {
            let (k, v) = item?;
            let key_str = String::from_utf8(k.to_vec()).unwrap();
            if !key_str.starts_with(&format!("{}:{}", table, prefix)) {
                break;
            }
            let user_key = key_str.split(':').nth(1).unwrap_or("").to_string();
            out.push((user_key, v.to_vec()));
        }
        Ok(out)
    }
}
