//! Configuration structures for memory adapters.
//!
//! These structures are usually loaded from a TOML file. Example:
//!
//! ```toml
//! [memory.projections]
//! engine = "sqlite"
//! path = "/var/lib/acu/projections/acu.db"
//! ```

use serde::Deserialize;
use std::path::PathBuf;

/// Top level configuration wrapper.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub memory: MemoryConfig,
}

/// Memory related configuration.
#[derive(Debug, Deserialize)]
pub struct MemoryConfig {
    #[serde(default)]
    pub projections: ProjectionConfig,
    #[serde(default)]
    pub ann: AnnConfig,
    #[serde(default)]
    pub blobs: BlobConfig,
}

/// Configuration for projection key/value store.
#[derive(Debug, Deserialize)]
pub struct ProjectionConfig {
    #[serde(default = "default_engine")]
    pub engine: String,
    #[serde(default = "default_proj_path")]
    pub path: PathBuf,
}

fn default_engine() -> String {
    "sqlite".to_string()
}

fn default_proj_path() -> PathBuf {
    "/var/lib/acu/projections/acu.db".into()
}

/// Configuration for ANN index.
#[derive(Debug, Deserialize)]
pub struct AnnConfig {
    #[serde(default = "default_ann_path")]
    pub path: PathBuf,
    #[serde(default = "default_dims")]
    pub dims: usize,
    #[serde(default = "default_m")]
    pub m: usize,
    #[serde(default = "default_ef_construct")]
    pub ef_construction: usize,
    #[serde(default = "default_ef_search")]
    pub ef_search: usize,
    #[serde(default = "default_ann_snap_path")]
    pub snapshot_path: PathBuf,
    #[serde(default = "default_snapshot_interval")]
    pub snapshot_interval_days: u64,
    #[serde(default = "default_keep_last")]
    pub keep_last: usize,
}

fn default_ann_path() -> PathBuf {
    "/var/lib/acu/ann/hnsw_main".into()
}
fn default_dims() -> usize {
    1536
}
fn default_m() -> usize {
    16
}
fn default_ef_construct() -> usize {
    200
}
fn default_ef_search() -> usize {
    64
}
fn default_ann_snap_path() -> PathBuf {
    "/var/lib/acu/ann/snapshots".into()
}
fn default_snapshot_interval() -> u64 {
    7
}
fn default_keep_last() -> usize {
    4
}

/// Configuration for blob storage.
#[derive(Debug, Deserialize)]
pub struct BlobConfig {
    #[serde(default = "default_kind")]
    pub kind: String,
    #[serde(default = "default_blob_root")]
    pub root: PathBuf,
    #[serde(default)]
    pub s3: Option<S3Config>,
}

fn default_kind() -> String {
    "fs".into()
}
fn default_blob_root() -> PathBuf {
    "/var/lib/acu/blobs".into()
}

/// S3 configuration used when `kind = "s3"`.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct S3Config {
    pub endpoint: Option<String>,
    pub bucket: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub region: Option<String>,
}

impl Config {
    /// Parse configuration from TOML string.
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
}

impl Default for ProjectionConfig {
    fn default() -> Self {
        Self {
            engine: default_engine(),
            path: default_proj_path(),
        }
    }
}

impl Default for AnnConfig {
    fn default() -> Self {
        Self {
            path: default_ann_path(),
            dims: default_dims(),
            m: default_m(),
            ef_construction: default_ef_construct(),
            ef_search: default_ef_search(),
            snapshot_path: default_ann_snap_path(),
            snapshot_interval_days: default_snapshot_interval(),
            keep_last: default_keep_last(),
        }
    }
}

impl Default for BlobConfig {
    fn default() -> Self {
        Self {
            kind: default_kind(),
            root: default_blob_root(),
            s3: None,
        }
    }
}
