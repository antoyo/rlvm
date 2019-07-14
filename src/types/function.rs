use ffi::LLVMFunctionType;
use super::Type;

pub fn new(return_type: Type, param_types: &[Type], variadic: bool) -> Type {
    unsafe {
        Type(LLVMFunctionType(return_type.0, param_types.as_ptr() as *mut _, param_types.len() as u32, variadic as i32))
    }
}
