use serde::{Deserialize, Serialize};

use super::EventMeta;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TrainingSessionStarted {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TrainingSessionStopped {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TrainingCheckpointCreated {
    pub meta: EventMeta,
    pub checkpoint: String,
}

use ordered_float::OrderedFloat;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RewardObserved {
    pub meta: EventMeta,
    pub reward: OrderedFloat<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PolicyAdjustedFromReward {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ModelWeightsUpdated {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ModelRolledBack {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExplorationStrategyChanged {
    pub meta: EventMeta,
    pub strategy: String,
}
