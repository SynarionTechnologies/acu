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
        s3::S3BlobStore::put(self, key, data)
    }
    fn get(&self, key: &str) -> Result<Vec<u8>, BlobError> {
        s3::S3BlobStore::get(self, key)
    }
    fn delete(&self, key: &str) -> Result<(), BlobError> {
        s3::S3BlobStore::delete(self, key)
    }
    fn list(&self) -> Result<Vec<String>, BlobError> {
        s3::S3BlobStore::list(self)
    }
    fn exists(&self, key: &str) -> bool {
        s3::S3BlobStore::exists(self, key)
    }
}
