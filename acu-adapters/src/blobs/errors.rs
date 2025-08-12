use thiserror::Error;

/// Errors for blob store operations.
#[derive(Debug, Error)]
pub enum BlobError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "s3")]
    #[error("S3 error: {0}")]
    S3(String),
}
