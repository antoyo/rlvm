use std::os::raw::{c_char, c_void};

pub type LLVMModuleRef = *mut c_void;

#[link(name="LLVM-8")]
extern "C" {
    pub fn LLVMModuleCreateWithName(ModuleID: *const c_char) -> LLVMModuleRef;
}
