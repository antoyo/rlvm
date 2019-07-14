pub mod function;
pub mod integer;

use ffi::LLVMTypeRef;

pub use self::integer::*;

pub struct Type(LLVMTypeRef);

impl Type {
    pub fn as_raw(&self) -> LLVMTypeRef {
        self.0
    }
}
