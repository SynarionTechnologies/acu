# Memory hierarchy

ACU uses a basic hot/warm/cold hierarchy. Projections are kept in a key/value
store (SQLite by default). Vector search uses a small persistent ANN index.
Blobs are stored on the local filesystem and can be moved to S3 with the `s3`
feature. Snapshot utilities allow backup and restore of both projections and
ANN data.

```toml
[memory.projections]
engine = "sqlite"
path = "/var/lib/acu/projections/acu.db"
```
