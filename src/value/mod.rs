pub mod constant;

use ffi::LLVMValueRef;

pub struct Value(LLVMValueRef);

impl Value {
    pub(crate) fn from_raw(value_ref: LLVMValueRef) -> Self {
        Value(value_ref)
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.0
    }
}
