use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::graph::artifact::{ArtifactType};


// ! Filesystem utilities
#[allow(dead_code)]
pub fn list_files(path: &Path, extension:Option<&str>) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
        } else {
            // match extension {
            //     Some(extension)=>{
            //         if path.extension().is_some_and(|ext| ext == extension) {
            //             files.push(path);
            //         }
            //     }
            //     None =>{
            //         files.push(path);
            //     }
            // }
            match ArtifactType::classify(&path) {
                ArtifactType::Other=>{} // invalid ext
                _ => {
                    match extension {
                        Some(extension)=>{
                            if path.extension().is_some_and(|ext| ext == extension) {
                                files.push(path);
                            }
                        }
                        None =>{
                            files.push(path);
                        }
                    }
                }
            };

        }
    }

    Ok(files)
}

pub fn recursive_list_files(path: &Path, extension:Option<&str>) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(recursive_list_files(&path, extension)?);
        } else {
            match ArtifactType::classify(&path) {
                ArtifactType::Other=>{} // invalid ext
                _ => {
                    match extension {
                        Some(extension)=>{
                            if path.extension().is_some_and(|ext| ext == extension) {
                                files.push(path);
                            }
                        }
                        None =>{
                            files.push(path);
                        }
                    }
                }
            };
        }
    }

    Ok(files)
}