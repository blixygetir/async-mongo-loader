use std::fs;
use std::path::PathBuf;

// returns the size of a file in bytes
pub fn file_size(path: &PathBuf) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}
