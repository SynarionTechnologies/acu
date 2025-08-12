//! Observability utilities.
//!
//! # Examples
//! ```
//! use acu_adapters::observability::log_info;
//! log_info("hello");
//! ```

/// Log an informational message.
pub fn log_info(message: &str) {
    log::info!("{message}");
}
