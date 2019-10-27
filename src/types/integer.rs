use Context;
use ffi::{
    LLVMInt32Type,
    LLVMInt32TypeInContext,
    LLVMInt8Type,
};
use super::Type;

pub fn int8() -> Type {
    unsafe {
        Type(LLVMInt8Type())
    }
}

pub fn int32() -> Type {
    unsafe {
        Type(LLVMInt32Type())
    }
}

impl Context {
    pub fn int32(&self) -> Type {
        unsafe {
            Type(LLVMInt32TypeInContext(self.as_raw()))
        }
    }
}
