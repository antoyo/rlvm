use ffi::{
    LLVMPointerType,
};
use super::Type;

pub fn new(element_type: Type, address_space: usize) -> Type {
    unsafe {
        Type(LLVMPointerType(element_type.as_raw(), address_space as u32))
    }
}
