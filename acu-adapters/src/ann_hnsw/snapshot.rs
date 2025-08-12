use super::{errors::AnnError, index::AnnIndex};
use std::fs;
use std::path::{Path, PathBuf};

/// Snapshot the ANN index into the given directory. Returns the snapshot file path and hash.
pub fn snapshot(index: &AnnIndex, dir: impl AsRef<Path>) -> Result<(PathBuf, String), AnnError> {
    fs::create_dir_all(&dir)?;
    let file = dir.as_ref().join("ann.snapshot");
    let hash = index.snapshot(&file)?;
    Ok((file, hash))
}

/// Restore an ANN index from the snapshot file.
pub fn restore(path: impl AsRef<Path>) -> Result<AnnIndex, AnnError> {
    AnnIndex::load(path)
}
