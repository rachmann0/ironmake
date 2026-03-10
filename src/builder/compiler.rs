use std::process::Command;
use crate::graph::artifact::{Artifact};
use crate::log_debug;

pub trait Compiler {
    fn compile(&self, extra_flags:&str, artifacts: &[Artifact])->Result<String, String>;
    fn compile2(&self, extra_flags:&str, artifacts: &[&mut Artifact])->Result<String, String>;
    fn link(&self, extra_flags:&str, artifacts: &[Artifact])->Result<String, String>;
}

pub struct GCC;
impl Compiler for GCC {
    fn compile2(&self, extra_flags:&str, artifacts: &[&mut Artifact])->Result<String, String>{
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

    fn link(&self, extra_flags:&str, artifacts: &[Artifact])->Result<String, String>{
        println!("gcc linking");
        dbg!(artifacts);
        dbg!(extra_flags);
        Ok("gcc linking".to_string())
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
