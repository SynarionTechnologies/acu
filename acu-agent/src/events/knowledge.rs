use serde::{Deserialize, Serialize};

use super::EventMeta;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KnowledgeModuleEnabled {
    pub meta: EventMeta,
    pub module: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KnowledgeModuleDisabled {
    pub meta: EventMeta,
    pub module: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KnowledgeItemAdded {
    pub meta: EventMeta,
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KnowledgeItemUpdated {
    pub meta: EventMeta,
    pub item_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KnowledgeItemRemoved {
    pub meta: EventMeta,
    pub item_id: String,
}
