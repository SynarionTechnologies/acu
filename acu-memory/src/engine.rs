use chrono::{DateTime, Utc};
use uuid::Uuid;
use acu_agent::events::{AcuEvent, EventMeta, memory::{MemoryCompacted, MemoryItemArchived, MemoryItemDeleted}};

use crate::config::RetentionConfig;

/// Channel of memory item.
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryChannel {
    Episodic,
    ToolLogs,
    Semantic,
    Knowledge,
}

/// Metadata describing a memory item for retention evaluation.
#[derive(Debug, Clone)]
pub struct MemoryItem {
    pub id: String,
    pub channel: MemoryChannel,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub reward: f32,
    pub reference_count: u32,
    pub uniqueness: f32,
}

/// Actions decided by the retention engine.
#[derive(Debug, Default)]
pub struct RetentionReport {
    pub deleted: usize,
    pub archived: usize,
    pub kept: usize,
}

/// Report from compaction phase.
#[derive(Debug, Default)]
pub struct CompactionReport {
    pub freed_space_bytes: u64,
}

/// Error type for the retention engine.
#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("store error: {0}")]
    Store(String),
}

/// Abstraction over underlying memory store.
pub trait MemoryStore {
    fn list_items(&self) -> Vec<MemoryItem>;
    fn delete(&mut self, id: &str) -> Result<(), String>;
    fn archive(&mut self, id: &str) -> Result<(), String>;
    fn compact(&mut self) -> Result<u64, String>;
}

/// Engine applying retention policies.
pub trait RetentionEngine {
    fn apply_policies(&mut self) -> Result<RetentionReport, EngineError>;
    fn compact(&mut self) -> Result<CompactionReport, EngineError>;
    fn events(&self) -> &[AcuEvent];
}

/// Default implementation of the retention engine.
pub struct DefaultRetentionEngine<S: MemoryStore> {
    config: RetentionConfig,
    store: S,
    events: Vec<AcuEvent>,
}

impl<S: MemoryStore> DefaultRetentionEngine<S> {
    pub fn new(config: RetentionConfig, store: S) -> Self {
        Self { config, store, events: Vec::new() }
    }

    fn new_meta() -> EventMeta {
        EventMeta::new(Uuid::new_v4(), Uuid::new_v4(), Utc::now(), None)
    }

    fn score(&self, item: &MemoryItem, now: DateTime<Utc>) -> f32 {
        let ttl_days = self.config.ttl_for_channel(&item.channel) as f32;
        let age_days = (now - item.last_accessed_at).num_days() as f32;
        let recency_factor = (-(age_days / ttl_days.max(1.0))).exp();
        let normalized_reward = item.reward.clamp(0.0, 1.0);
        let normalized_ref_count = (item.reference_count as f32
            / (1.0 + item.reference_count as f32))
            .clamp(0.0, 1.0);
        let uniqueness_factor = item.uniqueness.clamp(0.0, 1.0);
        let w = &self.config.weights;
        let score = w.recency * recency_factor
            + w.reward * normalized_reward
            + w.reference_count * normalized_ref_count
            + w.uniqueness * uniqueness_factor;
        score.clamp(0.0, 1.0)
    }
}

impl<S: MemoryStore> RetentionEngine for DefaultRetentionEngine<S> {
    fn apply_policies(&mut self) -> Result<RetentionReport, EngineError> {
        let now = Utc::now();
        let mut report = RetentionReport::default();
        let items = self.store.list_items();
        for item in items {
            let ttl = self.config.ttl_for_channel(&item.channel);
            let age_days = (now - item.created_at).num_days();
            if age_days <= ttl {
                report.kept += 1;
                continue;
            }
            let score = self.score(&item, now);
            if score < self.config.purge_threshold {
                self.store
                    .delete(&item.id)
                    .map_err(EngineError::Store)?;
                self.events.push(AcuEvent::MemoryItemDeleted(MemoryItemDeleted {
                    meta: Self::new_meta(),
                    item: item.id.clone(),
                    reason: "expired".into(),
                }));
                report.deleted += 1;
            } else {
                self.store
                    .archive(&item.id)
                    .map_err(EngineError::Store)?;
                self.events.push(AcuEvent::MemoryItemArchived(MemoryItemArchived {
                    meta: Self::new_meta(),
                    item: item.id.clone(),
                    destination: "cold".into(),
                }));
                report.archived += 1;
            }
        }
        Ok(report)
    }

    fn compact(&mut self) -> Result<CompactionReport, EngineError> {
        let freed = self.store.compact().map_err(EngineError::Store)?;
        self.events.push(AcuEvent::MemoryCompacted(MemoryCompacted {
            meta: Self::new_meta(),
            store: "kv".into(),
            freed_space_bytes: freed,
        }));
        Ok(CompactionReport { freed_space_bytes: freed })
    }

    fn events(&self) -> &[AcuEvent] {
        &self.events
    }
}

#[cfg(test)]
mod tests;
