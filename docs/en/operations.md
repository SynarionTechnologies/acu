# Operations

## Snapshots

Use the helpers from `acu-adapters` to create and verify snapshots.

```rust
use acu_adapters::snapshots::{create_snapshot, verify_snapshot};
use std::path::Path;
# fn demo() -> Result<(), Box<dyn std::error::Error>> {
let snap = create_snapshot(Path::new("data"), Path::new("snaps"))?;
let _hash = verify_snapshot(&snap)?;
# Ok(())
# }
```

`rotate_snapshots` keeps only the most recent snapshots.
