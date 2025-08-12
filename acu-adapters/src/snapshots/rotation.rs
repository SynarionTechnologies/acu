use std::fs;
use std::path::Path;

/// Keep only the last `keep` snapshots in `root` directory.
pub fn rotate_snapshots(root: &Path, keep: usize) -> Result<(), std::io::Error> {
    let mut dirs: Vec<_> = fs::read_dir(root)?
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| e.path())
        .collect();
    dirs.sort();
    if dirs.len() > keep {
        let remove_count = dirs.len() - keep;
        for old in dirs.into_iter().take(remove_count) {
            fs::remove_dir_all(old)?;
        }
    }
    Ok(())
}
