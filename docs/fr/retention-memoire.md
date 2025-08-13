# Rétention et compaction de la mémoire

ACU stocke les expériences dans des couches chaude, tiède et froide. Les politiques de rétention décident quand les éléments sont conservés, archivés ou supprimés.

## Score de rétention

```
score = w_recency * recency_factor
      + w_reward * normalized_reward
      + w_reference_count * normalized_ref_count
      + w_uniqueness * uniqueness_factor
```

Les éléments plus anciens que le TTL du canal sont supprimés lorsque le score est inférieur à `purge_threshold`; sinon ils sont archivés vers la couche froide.

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

## Exécution manuelle

```rust
let engine = DefaultRetentionEngine::new(config, store);
let report = engine.apply_policies()?;
println!("Supprimés: {}, Archivés: {}", report.deleted, report.archived);
```

La compaction libère de l'espace disque et fait tourner les instantanés après l'application des politiques.
