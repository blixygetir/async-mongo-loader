use super::metadata;
use std::collections::BTreeMap;
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

// iterates over the files in a directory and its subdirectories
// save the files sizes and their paths in a BTreeMap
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<BTreeMap<u64, PathBuf>> {
    let mut file_sizes: BTreeMap<u64, PathBuf> = BTreeMap::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                file_sizes.insert(metadata::file_size(&path)?, path.clone());
                cb(&entry);
            } else if path.is_dir() {
                visit_dirs(&path, cb)?;
            }
        }
    }

    Ok(file_sizes)
}
