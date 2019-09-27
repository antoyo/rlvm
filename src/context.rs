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

    pub fn as_raw(&self) -> LLVMContextRef {
        self.0
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.as_raw());
        }
    }
}
