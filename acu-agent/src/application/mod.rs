//! Application layer with commands, queries and buses.
//!
//! # Examples
//! ```
//! use acu_agent::application::{Command, CommandBus};
//!
//! struct Ping;
//! impl Command for Ping {}
//!
//! struct Bus;
//! impl CommandBus for Bus {
//!     fn dispatch<C: Command>(&self, _cmd: C) {}
//! }
//!
//! let bus = Bus;
//! bus.dispatch(Ping);
//! ```

/// Marker trait for commands.
pub trait Command {}

/// Marker trait for queries.
pub trait Query {}

/// Command bus stub.
pub trait CommandBus {
    /// Dispatch a command.
    fn dispatch<C: Command>(&self, cmd: C);
}

/// Query bus stub.
pub trait QueryBus<R> {
    /// Dispatch a query.
    fn dispatch<Q: Query>(&self, query: Q) -> R;
}
