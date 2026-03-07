mod fundamentals;
// crate:: avoids ambiguity (comes from current crate, not dependancy)
// This path always starts from the crate root.
use crate::fundamentals::compiler::{GCC}; 
use crate::fundamentals::build_context::{Build};

// std
use std::process::Command;

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

? Optimization Levels
GCC optimizes code at compile time, which takes extra time.
Lower optimization = faster compile.

-O0 → No optimization. Fastest compile, bigger binary, easy to debug.
-O1, -O2, -O3 → Increasing optimizations → slower compile, smaller/faster runtime code.
-Os → Optimize for size.
-Ofast → Aggressive optimizations (may break strict standard compliance).

Tip: For development builds, use -O0 to speed up compilation.

*/

fn main() {
    // let status = Command::new("gcc")
    //     .args(["math.c","main.c", "-o", "main"])
    //     .status()
    //     .expect("failed to execute gcc");

    // if !status.success() {
    //     panic!("Compilation failed");
    // }

    let files:[&str;2] = ["main.c", "math.c"];
    // let files:[&str;1] = ["main.c"];

    let flags:&str = "-c"; // compile to object file
    let output_path:&str = "main.o"; // compile to object file

    let output = Command::new("gcc")
    .args([flags])
    .args(["-O0"])
    .args(files)
    // .args(["-o", output_path])
    .output()
    .expect("failed to run gcc");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));



    // let build_context1:Build<GCC> = Build { compiler: GCC };
    // build_context1.compile(&files);


}
