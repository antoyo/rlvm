use ffi::{
    LLVMConstInt,
    LLVMConstNull,
    LLVMConstReal,
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
