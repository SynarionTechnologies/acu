use thiserror::Error;

/// Errors for snapshot utilities.
#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
