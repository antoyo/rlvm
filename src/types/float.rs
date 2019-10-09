use Context;
use ffi::{
    LLVMDoubleType,
    LLVMDoubleTypeInContext,
};
use super::Type;

pub fn double() -> Type {
    unsafe {
        Type(LLVMDoubleType())
    }
}

impl Context {
    pub fn double(&self) -> Type {
        unsafe {
            Type(LLVMDoubleTypeInContext(self.as_raw()))
        }
    }
}
