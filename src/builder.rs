use std::ffi::CString;

use basic_block::BasicBlock;
use ffi::{
    LLVMBuildAdd,
    LLVMBuildCall,
    LLVMBuildFAdd,
    LLVMBuildFCmp,
    LLVMBuildFMul,
    LLVMBuildFSub,
    LLVMBuilderRef,
    LLVMBuildRet,
    LLVMBuildUIToFP,
    LLVMCreateBuilder,
    LLVMDisposeBuilder,
    LLVMPositionBuilderAtEnd,
    LLVMRealPredicate,
};
use module::Function;
use types::Type;
use value::Value;

pub enum RealPredicate {
    False,
    OrderedEqual,
    OrderedGreaterThan,
    OrderedGreaterThanOrEqual,
    OrderedLesserThan,
    OrderedLesserThanOrEqual,
    OrderedNotEqual,
    Ordered,
    Unordered,
    UnorderedEqual,
    UnorderedGreaterThan,
    UnorderedGreaterThanOrEqual,
    UnorderedLesserThan,
    UnorderedLesserThanOrEqual,
    UnorderedNotEqual,
    True,
}

impl RealPredicate {
    fn as_raw(&self) -> LLVMRealPredicate {
        match *self {
            RealPredicate::False => LLVMRealPredicate::LLVMRealPredicateFalse,
            RealPredicate::OrderedEqual => LLVMRealPredicate::LLVMRealOEQ,
            RealPredicate::OrderedGreaterThan => LLVMRealPredicate::LLVMRealOGT,
            RealPredicate::OrderedGreaterThanOrEqual => LLVMRealPredicate::LLVMRealOGE,
            RealPredicate::OrderedLesserThan => LLVMRealPredicate::LLVMRealOLT,
            RealPredicate::OrderedLesserThanOrEqual => LLVMRealPredicate::LLVMRealOLE,
            RealPredicate::OrderedNotEqual => LLVMRealPredicate::LLVMRealONE,
            RealPredicate::Ordered => LLVMRealPredicate::LLVMRealORD,
            RealPredicate::Unordered => LLVMRealPredicate::LLVMRealUNO,
            RealPredicate::UnorderedEqual => LLVMRealPredicate::LLVMRealUEQ,
            RealPredicate::UnorderedGreaterThan => LLVMRealPredicate::LLVMRealUGT,
            RealPredicate::UnorderedGreaterThanOrEqual => LLVMRealPredicate::LLVMRealUGE,
            RealPredicate::UnorderedLesserThan => LLVMRealPredicate::LLVMRealULT,
            RealPredicate::UnorderedLesserThanOrEqual => LLVMRealPredicate::LLVMRealULE,
            RealPredicate::UnorderedNotEqual => LLVMRealPredicate::LLVMRealUNE,
            RealPredicate::True => LLVMRealPredicate::LLVMRealPredicateTrue,
        }
    }
}

pub struct Builder(LLVMBuilderRef);

impl Builder {
    pub fn new() -> Self {
        unsafe { Builder(LLVMCreateBuilder()) }
    }

    pub fn add(&self, op1: Value, op2: Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildAdd(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self.0
    }

    pub fn call(&self, func: Function, args: &[Value], name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildCall(self.as_raw(), func.as_raw(), args.as_ptr() as *mut _, args.len() as u32, cstring.as_ptr()))
        }
    }

    pub fn fadd(&self, op1: Value, op2: Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFAdd(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn fcmp(&self, op: RealPredicate, op1: Value, op2: Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFCmp(self.as_raw(), op.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn fmul(&self, op1: Value, op2: Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFMul(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn fsub(&self, op1: Value, op2: Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFSub(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn position_at_end(&self, entry: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), entry.as_raw());
        }
    }

    pub fn ret(&self, value: Value) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildRet(self.as_raw(), value.as_raw()))
        }
    }

    pub fn unsigned_int_to_floating_point(&self, value: Value, dest_type: Type, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildUIToFP(self.as_raw(), value.as_raw(), dest_type.as_raw(), cstring.as_ptr()))
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.as_raw());
        }
    }
}
