use crate::builder::compiler::{Compiler};
use crate::graph::artifact::{self, Artifact, ArtifactType};
use crate::graph::DAG::Graph;

use crate::{log_error, log_info};

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
    pub mode: Modes
}
impl<C: Compiler> Build<C> {
    pub fn compile(&self, artifacts: &[Artifact])->Result<String, String> {
        let extra_flags:&str = self.mode.flag_value();
        self.compiler.compile(extra_flags, artifacts)
    }
    pub fn compile2(&self, artifacts: &[&mut Artifact])->Result<String, String> {
        let extra_flags:&str = self.mode.flag_value();
        self.compiler.compile2(extra_flags, artifacts)
    }
    pub fn link(&self, artifacts: &[Artifact])->Result<String, String>{
        let extra_flags:&str = self.mode.flag_value();
        self.compiler.link(extra_flags, artifacts)
    }

    pub fn build(&self, target:&mut Artifact, graph:Graph){
        if target.dependancies.len() == 0 {
            //* no dependancy (end of graph) */
            match self.compile2(&[target]) {
                Ok(output) => {
                    log_info!("Compilation succeeded:\n{}", output)
                }
                Err(error) => {
                    log_error!("Compilation failed:\n{}", error);
                }
            }
        }

        for dependancy in target.dependancies.iter() {
            if dependancy.is_built {
            } else {
                // self.build(dependancy);
            }
        }

        // if artifact.artifact_type == ArtifactType::Binary {
        //     match self.compile(&artifact.dependancies) {
        //         Ok(output) => {
        //             log_info!("Compilation succeeded:\n{}", output)
        //         }
        //         Err(error) => {
        //             log_error!("Compilation failed:\n{}", error);
        //         }
        //     }
        // };
        // artifact.clone()
    }
}