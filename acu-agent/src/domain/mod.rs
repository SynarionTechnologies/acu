//! Domain layer containing aggregates, entities and value objects.
//!
//! # Examples
//! ```
//! use acu_agent::domain::AgentProfileId;
//! let id = AgentProfileId::new();
//! assert_eq!(id.as_str().len(), 36);
//! ```

use uuid::Uuid;

/// Identifier for an agent profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentProfileId(Uuid);

impl AgentProfileId {
    /// Create a new random identifier.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Represent the identifier as a string.
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl Default for AgentProfileId {
    fn default() -> Self {
        Self::new()
    }
}

/// Placeholder aggregate representing an agent profile.
#[derive(Debug, Default)]
pub struct AgentProfile;
