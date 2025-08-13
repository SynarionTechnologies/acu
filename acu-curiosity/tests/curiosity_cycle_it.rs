use acu_curiosity::config::CuriosityConfig;
use acu_curiosity::{CuriosityEngine, DefaultCuriosityEngine, ExplorationPlan};

#[test]
fn curiosity_cycle() {
    let config = CuriosityConfig::default();
    let mut engine = DefaultCuriosityEngine::new(config);
    let plan = ExplorationPlan {
        strategy: "web_crawl".to_string(),
        target: "https://example.com".to_string(),
    };
    let id = engine.start_exploration(plan).unwrap();
    engine.tick().unwrap();
    let report = engine.analyze_results(&id).unwrap();
    assert!(report.facts.contains(&"crawled".to_string()));
}
