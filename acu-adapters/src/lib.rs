//! Infrastructure adapters for ACU.
//!
//! # Examples
//! ```
//! use acu_adapters::observability::log_info;
//! log_info("hi");
//! ```

pub mod eventstore;
pub mod observability;
pub mod readstore;

pub mod ann_hnsw;
pub mod blobs;
pub mod config;
pub mod projections_kv;
pub mod snapshots;
