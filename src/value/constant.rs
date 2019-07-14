use ffi::{
    LLVMConstInt,
    LLVMConstReal,
};
use types::Type;
use Value;

pub fn int(typ: Type, value: u64, sign_extend: bool) -> Value {
    unsafe { Value::from_raw(LLVMConstInt(typ.as_raw(), value, sign_extend as i32)) }
}

pub fn real(typ: Type, value: f64) -> Value {
    unsafe { Value::from_raw(LLVMConstReal(typ.as_raw(), value)) }
}
