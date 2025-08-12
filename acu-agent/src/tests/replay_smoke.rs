use chrono::Utc;
use uuid::Uuid;

use crate::events::{
    curiosity::SourceAddedToWhitelist,
    learning::RewardObserved,
    profile::{PolicyUpdated, ProfileCreated},
    AcuEvent, EventMeta,
};
use ordered_float::OrderedFloat;

#[derive(Default)]
struct AgentProfileAgg {
    name: Option<String>,
    policies: Vec<String>,
    whitelist: Vec<String>,
}

impl AgentProfileAgg {
    fn apply(&mut self, event: &AcuEvent) {
        match event {
            AcuEvent::ProfileCreated(e) => self.name = Some(e.name.clone()),
            AcuEvent::PolicyUpdated(e) => self.policies.push(e.policy.clone()),
            AcuEvent::SourceAddedToWhitelist(e) => self.whitelist.push(e.source.clone()),
            _ => {}
        }
    }
}

#[derive(Default)]
struct RewardStats {
    count: usize,
    sum: f32,
}

impl RewardStats {
    fn apply(&mut self, event: &AcuEvent) {
        if let AcuEvent::RewardObserved(e) = event {
            self.count += 1;
            self.sum += e.reward.into_inner();
        }
    }
    fn mean(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f32
        }
    }
}

#[test]
fn replay_smoke() {
    let meta = EventMeta::new(Uuid::new_v4(), Uuid::new_v4(), Utc::now(), None);
    let events = vec![
        AcuEvent::ProfileCreated(ProfileCreated {
            meta: meta.clone(),
            name: "Alice".into(),
        }),
        AcuEvent::PolicyUpdated(PolicyUpdated {
            meta: meta.clone(),
            policy: "p1".into(),
        }),
        AcuEvent::SourceAddedToWhitelist(SourceAddedToWhitelist {
            meta: meta.clone(),
            source: "https://example.com".into(),
        }),
        AcuEvent::RewardObserved(RewardObserved {
            meta: meta.clone(),
            reward: OrderedFloat(0.8),
        }),
    ];

    let mut agg = AgentProfileAgg::default();
    let mut stats = RewardStats::default();

    for e in &events {
        agg.apply(e);
        stats.apply(e);
    }

    assert_eq!(agg.name, Some("Alice".to_string()));
    assert_eq!(agg.policies, vec!["p1".to_string()]);
    assert_eq!(agg.whitelist, vec!["https://example.com".to_string()]);
    assert!((stats.mean() - 0.8).abs() < f32::EPSILON);
}
