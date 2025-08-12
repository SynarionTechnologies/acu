#[cfg(feature = "s3")]
use super::errors::BlobError;

#[cfg(feature = "s3")]
/// Placeholder S3 store. Real implementation should wrap an S3 client.
pub struct S3BlobStore;

#[cfg(feature = "s3")]
impl S3BlobStore {
    pub fn new() -> Self {
        S3BlobStore
    }

    pub fn put(&self, _key: &str, _data: &[u8]) -> Result<String, BlobError> {
        Err(BlobError::S3("not implemented".into()))
    }
    pub fn get(&self, _key: &str) -> Result<Vec<u8>, BlobError> {
        Err(BlobError::S3("not implemented".into()))
    }
    pub fn delete(&self, _key: &str) -> Result<(), BlobError> {
        Err(BlobError::S3("not implemented".into()))
    }
    pub fn list(&self) -> Result<Vec<String>, BlobError> {
        Err(BlobError::S3("not implemented".into()))
    }
    pub fn exists(&self, _key: &str) -> bool {
        false
    }
}
