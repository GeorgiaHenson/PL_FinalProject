use std::fs;
use std::path::{Path, PathBuf};

pub fn get_sorted_files_by_size(dir: &Path) -> Vec<(PathBuf, u64)> {
    let mut files = Vec::new();
    collect_files(dir, &mut files);
    files.sort_by_key(|k| k.1);
    files
}

fn collect_files(dir: &Path, files: &mut Vec<(PathBuf, u64)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    files.push((path, metadata.len()));
                }
            } else if path.is_dir() {
                collect_files(&path, files);
            }
        }
    }
}