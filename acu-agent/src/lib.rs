//! Core agent library exposing domain, application, events and projections.
//!
//! # Examples
//! ```
//! use acu_agent::domain::AgentProfileId;
//! let _id = AgentProfileId::new();
//! ```

pub mod application;
pub mod domain;
pub mod events;
pub mod projections;

#[cfg(test)]
mod tests;
