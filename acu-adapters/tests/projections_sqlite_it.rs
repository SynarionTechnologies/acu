use acu_adapters::projections_kv::{KvStore, SqliteStore};

#[test]
fn sqlite_crud_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("test.db");
    let store = SqliteStore::new(&path)?;
    store.put("items", "a", b"one")?;
    assert_eq!(store.get("items", "a")?, Some(b"one".to_vec()));
    store.put("items", "b", b"two")?;
    let scan = store.scan_prefix("items", "")?;
    assert_eq!(scan.len(), 2);
    store.delete("items", "a")?;
    assert_eq!(store.get("items", "a")?, None);
    Ok(())
}
