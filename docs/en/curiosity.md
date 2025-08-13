# Curiosity Engine

The Curiosity Engine orchestrates autonomous exploration cycles.

1. Plan explorations with strategies like web crawling or API queries.
2. Execute the plan and collect logs.
3. Analyze results to extract facts and embeddings.
4. Update memory stores and adjust future strategies.

```rust
use acu_curiosity::config::CuriosityConfig;
use acu_curiosity::{CuriosityEngine, DefaultCuriosityEngine, ExplorationPlan};

let config = CuriosityConfig::default();
let mut engine = DefaultCuriosityEngine::new(config);
let plan = ExplorationPlan { strategy: "web_crawl".into(), target: "https://example.com".into() };
let id = engine.start_exploration(plan)?;
engine.tick()?;
let report = engine.analyze_results(&id)?;
# Ok::<(), acu_curiosity::CuriosityError>(())
```
