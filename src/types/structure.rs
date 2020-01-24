use ffi::{
    LLVMStructType,
};
use super::Type;

pub fn new(element_types: &[Type], packed: bool) -> Type {
    unsafe {
        Type(LLVMStructType(element_types.as_ptr() as *mut _, element_types.len() as u32, packed as i32))
    }
}
