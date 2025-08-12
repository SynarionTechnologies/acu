//! Snapshot utilities for projections and ANN indexes.

pub mod errors;
pub mod integrity;
pub mod rotation;

use chrono::Utc;
use errors::SnapshotError;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub use integrity::hash_dir;
pub use rotation::rotate_snapshots;

/// Create a snapshot of `src` inside `dest_root` and return the snapshot path.
pub fn create_snapshot(src: &Path, dest_root: &Path) -> Result<PathBuf, SnapshotError> {
    fs::create_dir_all(dest_root)?;
    let name = Utc::now().timestamp_nanos_opt().unwrap().to_string();
    let dest = dest_root.join(name);
    fs::create_dir(&dest)?;
    for entry in WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        let rel = p.strip_prefix(src).unwrap();
        let target = dest.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(p, target)?;
        }
    }
    Ok(dest)
}

/// Verify a snapshot by hashing its contents.
pub fn verify_snapshot(path: &Path) -> Result<String, SnapshotError> {
    Ok(integrity::hash_dir(path)?)
}

/// Restore a snapshot by copying it over `dest`.
pub fn restore_snapshot(snapshot: &Path, dest: &Path) -> Result<(), SnapshotError> {
    if dest.exists() {
        fs::remove_dir_all(dest)?;
    }
    fs::create_dir_all(dest)?;
    for entry in WalkDir::new(snapshot).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        let rel = p.strip_prefix(snapshot).unwrap();
        let target = dest.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(p, target)?;
        }
    }
    Ok(())
}
