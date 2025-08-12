use thiserror::Error;

/// Errors for projection key/value stores.
#[derive(Debug, Error)]
pub enum KvError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[cfg(feature = "rocksdb")]
    #[error("RocksDB error: {0}")]
    Rocks(#[from] rocksdb::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
