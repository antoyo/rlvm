use std::ffi::CString;

use Context;
use ffi::{
    LLVMAppendBasicBlockInContext,
    LLVMBasicBlockRef,
    LLVMCreateBasicBlockInContext,
    LLVMGetBasicBlockParent,
};
use module::Function;

pub struct BasicBlock(LLVMBasicBlockRef);

impl BasicBlock {
    pub fn append_in_context(context: &Context, function: &Function, name: &str) -> Self {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Self(LLVMAppendBasicBlockInContext(context.as_raw(), function.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn new_in_context(context: &Context, name: &str) -> Self {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Self(LLVMCreateBasicBlockInContext(context.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn from_raw(basic_block: LLVMBasicBlockRef) -> Self {
        Self(basic_block)
    }

    pub fn get_parent(&self) -> Function {
        unsafe {
            Function::from_raw(LLVMGetBasicBlockParent(self.as_raw()))
        }
    }

    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self.0
    }
}
