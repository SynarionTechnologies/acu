//! Infrastructure adapters for ACU.
//!
//! # Examples
//! ```
//! use acu_adapters::observability::log_info;
//! log_info("hi");
//! ```

pub mod eventstore;
pub mod observability;
pub mod readstore;
