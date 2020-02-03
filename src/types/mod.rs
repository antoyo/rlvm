pub mod array;
pub mod float;
pub mod function;
pub mod integer;
pub mod pointer;
pub mod structure;

use std::ffi::CStr;
use std::fmt::{self, Debug, Formatter};

use ffi::{
    LLVMAlignOf,
    LLVMDisposeMessage,
    LLVMDumpType,
    LLVMGetElementType,
    LLVMPrintTypeToString,
    LLVMSizeOf,
    LLVMTypeRef,
    LLVMVoidType,
};

use Value;
pub use self::float::*;
pub use self::integer::*;

#[derive(Clone)]
pub struct Type(LLVMTypeRef);

impl Debug for Type {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}

impl Type {
    pub fn as_raw(&self) -> LLVMTypeRef {
        self.0
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpType(self.as_raw())
        }
    }

    pub fn element_type(&self) -> Type {
        unsafe {
            Type::from_raw(LLVMGetElementType(self.as_raw()))
        }
    }

    pub unsafe fn from_raw(typ: LLVMTypeRef) -> Self {
        Type(typ)
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let cstring = LLVMPrintTypeToString(self.as_raw());
            let result = CStr::from_ptr(cstring).to_str().expect("to_str").to_string();
            LLVMDisposeMessage(cstring);
            result
        }
    }
}

impl PartialEq<Type> for Type {
    fn eq(&self, typ: &Type) -> bool {
        self.0 == typ.0
    }
}

impl<'a> PartialEq<Type> for &'a Type {
    fn eq(&self, typ: &Type) -> bool {
        self.0 == typ.0
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
