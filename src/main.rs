mod ds;
mod builder;
mod utils;

use crate::ds::artifact::{Artifact};
// ? crate:: avoids ambiguity (comes from current crate, not dependancy)
// ? This path always starts from the crate root.
use crate::builder::compiler::{GCC}; 
use crate::builder::build_context::{Build, Modes};
use crate::ds::graph::{Graph};
use crate::utils::logger::{init_logger, LogLevel, parse_log_level};
use crate::utils::fs::{list_files};

// use std::fs::create_dir_all;
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
    // // ! init /build folder
    // create_dir_all("./build").expect("Failed to create build dir");

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

    // ! init graph nodes from list of files
    let mut nodes:Vec<Artifact> = vec![];
    // let path_list:Vec<PathBuf> = recursive_list_files(Path::new("."), None)
    let path_list:Vec<PathBuf> = list_files(Path::new("."), None)
    .expect("failed to list files");

    // println!("{:?}", path_list);
    for path in path_list  {
        // println!("{}", path.display());
        //    Artifact::new(path, metadata)
        let artifact: Artifact = Artifact::new(path, None, vec![], false, vec![]);
        // let node:Node = Node::new(artifact, target);
        nodes.push(artifact);
    }
    // println!("nodes.len()={}", nodes.len());

    let mut build_graph:Graph = Graph::new(nodes);

    // ! connect dependancies for each nodes
    let mut dependancy_indexes:Vec<usize> = vec![];
    for (i, el) in build_graph.nodes.iter().enumerate() {
        if el.path.extension().is_some_and(|ext| ext == "c") {
            dependancy_indexes.push(i);
        }
    }

    // TODO: initialize is_built for each artifacts

    // ! init target
    let target:Artifact =
    Artifact::new(
        PathBuf::from("main.exe"),
        None, vec![],
        false, dependancy_indexes
    );
    build_graph.nodes.push(target);
    let target_index:usize = build_graph.nodes.len()-1;
    let target:&Artifact = &build_graph.nodes[target_index];

    println!("{:?}", target.dependancy_indexes);
    for &el in &target.dependancy_indexes {
        println!("{}", build_graph.nodes[el].path.display());
    }
    println!("target={}", build_graph.nodes[target_index].path.display());

    // ! init build context
    let mut build_context1:Build<GCC> = Build { compiler: GCC, mode: Modes::O0, graph: build_graph};

    // ! run build on target
    build_context1.build(target_index);
    // println!("{:?}", target);
}
