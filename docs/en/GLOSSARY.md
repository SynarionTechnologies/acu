# Glossary

- **Aggregate**: Consistent boundary containing entities and value objects.
- **Projection**: Read model derived from events.
- **Event**: Immutable record describing something that happened.
- **Replay**: Process of applying past events to rebuild state.
- **Upcaster**: Adapter that upgrades events from older schemas.
- **Event Store**: Persistence for ordered domain events.
- **Command**: Request to change system state.
- **Query**: Request to read system state.
- **Idempotency**: Property of producing the same result when executed multiple times.
- **ULID/UUID**: Unique identifiers used for entities and events.
- **Retention Policy**: Rules for keeping or deleting events over time.
- **Compaction**: Physical cleanup of storage to reclaim space.
- **Archiving**: Moving data from hot or warm storage to cold storage.
- **Curiosity Policy**: Configures exploration behaviors of the agent.
- **Safety Policy**: Guards against harmful actions.
