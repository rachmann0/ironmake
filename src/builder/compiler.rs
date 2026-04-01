use std::process::Command;
use std::path::{PathBuf};
// use crate::utils::cache::{compute_file_hash, load_cache, save_cache};

// Change this constant to redirect outputs to a different folder
use crate::ds::artifact::{ArtifactType};
use crate::ds::graph::{Graph};
use crate::{log_debug, log_error};

const BUILD_DIR: &str = "build";

pub trait Compiler {
    fn compile(&self, extra_flags:&str, target_index: usize, graph:&mut Graph, header_dirs:&Vec<String>);
    fn link(&self, extra_flags:&str, target_index: usize, graph:&mut Graph);
}

pub struct GCC;
impl Compiler for GCC {
    fn compile(&self, extra_flags:&str, target_index: usize, graph:&mut Graph, header_dirs:&Vec<String>){
        // Gather immutable data first to avoid simultaneous borrows
        if let Some(target_ref) = graph.nodes.get(target_index) {
            let dependancy_indexes = target_ref.dependancy_indexes.clone();
            let filename_lossy_owned = target_ref.path.file_name().unwrap().to_string_lossy().into_owned();

            let flag:&str = "-c"; // compile to object file

            let filename_full = target_ref.path.to_string_lossy().into_owned();

            // Collect file paths from the requested dependency indexes.
            let filepaths: Vec<String> = dependancy_indexes
                .iter()
                .filter_map(|&idx| graph.nodes.get(idx))
                .map(|a| a.path.to_string_lossy().into())
                .collect();

            // prepare output path into object build/<filename>.o
            let mut output_o_path = PathBuf::from(BUILD_DIR);
            output_o_path.push(&filename_lossy_owned);
            output_o_path.set_extension("o");
            let output_o_path_str = output_o_path.to_string_lossy().into_owned();

            // prepare output path into dependancy list build/<filename>.d
            let mut output_d_path = PathBuf::from(BUILD_DIR);
            output_d_path.push(&filename_lossy_owned);
            output_d_path.set_extension("d");
            let output_d_path_str = output_d_path.to_string_lossy().into_owned();

            // combine all args (extra flags, compile flag, source paths, target source, -o <out>)
            let all_args: Vec<String> =
                std::iter::once(extra_flags.to_string())
                .chain(std::iter::once(flag.to_string()))
                // .chain(std::iter::once(format!("-I{}",include_dir)))
                .chain(header_dirs.iter().map(|s| format!("-I{}",s)))
                .chain(filepaths.iter().map(|s| s.to_string()))
                .chain(std::iter::once(filename_full))
                .chain(std::iter::once("-o".to_string())) // object file
                .chain(std::iter::once(output_o_path_str))
                .chain(std::iter::once("-MMD".to_string())) // dependancy list
                .chain(std::iter::once("-MF".to_string()))
                .chain(std::iter::once(output_d_path_str))
                .collect();

            // ! Log the command
            log_debug!("compile command: gcc {}", all_args.join(" "));

            // ! ensure build directory exists
            let _ = std::fs::create_dir_all(BUILD_DIR);

            // ! run the command
            let output = Command::new("gcc")
                .args(&all_args)
                .output()
                .map_err(|e| format!("Failed to run gcc: {e}"));

            match output {
                Ok(output) =>{
                    if output.status.success() {
                        if let Some(target_mut) = graph.nodes.get_mut(target_index) {
                            // target_mut.is_built = true;
                            // change path to build/<filename>.o
                                let mut new_path = PathBuf::from(BUILD_DIR);
                                new_path.push(&filename_lossy_owned);
                                new_path.set_extension("o");
                                target_mut.path = new_path;
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

            // prepare output path into build/<filename>
            let mut output_path = PathBuf::from(BUILD_DIR);
            output_path.push(&filename_lossy_owned);
            let output_path_str = output_path.to_string_lossy().into_owned();

            // combine all args (extra flags, source paths, -o <out>)
            let all_args: Vec<&str> =
                std::iter::once(extra_flags)
                .chain(filepaths.iter().map(|s| s.as_str()))
                .chain(std::iter::once("-o"))
                .chain(std::iter::once(output_path_str.as_ref()))
                .collect();

            // ! Log the command
            log_debug!("link command: gcc {}", all_args.join(" "));

            // ! ensure build directory exists
            let _ = std::fs::create_dir_all(BUILD_DIR);

            // ! run the command
            let output = Command::new("gcc")
                .args(&all_args)
                .output()
                .map_err(|e| format!("Failed to run gcc: {e}"));

            match output {
                Ok(output) =>{
                    if output.status.success() {
                        if let Some(target_mut) = graph.nodes.get_mut(target_index) {
                            // target_mut.is_built = true;
                            // set linked output into build/<filename>
                            target_mut.path = output_path.clone();
                            // set artifact type to Binary for linked executables
                            target_mut.artifact_type = ArtifactType::Binary;
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
