//! Blob store implementations.

mod errors;
pub use errors::BlobError;

pub mod fs;
#[cfg(feature = "s3")]
pub mod s3;

/// Trait representing basic blob operations.
pub trait BlobStore {
    fn put(&self, key: &str, data: &[u8]) -> Result<String, BlobError>;
    fn get(&self, key: &str) -> Result<Vec<u8>, BlobError>;
    fn delete(&self, key: &str) -> Result<(), BlobError>;
    fn list(&self) -> Result<Vec<String>, BlobError>;
    fn exists(&self, key: &str) -> bool;
}

impl BlobStore for fs::FsBlobStore {
    fn put(&self, key: &str, data: &[u8]) -> Result<String, BlobError> {
        fs::FsBlobStore::put(self, key, data)
    }
    fn get(&self, key: &str) -> Result<Vec<u8>, BlobError> {
        fs::FsBlobStore::get(self, key)
    }
    fn delete(&self, key: &str) -> Result<(), BlobError> {
        fs::FsBlobStore::delete(self, key)
    }
    fn list(&self) -> Result<Vec<String>, BlobError> {
        fs::FsBlobStore::list(self)
    }
    fn exists(&self, key: &str) -> bool {
        fs::FsBlobStore::exists(self, key)
    }
}

#[cfg(feature = "s3")]
impl BlobStore for s3::S3BlobStore {
    fn put(&self, key: &str, data: &[u8]) -> Result<String, BlobError> {
        self.put(key, data)
    }
    fn get(&self, key: &str) -> Result<Vec<u8>, BlobError> {
        self.get(key)
    }
    fn delete(&self, key: &str) -> Result<(), BlobError> {
        self.delete(key)
    }
    fn list(&self) -> Result<Vec<String>, BlobError> {
        self.list()
    }
    fn exists(&self, key: &str) -> bool {
        self.exists(key)
    }
}
