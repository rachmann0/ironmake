use crate::builder::compiler::{Compiler};
use crate::ds::artifact::{ArtifactType};
use crate::ds::graph::Graph;
use std::path::{PathBuf};
use crate::{log_error, log_debug};
use crate::utils::cache::{compute_file_hash, load_cache, save_cache};

// use crate::{log_error, log_info};

/* 
    build context
    ↓
    compile()
    link()
    run()
*/

/* 

? Optimization Levels
GCC optimizes code at compile time, which takes extra time.
Lower optimization = faster compile.

-O0 → No optimization. Fastest compile, bigger binary, easy to debug.
-O1, -O2, -O3 → Increasing optimizations → slower compile, smaller/faster runtime code.
-Os → Optimize for size.
-Ofast → Aggressive optimizations (may break strict standard compliance).

Tip: For development builds, use -O0 to speed up compilation.

*/

const BUILD_DIR: &str = "build";

#[allow(dead_code)]
pub enum Modes {
    O0,
    O1,
    O2,
    O3,
    Os,
    Ofast,
}
impl Modes {
    fn flag_value(&self) -> &'static str {
        match self {
            Modes::O0 => "-O0",
            Modes::O1 => "-O1",
            Modes::O2 => "-O2",
            Modes::O3 => "-O3",
            Modes::Os => "-Os",
            Modes::Ofast => "-Ofast",
        }
    }
}

pub struct Build<C: Compiler> {
    pub compiler: C,
    pub mode: Modes,
    pub graph: Graph,
    pub header_dirs: Vec<String>
}
impl<C: Compiler> Build<C> {
    pub fn compile(&mut self, target_index:usize) {
        let extra_flags:&str = self.mode.flag_value();
        self.compiler.compile(extra_flags, target_index, &mut self.graph, &self.header_dirs);
    }
    pub fn link(&mut self, target_index:usize){
        let extra_flags:&str = self.mode.flag_value();
        self.compiler.link(extra_flags, target_index, &mut self.graph);
    }

    pub fn build(&mut self, target_index:usize){
        // Build dependencies for this target first
        let dependancy_indexes =
        self.graph.nodes[target_index].dependancy_indexes.clone();
        log_debug!("{:#?}", dependancy_indexes);

        if dependancy_indexes.is_empty() {
            if let Some(artifact) = self.graph.nodes.get(target_index) {
                match artifact.artifact_type {
                    ArtifactType::Source => {
                        // if self.is_cache_changed(target_index) {
                        // log_debug!("is_cache_changed");
                            self.compile(target_index);
                        // }
                    },
                    _ => {}
                }
            }
            
        } else {
            let mut is_all_cache_unchanged:bool = true;
            for dep_idx in dependancy_indexes {
                // let built = self.graph.nodes[dep_idx].is_built;
                if self.is_cache_changed(dep_idx) {
                    self.build(dep_idx);
                    is_all_cache_unchanged = false;
                }
            }
            // link target after dependencies
            // if !self.graph.nodes[target_index].is_built {
                // self.link(target_index);
            // }
            if !is_all_cache_unchanged {
                self.link(target_index);
            }
        }
    }

    fn is_cache_changed(&mut self, target_index: usize)->bool{
        if let Some(target_ref) = self.graph.nodes.get(target_index) {
        log_debug!("is_cache_changed {:?}", target_ref.path);

            let filename_lossy_owned = target_ref.path.file_name().unwrap().to_string_lossy().into_owned();

            // Compute source file hash
            let filename_full = target_ref.path.to_string_lossy().into_owned();
            let source_key = filename_full.clone();

            // Load cache (file: build/cache/build_cache.csv)
            let mut cache = load_cache().unwrap_or_default();
            let current_hash = match compute_file_hash(&target_ref.path) {
                Ok(h) => h,
                Err(e) => {
                    log_error!("Failed to compute hash for {}: {}", filename_full, e);
                    String::new()
                }
            };

            if let Some(cached) = cache.get(&source_key) {
                // if changed
                if cached != &current_hash || current_hash.is_empty() {
                    log_debug!("{:?}, is changed", target_ref.path);
                    // if changed, update cache with new hash
                    if !current_hash.is_empty() {
                        cache.insert(source_key.clone(), current_hash.clone());
                        if let Err(e) = save_cache(&cache) {
                            log_error!("Failed to save cache: {}", e);
                        }
                    }

                    return true;
                }
            } else {
                // Cache miss: treat as changed, insert and save the hash so cache file is created
                log_debug!("{:?}, cache miss - treating as changed", target_ref.path);
                if !current_hash.is_empty() {
                    cache.insert(source_key.clone(), current_hash.clone());
                    if let Err(e) = save_cache(&cache) {
                        log_error!("Failed to save cache: {}", e);
                    }
                }
                return true;
            }
            log_debug!("{:?}, not changed", target_ref.path);
            log_debug!("current_hash.is_empty() = {:?}", current_hash.is_empty());

            if let Some(target_mut) = self.graph.nodes.get_mut(target_index) {
                let mut new_path = PathBuf::from(BUILD_DIR);
                new_path.push(&filename_lossy_owned);
                new_path.set_extension("o");
                target_mut.path = new_path;
                target_mut.artifact_type = ArtifactType::Object;
            }

            // If unchanged according to cache, skip (return true)
            return false;
        } else {
            log_error!("check_cache: invalid target index");
            return false;
        }
    }
}