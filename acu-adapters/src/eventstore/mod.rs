//! In-memory event store stub.
//!
//! # Examples
//! ```
//! use acu_adapters::eventstore::MemoryEventStore;
//! let mut store = MemoryEventStore::default();
//! store.append("stream".into(), "event".into());
//! ```

use std::collections::HashMap;

/// Simple in-memory event store.
#[derive(Default)]
pub struct MemoryEventStore {
    events: HashMap<String, Vec<String>>,
}

impl MemoryEventStore {
    /// Append an event to a stream.
    pub fn append(&mut self, stream: String, event: String) {
        self.events.entry(stream).or_default().push(event);
    }
}
