use super::errors::BlobError;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Filesystem based blob store.
pub struct FsBlobStore {
    root: PathBuf,
}

impl FsBlobStore {
    /// Create a new store at the given root path.
    pub fn new(root: impl AsRef<Path>) -> Result<Self, BlobError> {
        fs::create_dir_all(root.as_ref())?;
        Ok(Self {
            root: root.as_ref().to_path_buf(),
        })
    }

    fn blob_path(&self, key: &str) -> PathBuf {
        self.root.join(key)
    }

    /// Store a blob and return its hash.
    pub fn put(&self, key: &str, data: &[u8]) -> Result<String, BlobError> {
        let path = self.blob_path(key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, data)?;
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Retrieve a blob.
    pub fn get(&self, key: &str) -> Result<Vec<u8>, BlobError> {
        let path = self.blob_path(key);
        Ok(fs::read(path)?)
    }

    /// Delete a blob.
    pub fn delete(&self, key: &str) -> Result<(), BlobError> {
        let path = self.blob_path(key);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// List all keys.
    pub fn list(&self) -> Result<Vec<String>, BlobError> {
        let mut keys = Vec::new();
        for entry in WalkDir::new(&self.root).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                let rel = entry
                    .path()
                    .strip_prefix(&self.root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                keys.push(rel);
            }
        }
        Ok(keys)
    }

    /// Check whether a blob exists.
    pub fn exists(&self, key: &str) -> bool {
        self.blob_path(key).exists()
    }
}
