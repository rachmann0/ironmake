use std::process::Command;
use crate::fundamentals::file::{File};

pub trait Compiler {
    fn compile(&self, file: &[&str])->&str;
    fn link(&self);
}
pub struct GCC;
impl Compiler for GCC {
    fn compile(&self, files: &[&str])->&str{
        // fn compile(source: &str, output: &str) {
        // Command::new("gcc")
        // .args([source, "-o", output])
        // .status()
        // .expect("gcc failed");
        // }

        // println!("gcc compiling {}", file);
        // let status = Command::new("gcc")
        //     .args(["main.c", "-o", "main"])
        //     .status()
        //     .expect("failed to execute gcc");

        // if !status.success() {
        //     panic!("Compilation failed");
        // }

        let flags:&str = "-c"; // compile to object file
        let output_path:&str = "main.o"; // compile to object file

        let output = Command::new("gcc")
        .args([flags])
        .args(files)
        .args(["-o", output_path])
        .output()
        .expect("failed to run gcc");

        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        return output_path;
    }

    fn link(&self) {
        println!("gcc linking");
    }
}

pub struct Clang;
impl Compiler for Clang {
    fn compile(&self, files: &[&str])->&str {
        println!("clang compiling {:?}", files);
        return "";
    }

    fn link(&self) {
        println!("clang linking");
    }
}
