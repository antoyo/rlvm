use std::ffi::CString;

use ffi::{
    LLVMConstInt,
    LLVMConstNull,
    LLVMConstReal,
    LLVMConstString,
};
use types::Type;
use Value;

pub fn int(typ: Type, value: u64, sign_extend: bool) -> Value {
    unsafe { Value::from_raw(LLVMConstInt(typ.as_raw(), value, sign_extend as i32)) }
}

pub fn null(typ: Type) -> Value {
    unsafe { Value::from_raw(LLVMConstNull(typ.as_raw())) }
}

pub fn real(typ: Type, value: f64) -> Value {
    unsafe { Value::from_raw(LLVMConstReal(typ.as_raw(), value)) }
}

pub fn string(string: &str, dont_null_terminate: bool) -> Value {
    let cstring = CString::new(string).expect("cstring");
    unsafe { Value::from_raw(LLVMConstString(cstring.as_ptr(), cstring.as_bytes().len() as u32, dont_null_terminate as i32)) }
}
