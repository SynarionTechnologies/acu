use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Compute a recursive hash of a directory.
pub fn hash_dir(path: &Path) -> Result<String, std::io::Error> {
    let mut entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|e| e.path().to_path_buf());
    let mut hasher = Sha256::new();
    for entry in entries {
        if entry.file_type().is_file() {
            let rel = entry.path().strip_prefix(path).unwrap();
            hasher.update(rel.to_string_lossy().as_bytes());
            let data = fs::read(entry.path())?;
            hasher.update(&data);
        }
    }
    Ok(format!("{:x}", hasher.finalize()))
}
