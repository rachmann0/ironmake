use crate::fundamentals::compiler::{Compiler};

/* 
    build context
    ↓
    compile()
    link()
    run()
*/

pub struct Build<C: Compiler> {
    pub compiler: C,
}
impl<C: Compiler> Build<C> {
    pub fn compile(&self, files: &[&str]) {
        self.compiler.compile(files);
    }
    pub fn link(&self) {
        self.compiler.link();
    }
}