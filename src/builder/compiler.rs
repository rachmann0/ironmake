use std::process::Command;
use crate::ds::artifact::{Artifact, ArtifactType};
use crate::ds::graph::{Graph};
use crate::{log_debug, log_error};

pub trait Compiler {
    fn compile(&self, extra_flags:&str, artifacts: &[Artifact])->Result<String, String>;
    fn compile2(&self, extra_flags:&str, target_index: usize, graph:&mut Graph);
    fn link(&self, extra_flags:&str, target_index: usize, graph:&mut Graph);
}

pub struct GCC;
impl Compiler for GCC {
    fn compile2(&self, extra_flags:&str, target_index: usize, graph:&mut Graph){
        // Gather immutable data first to avoid simultaneous borrows
        if let Some(target_ref) = graph.nodes.get(target_index) {
            let dependancy_indexes = target_ref.dependancy_indexes.clone();
            let filename_lossy_owned = target_ref.path.file_name().unwrap().to_string_lossy().into_owned();

            let flag:&str = "-c"; // compile to object file

            // Collect file paths from the requested dependency indexes.
            let filepaths: Vec<String> = dependancy_indexes
                .iter()
                .filter_map(|&idx| graph.nodes.get(idx))
                .map(|a| a.path.to_string_lossy().into())
                .collect();

            // combine all args (extra flags, compile flag, source paths)
            let all_args: Vec<&str> =
                std::iter::once(extra_flags)
                .chain(std::iter::once(flag))
                .chain(filepaths.iter().map(|s| s.as_str()))
                .chain(std::iter::once(filename_lossy_owned.as_ref()))
                .collect();

            // ! Log the command
            log_debug!("Running command: gcc {}", all_args.join(" "));

            // ! run the command
            let output = Command::new("gcc")
                .args(&all_args)
                .output()
                .map_err(|e| format!("Failed to run gcc: {e}"));

            match output {
                Ok(output) =>{
                    if output.status.success() {
                        if let Some(target_mut) = graph.nodes.get_mut(target_index) {
                            target_mut.is_built = true;
                            // change extension from source (e.g., main.c) to object (.o)
                            target_mut.path.set_extension("o");
                            target_mut.artifact_type = ArtifactType::Object;
                        }
                        log_debug!("Command Success: {}", String::from_utf8_lossy(&output.stdout).to_string());
                    } else {
                        log_error!("Command Error: {}", String::from_utf8_lossy(&output.stderr).to_string());
                    }
                }
                _ => {}
            }
        } else {
            log_error!("Compiler GCC: invalid target index");
        }
    }

    /// input
    fn compile(&self, extra_flags:&str, artifacts: &[Artifact])->Result<String, String>{
        let filepaths: Vec<String> = artifacts
        .iter()
        .map(|a| a.path.to_string_lossy().into())
        .collect();

        let flag:&str = "-c"; // compile to object file

        // combine all args
        let all_args: Vec<&str> = [extra_flags, flag].into_iter()
        .chain(filepaths.iter().map(|s| s.as_str())) 
        .collect();

        // Log the command
        log_debug!("Running command: gcc {}", all_args.join(" "));

        let output = Command::new("gcc")
            .args(&all_args)
            .output()
            .map_err(|e| format!("Failed to run gcc: {e}"))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    fn link(&self, extra_flags:&str, target_index: usize, graph:&mut Graph){
        // Gather immutable data first to avoid simultaneous borrows
        if let Some(target_ref) = graph.nodes.get(target_index) {
            let dependancy_indexes = target_ref.dependancy_indexes.clone();
            let filename_lossy_owned = target_ref.path.file_name().unwrap().to_string_lossy().into_owned();

            // let flag:&str = "-c"; // compile to object file

            // Collect file paths from the requested dependency indexes.
            let filepaths: Vec<String> = dependancy_indexes
                .iter()
                .filter_map(|&idx| graph.nodes.get(idx))
                .map(|a| a.path.to_string_lossy().into())
                .collect();

            // combine all args (extra flags, compile flag, source paths)
            let all_args: Vec<&str> =
                std::iter::once(extra_flags)
                // .chain(std::iter::once(flag))
                .chain(filepaths.iter().map(|s| s.as_str()))
                .chain(std::iter::once("-o"))
                .chain(std::iter::once(filename_lossy_owned.as_ref()))
                .collect();

            // ! Log the command
            log_debug!("Running command: gcc {}", all_args.join(" "));

            // ! run the command
            let output = Command::new("gcc")
                .args(&all_args)
                .output()
                .map_err(|e| format!("Failed to run gcc: {e}"));

            match output {
                Ok(output) =>{
                    if output.status.success() {
                        if let Some(target_mut) = graph.nodes.get_mut(target_index) {
                            target_mut.is_built = true;
                            // change extension from source (e.g., main.c) to object (.o)
                            target_mut.path.set_extension("o");
                            target_mut.artifact_type = ArtifactType::Object;
                        }
                        log_debug!("Command Success: {}", String::from_utf8_lossy(&output.stdout).to_string());
                    } else {
                        log_error!("Command Error: {}", String::from_utf8_lossy(&output.stderr).to_string());
                    }
                }
                _ => {}
            }
        } else {
            log_error!("Compiler GCC: invalid target index");
        }

    }
}

// pub struct Clang;
// impl Compiler for Clang {
//     fn compile(&self, artifacts: &[Artifact])->Result<String, String> {
//         println!("clang compiling {:#?}", artifacts);
//         Ok("Ok".to_string())
//     }

//     fn link(&self) {
//         println!("clang linking");
//     }
// }
