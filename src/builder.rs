use std::ffi::CString;
use std::os::raw::c_uint;
use std::ptr;

use Context;
use basic_block::BasicBlock;
use ffi::{
    LLVMBuildAdd,
    LLVMBuildAlloca,
    LLVMBuildBitCast,
    LLVMBuildBr,
    LLVMBuildCall,
    LLVMBuildCondBr,
    LLVMBuildFAdd,
    LLVMBuildFCmp,
    LLVMBuildFMul,
    LLVMBuildFSub,
    LLVMBuildGEP2,
    LLVMBuildGlobalStringPtr,
    LLVMBuildICmp,
    LLVMBuildLoad2,
    LLVMBuildMemMove,
    LLVMBuildMemSet,
    LLVMBuildPhi,
    LLVMBuilderRef,
    LLVMBuildRet,
    LLVMBuildStore,
    LLVMBuildSub,
    LLVMBuildUIToFP,
    LLVMCreateBuilder,
    LLVMCreateBuilderInContext,
    LLVMDisposeBuilder,
    LLVMGetInsertBlock,
    LLVMIntPredicate,
    LLVMPositionBuilder,
    LLVMPositionBuilderAtEnd,
    LLVMRealPredicate,
};
use module::Function;
use types::Type;
use value::Value;

pub enum IntPredicate {
  Equal,
  NotEqual,
  SignedGreaterThan,
  SignedGreaterThanOrEqual,
  SignedLesserThan,
  SignedLesserThanOrEqual,
  UnsignedGreaterThan,
  UnsignedGreaterThanOrEqual,
  UnsignedLesserThan,
  UnsignedLesserThanOrEqual,
}

impl IntPredicate {
    fn as_raw(&self) -> LLVMIntPredicate {
        match *self {
            Self::Equal => LLVMIntPredicate::LLVMIntEQ,
            Self::NotEqual => LLVMIntPredicate::LLVMIntNE,
            Self::SignedGreaterThan => LLVMIntPredicate::LLVMIntUGT,
            Self::SignedGreaterThanOrEqual => LLVMIntPredicate::LLVMIntUGE,
            Self::SignedLesserThan => LLVMIntPredicate::LLVMIntULT,
            Self::SignedLesserThanOrEqual => LLVMIntPredicate::LLVMIntULE,
            Self::UnsignedGreaterThan => LLVMIntPredicate::LLVMIntSGT,
            Self::UnsignedGreaterThanOrEqual => LLVMIntPredicate::LLVMIntSGE,
            Self::UnsignedLesserThan => LLVMIntPredicate::LLVMIntSLT,
            Self::UnsignedLesserThanOrEqual => LLVMIntPredicate::LLVMIntSLE,
        }
    }
}

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

    pub fn new_in_context(context: &Context) -> Self {
        unsafe {
            Builder(LLVMCreateBuilderInContext(context.as_raw()))
        }
    }

    pub fn add(&self, op1: &Value, op2: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildAdd(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn alloca(&self, typ: Type, name: &str) -> Value {
        assert!(self.get_insert_block().is_some(), "position the builder before calling alloca");
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildAlloca(self.as_raw(), typ.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn as_raw(&self) -> LLVMBuilderRef {
        self.0
    }

    pub fn bitcast(&self, value: &Value, dest_type: Type, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildBitCast(self.as_raw(), value.as_raw(), dest_type.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn br(&self, basic_block: &BasicBlock) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildBr(self.as_raw(), basic_block.as_raw()))
        }
    }

    pub fn call(&self, func: Function, args: &[Value], name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildCall(self.as_raw(), func.as_raw(), args.as_ptr() as *mut _, args.len() as u32, cstring.as_ptr()))
        }
    }

    pub fn cond_br(&self, if_: &Value, then: &BasicBlock, else_block: &BasicBlock) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildCondBr(self.as_raw(), if_.as_raw(), then.as_raw(), else_block.as_raw()))
        }
    }

    pub fn fadd(&self, op1: &Value, op2: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFAdd(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn fcmp(&self, op: RealPredicate, op1: &Value, op2: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFCmp(self.as_raw(), op.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn icmp(&self, predicate: IntPredicate, op1: &Value, op2: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildICmp(self.as_raw(), predicate.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn fmul(&self, op1: &Value, op2: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFMul(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn fsub(&self, op1: &Value, op2: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildFSub(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn gep(&self, typ: &Type, pointer: &Value, indices: &[Value], name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildGEP2(self.as_raw(), typ.as_raw(), pointer.as_raw(), indices.as_ptr() as *mut _, indices.len() as u32, cstring.as_ptr()))
        }
    }

    pub fn get_insert_block(&self) -> Option<BasicBlock> {
        unsafe {
            let basic_block = LLVMGetInsertBlock(self.as_raw());
            if basic_block.is_null() {
                return None;
            }
            Some(BasicBlock::from_raw(basic_block))
        }
    }

    pub fn global_string_ptr(&self, string: &str, name: &str) -> Value {
        assert!(self.get_insert_block().is_some(), "position the builder before creating a global string pointer");
        let string = CString::new(string).expect("cstring");
        let name = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildGlobalStringPtr(self.as_raw(), string.as_ptr(), name.as_ptr()))
        }
    }

    pub fn load(&self, typ: Type, value: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildLoad2(self.as_raw(), typ.as_raw(), value.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn mem_move(&self, dest: &Value, dest_align: usize, src: &Value, src_align: usize, size: &Value) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildMemMove(self.as_raw(), dest.as_raw(), dest_align as c_uint, src.as_raw(), src_align as c_uint, size.as_raw()))
        }
    }

    pub fn mem_set(&self, ptr: &Value, value: &Value, len: &Value, align: usize) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildMemSet(self.as_raw(), ptr.as_raw(), value.as_raw(), len.as_raw(), align as c_uint))
        }
    }

    pub fn phi(&self, typ: Type, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildPhi(self.as_raw(), typ.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn position(&self, block: &BasicBlock, instruction: &Value) {
        unsafe {
            LLVMPositionBuilder(self.as_raw(), block.as_raw(), instruction.as_raw());
        }
    }

    pub fn position_at_end(&self, entry: &BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_raw(), entry.as_raw());
        }
    }

    pub fn ret(&self, value: &Value) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildRet(self.as_raw(), value.as_raw()))
        }
    }

    pub fn ret_no_value(&self) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildRet(self.as_raw(), ptr::null_mut()))
        }
    }

    pub fn store(&self, value: &Value, pointer: &Value) -> Value {
        unsafe {
            Value::from_raw(LLVMBuildStore(self.as_raw(), value.as_raw(), pointer.as_raw()))
        }
    }

    pub fn sub(&self, op1: &Value, op2: &Value, name: &str) -> Value {
        let cstring = CString::new(name).expect("cstring");
        unsafe {
            Value::from_raw(LLVMBuildSub(self.as_raw(), op1.as_raw(), op2.as_raw(), cstring.as_ptr()))
        }
    }

    pub fn unsigned_int_to_floating_point(&self, value: &Value, dest_type: Type, name: &str) -> Value {
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
