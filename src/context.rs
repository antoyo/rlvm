use Module;
use ffi::{
    LLVMContextCreate,
    LLVMContextDispose,
    LLVMContextRef,
};

pub struct Context(LLVMContextRef);

impl Context {
    pub fn new() -> Self {
        unsafe {
            Context(LLVMContextCreate())
        }
    }

    pub fn new_module(&self, name: &str) -> Module {
        Module::new_with_name_in_context(name, self)
    }

    pub fn as_raw(&self) -> LLVMContextRef {
        self.0
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Drop context");
        unsafe {
            LLVMContextDispose(self.as_raw());
        }
    }
}
