pub mod array;
pub mod float;
pub mod function;
pub mod integer;
pub mod pointer;
pub mod structure;

use ffi::{
    LLVMAlignOf,
    LLVMDumpType,
    LLVMSizeOf,
    LLVMTypeRef,
    LLVMVoidType,
};

use Value;
pub use self::float::*;
pub use self::integer::*;

#[derive(Clone)]
pub struct Type(LLVMTypeRef);

impl Type {
    pub fn as_raw(&self) -> LLVMTypeRef {
        self.0
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpType(self.as_raw())
        }
    }

    pub unsafe fn from_raw(typ: LLVMTypeRef) -> Self {
        Type(typ)
    }
}

pub fn void() -> Type {
    unsafe {
        Type(LLVMVoidType())
    }
}

pub fn align_of(typ: &Type) -> Value {
    unsafe {
        Value::from_raw(LLVMAlignOf(typ.as_raw()))
    }
}

pub fn size_of(typ: &Type) -> Value {
    unsafe {
        Value::from_raw(LLVMSizeOf(typ.as_raw()))
    }
}
