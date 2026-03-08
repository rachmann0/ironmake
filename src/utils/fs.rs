use std::fs;
use std::io;
// Filesystem utilities

fn recursive_list_files(path: &str) -> io::Result<()>{
    for entry in fs::read_dir(path)? {
        let entry = entry?;              // unwrap Result<DirEntry>
        let path = entry.path();         // full path
        println!("{}", path.display());
    }
    Ok(())
}