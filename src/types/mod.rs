pub mod array;
pub mod float;
pub mod function;
pub mod integer;
pub mod pointer;

use ffi::LLVMTypeRef;

pub use self::float::*;
pub use self::integer::*;

#[derive(Clone)]
pub struct Type(LLVMTypeRef);

impl Type {
    pub fn as_raw(&self) -> LLVMTypeRef {
        self.0
    }
}
