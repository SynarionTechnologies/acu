use super::errors::AnnError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct AnnData {
    dims: usize,
    vectors: HashMap<u64, Vec<f32>>,
}

/// Naive in-memory ANN index persisted to disk.
pub struct AnnIndex {
    data: AnnData,
    path: PathBuf,
}

impl AnnIndex {
    /// Create a new empty index.
    pub fn new(path: impl AsRef<Path>, dims: usize) -> Self {
        Self {
            data: AnnData {
                dims,
                vectors: HashMap::new(),
            },
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Insert or replace a vector by id.
    pub fn insert(&mut self, id: u64, embedding: Vec<f32>) {
        self.data.vectors.insert(id, embedding);
    }

    /// Remove a vector.
    pub fn remove(&mut self, id: u64) {
        self.data.vectors.remove(&id);
    }

    /// Search the `k` nearest vectors using cosine similarity.
    pub fn search(&self, query: &[f32], k: usize) -> Vec<(u64, f32)> {
        let mut scores: Vec<(u64, f32)> = self
            .data
            .vectors
            .iter()
            .map(|(id, v)| (*id, cosine_similarity(query, v)))
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores.into_iter().take(k).collect()
    }

    /// Persist the index to the configured path.
    pub fn persist(&self) -> Result<(), AnnError> {
        let bytes = bincode::serialize(&self.data)?;
        let mut file = File::create(&self.path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    /// Load an index from disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, AnnError> {
        let mut file = File::open(&path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let data: AnnData = bincode::deserialize(&buf)?;
        Ok(Self {
            data,
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Create a snapshot file at `snapshot_path`.
    pub fn snapshot(&self, snapshot_path: impl AsRef<Path>) -> Result<String, AnnError> {
        let bytes = bincode::serialize(&self.data)?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = format!("{:x}", hasher.finalize());
        let mut file = File::create(snapshot_path)?;
        file.write_all(&bytes)?;
        Ok(hash)
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum();
    let nb: f32 = b.iter().map(|x| x * x).sum();
    dot / (na.sqrt() * nb.sqrt())
}
