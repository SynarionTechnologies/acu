use serde::{Deserialize, Serialize};

use super::EventMeta;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryItemAppended {
    pub meta: EventMeta,
    pub item: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryItemAccessed {
    pub meta: EventMeta,
    pub item: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryItemUpdated {
    pub meta: EventMeta,
    pub item: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryItemDeleted {
    pub meta: EventMeta,
    pub item: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryItemArchived {
    pub meta: EventMeta,
    pub item: String,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryCompacted {
    pub meta: EventMeta,
    pub store: String,
    pub freed_space_bytes: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryRetentionPolicyChanged {
    pub meta: EventMeta,
    pub old: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MemoryReindexed {
    pub meta: EventMeta,
}

