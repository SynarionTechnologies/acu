use super::*;
use crate::config::{RetentionConfig, RetentionWeights};
use chrono::Utc;

struct TestStore {
    items: Vec<MemoryItem>,
}

impl MemoryStore for TestStore {
    fn list_items(&self) -> Vec<MemoryItem> {
        self.items.clone()
    }
    fn delete(&mut self, id: &str) -> Result<(), String> {
        self.items.retain(|i| i.id != id);
        Ok(())
    }
    fn archive(&mut self, id: &str) -> Result<(), String> {
        self.items.retain(|i| i.id != id);
        Ok(())
    }
    fn compact(&mut self) -> Result<u64, String> {
        Ok(42)
    }
}

fn sample_config() -> RetentionConfig {
    RetentionConfig {
        episodic_ttl_days: 60,
        tool_logs_ttl_days: 14,
        purge_threshold: 0.2,
        weights: RetentionWeights {
            recency: 0.5,
            reward: 0.3,
            reference_count: 0.1,
            uniqueness: 0.1,
        },
    }
}

fn sample_item(id: &str, days_old: i64, score: f32) -> MemoryItem {
    let now = Utc::now();
    MemoryItem {
        id: id.into(),
        channel: MemoryChannel::Episodic,
        created_at: now - chrono::Duration::days(days_old),
        last_accessed_at: now - chrono::Duration::days(days_old),
        reward: score,
        reference_count: 0,
        uniqueness: 0.5,
    }
}

#[test]
fn computes_score_within_bounds() {
    let cfg = sample_config();
    let store = TestStore { items: vec![] };
    let engine = DefaultRetentionEngine::new(cfg, store);
    let item = sample_item("1", 10, 0.5);
    let s = engine.score(&item, Utc::now());
    assert!((0.0..=1.0).contains(&s));
}

#[test]
fn deletes_and_archives_items() {
    let cfg = sample_config();
    let items = vec![
        sample_item("d", 100, 0.0), // delete
        sample_item("a", 100, 1.0), // archive
        sample_item("k", 5, 0.0),   // keep
    ];
    let store = TestStore { items };
    let mut engine = DefaultRetentionEngine::new(cfg, store);
    let report = engine.apply_policies().unwrap();
    assert_eq!(report.deleted, 1);
    assert_eq!(report.archived, 1);
    assert_eq!(report.kept, 1);
    assert_eq!(engine.events().len(), 2);
}

#[test]
fn compaction_emits_event() {
    let cfg = sample_config();
    let store = TestStore { items: vec![] };
    let mut engine = DefaultRetentionEngine::new(cfg, store);
    let report = engine.compact().unwrap();
    assert_eq!(report.freed_space_bytes, 42);
    assert_eq!(engine.events().len(), 1);
}
