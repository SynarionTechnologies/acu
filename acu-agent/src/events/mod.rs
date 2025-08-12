//! ACU domain events.
//!
//! Events capture every meaningful change in the agent. They are immutable,
//! timestamped and can be replayed to rebuild state.
//!
//! # Examples
//! ```
//! use chrono::Utc;
//! use uuid::Uuid;
//! use acu_agent::events::{AcuEvent, EventMeta, profile::{ProfileCreated, PolicyUpdated}, curiosity::SourceAddedToWhitelist};
//!
//! #[derive(Default)]
//! struct AgentProfileAgg {
//!     name: Option<String>,
//!     policies: Vec<String>,
//!     whitelist: Vec<String>,
//! }
//!
//! impl AgentProfileAgg {
//!     fn apply(&mut self, event: &AcuEvent) {
//!         match event {
//!             AcuEvent::ProfileCreated(e) => self.name = Some(e.name.clone()),
//!             AcuEvent::PolicyUpdated(e) => self.policies.push(e.policy.clone()),
//!             AcuEvent::SourceAddedToWhitelist(e) => self.whitelist.push(e.source.clone()),
//!             _ => {}
//!         }
//!     }
//! }
//!
//! let meta = EventMeta::new(Uuid::new_v4(), Uuid::new_v4(), Utc::now(), None);
//! let events = vec![
//!     AcuEvent::ProfileCreated(ProfileCreated { meta: meta.clone(), name: "Alice".into() }),
//!     AcuEvent::PolicyUpdated(PolicyUpdated { meta: meta.clone(), policy: "p1".into() }),
//!     AcuEvent::SourceAddedToWhitelist(SourceAddedToWhitelist { meta: meta.clone(), source: "https://example.com".into() }),
//! ];
//! let mut agg = AgentProfileAgg::default();
//! for e in &events { agg.apply(e); }
//! assert_eq!(agg.name, Some("Alice".to_string()));
//! assert_eq!(agg.policies, vec!["p1".to_string()]);
//! assert_eq!(agg.whitelist, vec!["https://example.com".to_string()]);
//! ```

pub mod curiosity;
pub mod integrations;
pub mod interaction;
pub mod knowledge;
pub mod learning;
pub mod memory;
pub mod meta;
pub mod profile;
pub mod system;

pub use meta::EventMeta;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum AcuEvent {
    ProfileCreated(profile::ProfileCreated),
    ProfileRenamed(profile::ProfileRenamed),
    PolicyUpdated(profile::PolicyUpdated),
    ConfigUpdated(profile::ConfigUpdated),
    AuthenticationKeyAdded(profile::AuthenticationKeyAdded),
    AuthenticationKeyRevoked(profile::AuthenticationKeyRevoked),
    UserMessageReceived(interaction::UserMessageReceived),
    UserCommandIssued(interaction::UserCommandIssued),
    ResponseGenerated(interaction::ResponseGenerated),
    ResponseRejected(interaction::ResponseRejected),
    ToolInvoked(interaction::ToolInvoked),
    ToolInvocationFailed(interaction::ToolInvocationFailed),
    ConversationStarted(interaction::ConversationStarted),
    ConversationEnded(interaction::ConversationEnded),
    CuriosityExplorationStarted(curiosity::CuriosityExplorationStarted),
    CuriosityExplorationCompleted(curiosity::CuriosityExplorationCompleted),
    ExplorationAborted(curiosity::ExplorationAborted),
    ExplorationResultValidated(curiosity::ExplorationResultValidated),
    SourceAddedToWhitelist(curiosity::SourceAddedToWhitelist),
    SourceAddedToBlacklist(curiosity::SourceAddedToBlacklist),
    KnowledgeModuleEnabled(knowledge::KnowledgeModuleEnabled),
    KnowledgeModuleDisabled(knowledge::KnowledgeModuleDisabled),
    KnowledgeItemAdded(knowledge::KnowledgeItemAdded),
    KnowledgeItemUpdated(knowledge::KnowledgeItemUpdated),
    KnowledgeItemRemoved(knowledge::KnowledgeItemRemoved),
    MemoryItemAppended(memory::MemoryItemAppended),
    MemoryItemAccessed(memory::MemoryItemAccessed),
    MemoryItemUpdated(memory::MemoryItemUpdated),
    MemoryItemDeleted(memory::MemoryItemDeleted),
    MemoryCompacted(memory::MemoryCompacted),
    MemoryRetentionPolicyChanged(memory::MemoryRetentionPolicyChanged),
    MemoryReindexed(memory::MemoryReindexed),
    TrainingSessionStarted(learning::TrainingSessionStarted),
    TrainingSessionStopped(learning::TrainingSessionStopped),
    TrainingCheckpointCreated(learning::TrainingCheckpointCreated),
    RewardObserved(learning::RewardObserved),
    PolicyAdjustedFromReward(learning::PolicyAdjustedFromReward),
    ModelWeightsUpdated(learning::ModelWeightsUpdated),
    ModelRolledBack(learning::ModelRolledBack),
    ExplorationStrategyChanged(learning::ExplorationStrategyChanged),
    AgentStarted(system::AgentStarted),
    AgentStopped(system::AgentStopped),
    AgentUpgraded(system::AgentUpgraded),
    BackupCreated(system::BackupCreated),
    BackupRestored(system::BackupRestored),
    StorageCompacted(system::StorageCompacted),
    ErrorLogged(system::ErrorLogged),
    AlertTriggered(system::AlertTriggered),
    SecurityIncidentDetected(system::SecurityIncidentDetected),
    SecurityIncidentResolved(system::SecurityIncidentResolved),
    ExternalApiCalled(integrations::ExternalApiCalled),
    ExternalApiFailed(integrations::ExternalApiFailed),
    WebhookReceived(integrations::WebhookReceived),
    WebhookSent(integrations::WebhookSent),
    IntegrationEnabled(integrations::IntegrationEnabled),
    IntegrationDisabled(integrations::IntegrationDisabled),
}

