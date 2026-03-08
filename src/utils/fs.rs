use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// ! Filesystem utilities

#[allow(dead_code)]
pub fn list_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        files.push(path);
    }

    Ok(files)
}

pub fn recursive_list_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(recursive_list_files(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}