# Moteur de curiosité

Le moteur de curiosité orchestre des cycles d'exploration autonome.

1. Planifier des explorations avec des stratégies comme le web crawl ou les requêtes API.
2. Exécuter le plan et collecter des journaux.
3. Analyser les résultats pour extraire des faits et des embeddings.
4. Mettre à jour la mémoire et ajuster les stratégies futures.

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
