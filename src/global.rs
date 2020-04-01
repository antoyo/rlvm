use ffi::{
    LLVMSetInitializer,
    LLVMTypeOf,
    LLVMValueRef,
};
use types::Type;
use Value;

#[derive(Clone, Debug)]
pub struct GlobalVariable(LLVMValueRef);

impl GlobalVariable {
    pub fn set_initializer(&self, constant_val: &Value) {
        // FIXME: wrong assertion.
        //debug_assert_eq!(self.get_type(), constant_val.get_type(), "the type of constant_val should match the type of global variable");
        unsafe {
            LLVMSetInitializer(self.as_raw(), constant_val.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMValueRef {
        self.0
    }

    pub fn as_value(&self) -> Value {
        unsafe {
            Value::from_raw(self.as_raw())
        }
    }

    pub unsafe fn from_raw(value: LLVMValueRef) -> Self {
        Self(value)
    }

    pub fn get_type(&self) -> Type {
        unsafe {
            Type::from_raw(LLVMTypeOf(self.as_raw()))
        }
    }
}
