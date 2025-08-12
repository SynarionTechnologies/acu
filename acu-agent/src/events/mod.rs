//! Domain events for ACU.
//!
//! # Examples
//! ```
//! use acu_agent::events::ProfileCreated;
//! let evt = ProfileCreated { id: "id".to_string() };
//! assert_eq!(evt.id, "id");
//! ```

/// Event emitted when a profile is created.
#[derive(Debug, Clone)]
pub struct ProfileCreated {
    /// Identifier of the new profile.
    pub id: String,
}
