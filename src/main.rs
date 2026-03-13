mod ds;
mod builder;
mod utils;

use crate::ds::artifact::{self, Artifact};
// ? crate:: avoids ambiguity (comes from current crate, not dependancy)
// ? This path always starts from the crate root.
use crate::builder::compiler::{GCC}; 
use crate::builder::build_context::{Build, Modes};
use crate::ds::graph::{Graph, Rule};
use crate::utils::logger::{init_logger, LogLevel, parse_log_level};
use crate::utils::fs::{recursive_list_files, list_files};

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

    // ! init build context
    let build_context1:Build<GCC> = Build { compiler: GCC, mode: Modes::O0};

    // ! init graph from list of files
    let mut nodes:Vec<Artifact> = vec![];
    // let path_list:Vec<PathBuf> = recursive_list_files(Path::new("."), None)
    let path_list:Vec<PathBuf> = list_files(Path::new("."), None)
    .expect("failed to list files");

    // println!("{:?}", path_list);
    for path in path_list  {
        // println!("{}", path.display());
        //    Artifact::new(path, metadata)
        let artifact: Artifact = Artifact::new(path, None, vec![], false);
        // let node:Node = Node::new(artifact, target);
        nodes.push(artifact);
    }
    // println!("nodes.len()={}", nodes.len());

    let mut build_graph:Graph = Graph::new(nodes, vec![]);

    // ! init target
    // let target:&mut Artifact = &mut Artifact::new(PathBuf::from("main.exe"), None, vec![], false);
    let target:Artifact = Artifact::new(PathBuf::from("main.exe"), None, vec![], false);
    build_graph.nodes.push(target);
    let target_index:usize = build_graph.nodes.len()-1;

    // ! connect init to target
    let mut dependancy_indexes:Vec<usize> = vec![];
    for (i, el) in build_graph.nodes.iter().enumerate() {
        if el.path.extension().is_some_and(|ext| ext == "c") {
            dependancy_indexes.push(i);
        }
    }
    let rule:Rule = Rule::new(dependancy_indexes, target_index);
    build_graph.edges.push(rule);
    // println!("{:?}", build_graph.nodes);
    // println!("{:?}", build_graph.edges);

    // ! run build on target
    build_context1.build(target_index, build_graph);
    // println!("{:?}", target);
    

    // // ! dependancies
    // let mut dependancies:Vec<Artifact> = vec![];
    // let path_list:Vec<PathBuf> = recursive_list_files(Path::new("."), "c")
    // .expect("failed to list files");


    // for path in path_list  {
    //     println!("{}", path.display());
    //     //    Artifact::new(path, metadata)
    //     let artifact: Artifact = Artifact::new(path, None, vec![], false);
    //     // let node:Node = Node::new(artifact, target);
    //     dependancies.push(artifact);
    // }

    // println!("dependancies.len() = {}", dependancies.len());

    // // ! target
    // let mut target:Artifact = Artifact::new(PathBuf::from("main.exe"), None, dependancies, false);

    // // ! build
    // build_context1.build(&mut target);

    // let files:[&str;2] = ["main.c", "math.c"];


    // let artifacts:Vec<Artifact> =
    // files.iter()
    // .map(|f| Artifact::new(PathBuf::from(f), None, vec![], false))
    // .collect();
    // // build_context1.compile(&artifacts).expect("Compile Failed");
    // match build_context1.compile(&artifacts) {
    // Ok(output) => {
    //     println!("Compilation succeeded:\n{}", output);
    // }
    // Err(error) => {
    //     eprintln!("Compilation failed:\n{}", error);
    // }
    // }


    // match build_context1.link(&artifacts) {
    // Ok(output) => {
    //     println!("Linking succeeded:\n{}", output);
    // }
    // Err(error) => {
    //     eprintln!("Linking failed:\n{}", error);
    // }
    // }


}
