use std::ffi::CString;

use basic_block::BasicBlock;
use exec_engine::TargetData;
use ffi::{
    LLVMAddFunction,
    LLVMAppendBasicBlock,
    LLVMCountParams,
    LLVMDeleteFunction,
    LLVMDisposeModule,
    LLVMDumpModule,
    LLVMDumpValue,
    LLVMGetEntryBasicBlock,
    LLVMGetNamedFunction,
    LLVMGetParam,
    LLVMModuleCreateWithName,
    LLVMModuleRef,
    LLVMSetDataLayout,
    LLVMSetTarget,
    LLVMValueRef,
    LLVMVerifyFunction,
};
use target::TargetTriple;
use types::Type;
use value::Value;
use VerifierFailureAction;

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

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.as_raw()) }
    }

    pub fn get_named_function(&self, name: &str) -> Option<Function> {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            let value = LLVMGetNamedFunction(self.as_raw(), cstring.as_ptr());
            if value.is_null() {
                None
            }
            else {
                Some(Function(value))
            }
        }
    }

    pub fn set_data_layout(&self, data_layout: TargetData) {
        unsafe {
            LLVMSetDataLayout(self.as_raw(), data_layout.as_raw() as *const _);
        }
    }

    pub fn set_target(&self, target: TargetTriple) {
        unsafe {
            LLVMSetTarget(self.as_raw(), target.as_raw())
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.as_raw());
        }
    }
}

pub struct Function(LLVMValueRef);

impl Function {
    pub fn from_raw(value: LLVMValueRef) -> Self {
        Self(value)
    }

    pub fn append_basic_block(&self, block_name: &str) -> BasicBlock {
        let cstring = CString::new(block_name).expect("cstring");
        unsafe {
            BasicBlock::from_raw(LLVMAppendBasicBlock(self.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.0
    }

    pub fn delete(&self) {
        unsafe { LLVMDeleteFunction(self.as_raw()); }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpValue(self.as_raw()); }
    }

    pub fn get_entry_basic_block(&self) -> BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetEntryBasicBlock(self.as_raw())) }
    }

    pub fn get_param(&self, index: usize) -> Value {
        unsafe {
            Value::from_raw(LLVMGetParam(self.as_raw(), index as u32))
        }
    }

    pub fn param_count(&self) -> usize {
        unsafe { LLVMCountParams(self.as_raw()) as usize }
    }

    pub fn verify(&self, action: VerifierFailureAction) -> bool {
        unsafe { LLVMVerifyFunction(self.as_raw(), action.as_raw()) != 0 }
    }
}
