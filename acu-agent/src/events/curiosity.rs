use serde::{Deserialize, Serialize};

use super::EventMeta;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CuriosityExplorationStarted {
    pub meta: EventMeta,
    pub query: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CuriosityExplorationCompleted {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExplorationAborted {
    pub meta: EventMeta,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExplorationResultValidated {
    pub meta: EventMeta,
    pub result_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceAddedToWhitelist {
    pub meta: EventMeta,
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SourceAddedToBlacklist {
    pub meta: EventMeta,
    pub source: String,
}
