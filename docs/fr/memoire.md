# Hiérarchie mémoire

ACU adopte une hiérarchie simple hot/warm/cold. Les projections sont stockées
dans un magasin clé/valeur (SQLite par défaut). La recherche vectorielle repose
sur un index ANN persistant. Les blobs sont stockés sur le système de fichiers
local et peuvent être envoyés vers S3 via la feature `s3`. Des utilitaires de
snapshot permettent la sauvegarde et la restauration des données.

```toml
[memory.projections]
engine = "sqlite"
path = "/var/lib/acu/projections/acu.db"
```
