use ffi::{
    LLVMArrayType,
};
use super::Type;

pub fn array(element_type: Type, count: usize) -> Type {
    unsafe {
        Type(LLVMArrayType(element_type.as_raw(), count as u32))
    }
}
