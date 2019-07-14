use std::ffi::CString;

use basic_block::BasicBlock;
use ffi::{
    LLVMAddFunction,
    LLVMAppendBasicBlock,
    LLVMGetParam,
    LLVMModuleCreateWithName,
    LLVMModuleRef,
    LLVMValueRef,
};
use types::Type;
use value::Value;

pub struct Module(LLVMModuleRef);

impl Module {
    pub fn new_with_name(name: &str) -> Self {
        let cstring = CString::new(name).expect("cstring");
        let module = unsafe { LLVMModuleCreateWithName(cstring.as_ptr()) };
        Self(module)
    }

    pub fn add_function(&self, name: &str, function_type: Type) -> Function {
        let cstring = CString::new(name).expect("cstring");
        let value = unsafe { LLVMAddFunction(self.as_raw(), cstring.as_ptr(), function_type.as_raw()) };
        Function(value)
    }

    pub fn as_raw(&self) -> LLVMModuleRef {
        self.0
    }
}

pub struct Function(LLVMValueRef);

impl Function {
    pub fn append_basic_block(&self, block_name: &str) -> BasicBlock {
        let cstring = CString::new(block_name).expect("cstring");
        unsafe {
            BasicBlock::from_raw(LLVMAppendBasicBlock(self.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.0
    }

    pub fn get_param(&self, index: usize) -> Value {
        unsafe {
            Value::from_raw(LLVMGetParam(self.as_raw(), index as u32))
        }
    }
}
