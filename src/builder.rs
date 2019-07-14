use std::ffi::CString;

use basic_block::BasicBlock;
use ffi::{
    LLVMBuildAdd,
    LLVMBuilderRef,
    LLVMBuildRet,
    LLVMCreateBuilder,
    LLVMDisposeBuilder,
    LLVMPositionBuilderAtEnd,
};
use value::Value;

pub struct Builder(LLVMBuilderRef);

impl Builder {
    pub fn new() -> Self {
        unsafe { Builder(LLVMCreateBuilder()) }
    }

    pub fn add(&self, op1: Value, op2: Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildAdd(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self.0
    }

    pub fn position_at_end(&self, entry: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), entry.as_raw());
        }
    }

    pub fn ret(&self, value: Value) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildRet(self.as_raw(), value.as_raw()))
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.as_raw());
        }
    }
}
