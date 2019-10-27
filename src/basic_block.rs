use std::ffi::CString;

use {Context, Value};
use ffi::{
    LLVMAppendBasicBlock,
    LLVMAppendBasicBlockInContext,
    LLVMBasicBlockRef,
    LLVMCreateBasicBlockInContext,
    LLVMGetBasicBlockParent,
    LLVMGetFirstInstruction,
};
use module::Function;

pub struct BasicBlock(LLVMBasicBlockRef);

impl BasicBlock {
    pub fn append(function: &Function, name: &str) -> Self {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Self(LLVMAppendBasicBlock(function.as_raw(), cstring.as_ptr()))
        }
    }

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

    pub fn get_first_instruction(&self) -> Value {
        unsafe {
            Value::from_raw(LLVMGetFirstInstruction(self.as_raw()))
        }
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
