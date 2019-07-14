use ffi::LLVMDoubleType;
use super::Type;

pub fn double() -> Type {
    unsafe {
        Type(LLVMDoubleType())
    }
}
