//! Memory retention and compaction engine.

pub mod config;
pub mod engine;

pub use config::{RetentionConfig, RetentionWeights};
pub use engine::{CompactionReport, DefaultRetentionEngine, EngineError, MemoryChannel, MemoryItem, RetentionEngine, RetentionReport};
