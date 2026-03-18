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
use crate::utils::fs::{recursive_list_files};

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
    let args: Vec<String> = env::args().collect();
    println!("Running From: {}", args[0]);

    // ! initialize Logger
    let log_level:LogLevel = parse_log_level();
    init_logger(log_level);

    log_info!("Build started");

    // ! init graph nodes from list of files
    let mut nodes:Vec<Artifact> = vec![];
    // TODO: init /include dir to tell compiler where to find header files
    const INCLUDE_DIR:&str = "./include";
    let header_path_list:Vec<PathBuf> = recursive_list_files(Path::new(INCLUDE_DIR), None)
    .expect("failed to list header files");
    let source_path_list:Vec<PathBuf> = recursive_list_files(Path::new("./src"), None)
    .expect("failed to list files");

    let mut path_list:Vec<PathBuf> = vec![];
    path_list.extend(header_path_list);
    path_list.extend(source_path_list);

    // println!("{:?}", path_list);
    for path in path_list  {
        // println!("{}", path.display());
        //    Artifact::new(path, metadata)
        let artifact: Artifact = Artifact::new(path, false, vec![]);
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

    // ! init target
    let target:Artifact =
    Artifact::new(
        PathBuf::from("main.exe"),
        false, dependancy_indexes,
    );
    build_graph.nodes.push(target);
    let target_index:usize = build_graph.nodes.len()-1;
    let target:&Artifact = &build_graph.nodes[target_index];

    // println!("{:?}", target.dependancy_indexes);
    for &el in &target.dependancy_indexes {
        println!("{}", build_graph.nodes[el].path.display());
    }
    println!("target={}", build_graph.nodes[target_index].path.display());

    // ! init build context
    let mut build_context1:Build<GCC> =
    Build {
        compiler: GCC,
        mode: Modes::O0,
        graph: build_graph,
        header_dirs:vec![INCLUDE_DIR.to_string()]
    };

    // ! run build on target
    build_context1.build(target_index);
    // println!("{:?}", target);
}
