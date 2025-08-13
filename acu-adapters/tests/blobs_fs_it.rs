use acu_adapters::blobs::fs::FsBlobStore;

#[test]
fn fs_blob_store_ops() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let store = FsBlobStore::new(dir.path())?;
    let hash = store.put("docs/readme.txt", b"hello world")?;
    assert_eq!(hash.len(), 64);
    assert!(store.exists("docs/readme.txt"));
    let data = store.get("docs/readme.txt")?;
    assert_eq!(data, b"hello world");
    let list = store.list()?;
    assert_eq!(list.len(), 1);
    store.delete("docs/readme.txt")?;
    assert!(!store.exists("docs/readme.txt"));
    Ok(())
}
