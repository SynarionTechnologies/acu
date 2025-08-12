//! Read models derived from events.
//!
//! # Examples
//! ```
//! use acu_agent::projections::ProfileView;
//! let view = ProfileView { id: "id".to_string() };
//! assert_eq!(view.id, "id");
//! ```

/// Projection of a profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileView {
    /// Identifier of the profile.
    pub id: String,
}
