use crate::builder::compiler::{Compiler};
use crate::ds::artifact::{ArtifactType};
use crate::ds::graph::Graph;

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

        if dependancy_indexes.is_empty() {
            if let Some(artifact) = self.graph.nodes.get(target_index) {
                match artifact.artifact_type {
                    ArtifactType::Source => {
                        self.compile(target_index);
                    },
                    _ => {}
                }
            }
            
        } else {
            for dep_idx in dependancy_indexes {
                // let built = self.graph.nodes[dep_idx].is_built;
                // if !built {
                    self.build(dep_idx);
                // }
            }
            // link target after dependencies
            // if !self.graph.nodes[target_index].is_built {
                self.link(target_index);
            // }
        }
    }
}