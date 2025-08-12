# Événements

ACU enregistre des événements de domaine immuables pour suivre chaque
modification de l’agent. Les événements sont horodatés en UTC et identifiés de
façon unique. La relecture de la séquence d’événements reconstruit l’état des
agrégats.

## Catégories

- **Profil & Politiques :** `ProfileCreated`, `PolicyUpdated`, `AuthenticationKeyAdded` …
- **Interaction :** `UserMessageReceived`, `ResponseGenerated`, `ToolInvoked` …
- **Curiosité & Exploration :** `CuriosityExplorationStarted`, `SourceAddedToWhitelist` …
- **Connaissance & Modules :** `KnowledgeModuleEnabled`, `KnowledgeItemAdded` …
- **Mémoire :** `MemoryItemAppended`, `MemoryCompacted` …
- **Apprentissage & Récompense :** `TrainingSessionStarted`, `RewardObserved` …
- **Système & Maintenance :** `AgentStarted`, `BackupCreated`, `ErrorLogged` …
- **Intégrations :** `ExternalApiCalled`, `WebhookReceived`, `IntegrationEnabled` …

## Exemple JSON

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
