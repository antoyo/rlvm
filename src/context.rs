use ffi::{
    LLVMContextCreate,
    LLVMContextRef,
};

pub struct Context(LLVMContextRef);

impl Context {
    pub fn new() -> Self {
        unsafe {
            Context(LLVMContextCreate())
        }
    }

    pub fn as_raw(&self) -> LLVMContextRef {
        self.0
    }
}
