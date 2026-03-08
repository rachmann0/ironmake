mod fundamentals;
use crate::fundamentals::artifact::Artifact;
// ? crate:: avoids ambiguity (comes from current crate, not dependancy)
// ? This path always starts from the crate root.
use crate::fundamentals::compiler::{GCC}; 
use crate::fundamentals::build_context::{Build, Modes};

// ? std
use std::path::PathBuf;

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
