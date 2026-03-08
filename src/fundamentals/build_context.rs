use crate::fundamentals::compiler::{Compiler};
use crate::fundamentals::artifact::{Artifact};

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
    pub fn link(&self, artifacts: &[Artifact])->Result<String, String>{
        let extra_flags:&str = self.mode.flag_value();
        self.compiler.link(extra_flags, artifacts)
    }
}