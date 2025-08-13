use acu_adapters::ann_hnsw::{snapshot, AnnIndex};

#[test]
fn ann_insert_search_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let index_path = dir.path().join("ann.bin");
    let mut index = AnnIndex::new(&index_path, 3);
    index.insert(1, vec![0.0, 0.0, 1.0]);
    index.insert(2, vec![0.0, 1.0, 0.0]);
    index.insert(3, vec![1.0, 0.0, 0.0]);
    let results = index.search(&[0.0, 0.0, 1.0], 1);
    assert_eq!(results[0].0, 1);
    index.persist()?;
    let snap_dir = dir.path().join("snapshots");
    let (file, hash) = snapshot::snapshot(&index, &snap_dir)?;
    assert_eq!(hash.len(), 64);
    let restored = snapshot::restore(file)?;
    let results2 = restored.search(&[0.0, 0.0, 1.0], 1);
    assert_eq!(results2[0].0, 1);
    Ok(())
}
