# Events

ACU records immutable domain events to track every change of the agent. Events
are stored with UTC timestamps and unique identifiers. Replaying the sequence of
events rebuilds the state of aggregates.

## Categories

- **Profile & Policies:** `ProfileCreated`, `PolicyUpdated`, `AuthenticationKeyAdded` …
- **Interaction:** `UserMessageReceived`, `ResponseGenerated`, `ToolInvoked` …
- **Curiosity & Exploration:** `CuriosityExplorationStarted`, `SourceAddedToWhitelist` …
- **Knowledge & Modules:** `KnowledgeModuleEnabled`, `KnowledgeItemAdded` …
- **Memory:** `MemoryItemAppended`, `MemoryCompacted` …
- **Learning & Reward:** `TrainingSessionStarted`, `RewardObserved` …
- **System & Maintenance:** `AgentStarted`, `BackupCreated`, `ErrorLogged` …
- **Integrations:** `ExternalApiCalled`, `WebhookReceived`, `IntegrationEnabled` …

## JSON Example

```json
{
  "type": "ProfileCreated",
  "data": {
    "meta": {
      "id": "01HZY1YQ5YQWFM1K4ZXW1Y1X3F",
      "occurred_at": "2024-01-01T00:00:00Z",
      "aggregate_id": "01HZY1YQ5YQWFM1K4ZXW1Y1X3F",
      "source": null
    },
    "name": "Alice"
  }
}
```
