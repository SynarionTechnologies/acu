use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common metadata for all events.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EventMeta {
    /// Unique identifier of the event.
    pub id: Uuid,
    /// Time when the event occurred (UTC).
    pub occurred_at: DateTime<Utc>,
    /// Identifier of the aggregate root.
    pub aggregate_id: Uuid,
    /// Optional origin of the event.
    pub source: Option<String>,
}

impl EventMeta {
    /// Convenience constructor.
    pub fn new(
        id: Uuid,
        aggregate_id: Uuid,
        occurred_at: DateTime<Utc>,
        source: Option<String>,
    ) -> Self {
        Self {
            id,
            occurred_at,
            aggregate_id,
            source,
        }
    }
}
