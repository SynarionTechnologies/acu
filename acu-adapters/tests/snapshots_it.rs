use acu_adapters::snapshots::{
    create_snapshot, restore_snapshot, rotate_snapshots, verify_snapshot,
};
use std::fs;

#[test]
fn snapshot_create_verify_restore() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let src = dir.path().join("src");
    fs::create_dir_all(&src)?;
    fs::write(src.join("file.txt"), b"hello")?;
    let root = dir.path().join("snaps");
    let snap1 = create_snapshot(&src, &root)?;
    let hash1 = verify_snapshot(&snap1)?;
    fs::write(src.join("file.txt"), b"hello2")?;
    let snap2 = create_snapshot(&src, &root)?;
    let hash2 = verify_snapshot(&snap2)?;
    assert_ne!(hash1, hash2);
    rotate_snapshots(&root, 1)?;
    assert!(!snap1.exists());
    restore_snapshot(&snap2, &src)?;
    let restored = fs::read(src.join("file.txt"))?;
    assert_eq!(restored, b"hello2");
    Ok(())
}
