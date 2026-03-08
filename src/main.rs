mod graph;
mod fundamentals;
mod utils;

use crate::fundamentals::artifact::Artifact;
// ? crate:: avoids ambiguity (comes from current crate, not dependancy)
// ? This path always starts from the crate root.
use crate::fundamentals::compiler::{GCC}; 
use crate::fundamentals::build_context::{Build, Modes};
use crate::utils::logger::{init_logger, LogLevel, parse_log_level};
use crate::utils::fs::{recursive_list_files};

// ? std
use std::path::{Path, PathBuf};
use std::env;

/*

!1. Create build context
!2. Use build context to perform fundamental operations (compile and link)

*/

/*

? The 6 Core Build Primitives
1.Compile
2.Link
3.Archive
4.Depencancy Tracking (Know what needs rebuilding.)
5.File Discovery
6.Configuration / Target Selection

*/

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running From: {}", args[0]);

    let path_list:Vec<PathBuf> = recursive_list_files(Path::new(".")).expect("failed to list files");
    for path in path_list  {
        println!("{}", path.display());
    }

    // ! initialize Logger
    let log_level:LogLevel = parse_log_level();
    init_logger(log_level);

    log_info!("Build started");

    // log from main
    log_error!("error test");
    log_warn!("warn test");
    log_info!("info test");
    log_debug!("debug test");
    log_trace!("trace test");

    let files:[&str;2] = ["main.c", "math.c"];

    let build_context1:Build<GCC> = Build { compiler: GCC, mode: Modes::O0};
    let artifacts:Vec<Artifact> =
    files.iter()
    .map(|f| Artifact::new(PathBuf::from(f), None))
    .collect();
    // build_context1.compile(&artifacts).expect("Compile Failed");
    match build_context1.compile(&artifacts) {
    Ok(output) => {
        println!("Compilation succeeded:\n{}", output);
    }
    Err(error) => {
        eprintln!("Compilation failed:\n{}", error);
    }
    }


    // match build_context1.link(&artifacts) {
    // Ok(output) => {
    //     println!("Linking succeeded:\n{}", output);
    // }
    // Err(error) => {
    //     eprintln!("Linking failed:\n{}", error);
    // }
    // }


}
