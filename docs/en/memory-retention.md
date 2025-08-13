# Memory Retention & Compaction

ACU stores experiences in hot, warm and cold layers. Retention policies decide when items are kept, archived or deleted.

## Retention score

```
score = w_recency * recency_factor
      + w_reward * normalized_reward
      + w_reference_count * normalized_ref_count
      + w_uniqueness * uniqueness_factor
```

Items older than the channel TTL are removed when the score drops below `purge_threshold`; otherwise they are archived to the cold layer.

## Configuration

```toml
[memory.retention]
episodic_ttl_days = 60
tool_logs_ttl_days = 14
purge_threshold = 0.2

[memory.retention.weights]
recency = 0.5
reward = 0.3
reference_count = 0.1
uniqueness = 0.1
```

## Manual execution

```rust
let engine = DefaultRetentionEngine::new(config, store);
let report = engine.apply_policies()?;
println!("Deleted: {}, Archived: {}", report.deleted, report.archived);
```

Compaction frees disk space and rotates snapshots after applying the policies.
