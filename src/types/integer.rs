use ffi::LLVMInt32Type;
use super::Type;

pub fn int32() -> Type {
    unsafe {
        Type(LLVMInt32Type())
    }
}
