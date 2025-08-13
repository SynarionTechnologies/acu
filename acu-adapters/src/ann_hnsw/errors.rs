use thiserror::Error;

/// Errors for ANN index operations.
#[derive(Debug, Error)]
pub enum AnnError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] bincode::Error),
}