impl AcuEvent {
    /// Version of the event schema.
    pub fn version(&self) -> u16 {
        1
    }

    /// Returns the name of the event kind.
    pub fn kind(&self) -> &'static str {
        match self {
            AcuEvent::ProfileCreated(_) => "ProfileCreated",
            AcuEvent::ProfileRenamed(_) => "ProfileRenamed",
            AcuEvent::PolicyUpdated(_) => "PolicyUpdated",
            AcuEvent::ConfigUpdated(_) => "ConfigUpdated",
            AcuEvent::AuthenticationKeyAdded(_) => "AuthenticationKeyAdded",
            AcuEvent::AuthenticationKeyRevoked(_) => "AuthenticationKeyRevoked",
            AcuEvent::UserMessageReceived(_) => "UserMessageReceived",
            AcuEvent::UserCommandIssued(_) => "UserCommandIssued",
            AcuEvent::ResponseGenerated(_) => "ResponseGenerated",
            AcuEvent::ResponseRejected(_) => "ResponseRejected",
            AcuEvent::ToolInvoked(_) => "ToolInvoked",
            AcuEvent::ToolInvocationFailed(_) => "ToolInvocationFailed",
            AcuEvent::ConversationStarted(_) => "ConversationStarted",
            AcuEvent::ConversationEnded(_) => "ConversationEnded",
            AcuEvent::CuriosityExplorationStarted(_) => "CuriosityExplorationStarted",
            AcuEvent::CuriosityExplorationCompleted(_) => "CuriosityExplorationCompleted",
            AcuEvent::ExplorationAborted(_) => "ExplorationAborted",
            AcuEvent::ExplorationResultValidated(_) => "ExplorationResultValidated",
            AcuEvent::SourceAddedToWhitelist(_) => "SourceAddedToWhitelist",
            AcuEvent::SourceAddedToBlacklist(_) => "SourceAddedToBlacklist",
            AcuEvent::KnowledgeModuleEnabled(_) => "KnowledgeModuleEnabled",
            AcuEvent::KnowledgeModuleDisabled(_) => "KnowledgeModuleDisabled",
            AcuEvent::KnowledgeItemAdded(_) => "KnowledgeItemAdded",
            AcuEvent::KnowledgeItemUpdated(_) => "KnowledgeItemUpdated",
            AcuEvent::KnowledgeItemRemoved(_) => "KnowledgeItemRemoved",
            AcuEvent::MemoryItemAppended(_) => "MemoryItemAppended",
            AcuEvent::MemoryItemAccessed(_) => "MemoryItemAccessed",
            AcuEvent::MemoryItemUpdated(_) => "MemoryItemUpdated",
            AcuEvent::MemoryItemDeleted(_) => "MemoryItemDeleted",
            AcuEvent::MemoryCompacted(_) => "MemoryCompacted",
            AcuEvent::MemoryRetentionPolicyChanged(_) => "MemoryRetentionPolicyChanged",
            AcuEvent::MemoryReindexed(_) => "MemoryReindexed",
            AcuEvent::TrainingSessionStarted(_) => "TrainingSessionStarted",
            AcuEvent::TrainingSessionStopped(_) => "TrainingSessionStopped",
            AcuEvent::TrainingCheckpointCreated(_) => "TrainingCheckpointCreated",
            AcuEvent::RewardObserved(_) => "RewardObserved",
            AcuEvent::PolicyAdjustedFromReward(_) => "PolicyAdjustedFromReward",
            AcuEvent::ModelWeightsUpdated(_) => "ModelWeightsUpdated",
            AcuEvent::ModelRolledBack(_) => "ModelRolledBack",
            AcuEvent::ExplorationStrategyChanged(_) => "ExplorationStrategyChanged",
            AcuEvent::AgentStarted(_) => "AgentStarted",
            AcuEvent::AgentStopped(_) => "AgentStopped",
            AcuEvent::AgentUpgraded(_) => "AgentUpgraded",
            AcuEvent::BackupCreated(_) => "BackupCreated",
            AcuEvent::BackupRestored(_) => "BackupRestored",
            AcuEvent::StorageCompacted(_) => "StorageCompacted",
            AcuEvent::ErrorLogged(_) => "ErrorLogged",
            AcuEvent::AlertTriggered(_) => "AlertTriggered",
            AcuEvent::SecurityIncidentDetected(_) => "SecurityIncidentDetected",
            AcuEvent::SecurityIncidentResolved(_) => "SecurityIncidentResolved",
            AcuEvent::ExternalApiCalled(_) => "ExternalApiCalled",
            AcuEvent::ExternalApiFailed(_) => "ExternalApiFailed",
            AcuEvent::WebhookReceived(_) => "WebhookReceived",
            AcuEvent::WebhookSent(_) => "WebhookSent",
            AcuEvent::IntegrationEnabled(_) => "IntegrationEnabled",
            AcuEvent::IntegrationDisabled(_) => "IntegrationDisabled",
        }
    }
}
