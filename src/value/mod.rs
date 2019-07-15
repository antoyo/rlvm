pub mod constant;

use std::ffi::CString;

use ffi::{
    LLVMSetValueName2,
    LLVMValueRef,
};

#[derive(Clone)]
pub struct Value(LLVMValueRef);

impl Value {
    pub(crate) fn from_raw(value_ref: LLVMValueRef) -> Self {
        Value(value_ref)
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.0
    }

    pub fn set_name(&self, name: &str) {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            LLVMSetValueName2(self.as_raw(), cstring.as_ptr(), name.as_bytes().len());
        }
    }
}
