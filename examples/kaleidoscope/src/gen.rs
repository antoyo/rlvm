use std::ffi::CString;

use rlvm::LLVMModuleCreateWithName;

use ast::{Function, Prototype};
use error::Result;

pub struct Generator {
}

impl Generator {
    pub fn new() -> Self {
        let cstring = CString::new("module").expect("cstring");
        unsafe {
            let module = LLVMModuleCreateWithName(cstring.as_ptr());
        }
        Self {
        }
    }

    pub fn function(&self, function: Function) -> Result<fn() -> f64> {
        Ok(|| 0.0)
    }

    pub fn prototype(&self, prototype: &Prototype) -> Result<i32> {
        Ok(0)
    }
}
