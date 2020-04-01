use std::ffi::CString;
use std::os::raw::c_uint;

use ffi::{
    LLVMConstArray,
    LLVMConstInt,
    LLVMConstNull,
    LLVMConstReal,
    LLVMConstString,
};
use types::Type;
use Value;

pub fn array(element_type: &Type, constant_values: &[Value]) -> Value {
    // TODO: avoid doing a collect()?
    let mut values: Vec<_> = constant_values.iter().map(|value| value.as_raw()).collect();
    unsafe { Value::from_raw(LLVMConstArray(element_type.as_raw(), values.as_mut_ptr(), constant_values.len() as c_uint)) }
}

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
