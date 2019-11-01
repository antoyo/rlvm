pub mod constant;

use std::ffi::CString;

use BasicBlock;
use ffi::{
    LLVMAddIncoming,
    LLVMSetValueName2,
    LLVMValueRef,
};

#[derive(Clone, Debug)]
pub struct Value(LLVMValueRef);

impl Value {
    pub(crate) fn from_raw(value_ref: LLVMValueRef) -> Self {
        Value(value_ref)
    }

    // TODO: change the API so that Builder::phi() takes this array (like the OCaml binding)?
    pub fn add_incoming(&self, incoming: &[(&Value, &BasicBlock)]) {
        let mut incoming_values: Vec<_> = incoming.iter().map(|(value, _)| value.as_raw()).collect();
        let mut incoming_blocks: Vec<_> = incoming.iter().map(|(_, block)| block.as_raw()).collect();
        unsafe {
            LLVMAddIncoming(self.as_raw(), incoming_values.as_mut_ptr(), incoming_blocks.as_mut_ptr(), incoming.len() as u32);
        }
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
