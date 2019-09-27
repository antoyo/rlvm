use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

use assert_llvm_initialized;
use ffi::{
    LLVMAddModule,
    LLVMCreateExecutionEngineForModule,
    LLVMDisposeExecutionEngine,
    LLVMDisposeMessage,
    LLVMDisposeTargetData,
    LLVMExecutionEngineRef,
    LLVMGetExecutionEngineTargetData,
    LLVMGetFunctionAddress,
    LLVMLinkInMCJIT,
    LLVMRemoveModule,
    LLVMTargetDataRef,
};
use module::Module;

pub fn link_mcjit() {
    unsafe {
        LLVMLinkInMCJIT();
    }
}

pub struct TargetData(LLVMTargetDataRef);

impl TargetData {
    pub fn as_raw(&self) -> LLVMTargetDataRef {
        self.0
    }

    pub fn from_raw(target_data: LLVMTargetDataRef) -> Self {
        Self(target_data)
    }
}

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetData(self.as_raw());
        }
    }
}

pub struct ExecutionEngine(LLVMExecutionEngineRef);

impl ExecutionEngine {
    pub fn new_for_module(module: &Module) -> Result<Self, String> {
        assert_llvm_initialized();

        let mut engine: LLVMExecutionEngineRef = ptr::null_mut();
        let mut error = ptr::null_mut();
        unsafe {
            if LLVMCreateExecutionEngineForModule(&mut engine, module.as_raw(), &mut error) != 0 {
                assert_ne!(error, ptr::null_mut());
                let msg = CStr::from_ptr(error);
                let error_msg = msg.to_str().expect("error message").to_string();
                LLVMDisposeMessage(error);
                Err(error_msg)
            }
            else {
                Ok(ExecutionEngine(engine))
            }
        }
    }

    pub fn add_module(&self, module: &Module) {
        unsafe {
            LLVMAddModule(self.as_raw(), module.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMExecutionEngineRef {
        self.0
    }

    pub fn get_function_address(&self, name: &str) -> Option<FunctionAddress> {
        let cstring = CString::new(name).expect("cstring");
        let address =
            unsafe {
                LLVMGetFunctionAddress(self.as_raw(), cstring.as_ptr())
            };
        if address == 0 {
            None
        }
        else {
            Some(FunctionAddress(address))
        }
    }

    pub fn get_target_data(&self) -> TargetData {
        unsafe {
            TargetData::from_raw(LLVMGetExecutionEngineTargetData(self.as_raw()))
        }
    }

    pub fn remove_module(&self, module: &Module) -> Result<(), String> {
        let mut error = ptr::null_mut();
        unsafe {
            let mut new_mod = mem::zeroed();
            if LLVMRemoveModule(self.as_raw(), module.as_raw(), &mut new_mod, &mut error) != 0 {
                assert_ne!(error, ptr::null_mut());
                let msg = CStr::from_ptr(error);
                let error_msg = msg.to_str().expect("error message").to_string();
                LLVMDisposeMessage(error);
                Err(error_msg)
            }
            else {
                Ok(())
            }
        }
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.as_raw())
        }
    }
}

pub struct FunctionAddress(u64);

impl FunctionAddress {
    pub unsafe fn cast0(&self) -> fn() {
        mem::transmute(self.0)
    }

    pub unsafe fn cast0_ret<T>(&self) -> fn() -> T {
        mem::transmute(self.0)
    }

    pub unsafe fn cast1<T>(&self) -> fn(T) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast1_ret<S, T>(&self) -> fn(T) -> S {
        mem::transmute(self.0)
    }

    pub unsafe fn cast2<S, T>(&self) -> fn(T, S) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast2_ret<R, S, T>(&self) -> fn(T, S) -> R {
        mem::transmute(self.0)
    }

    pub unsafe fn cast3<R, S, T>(&self) -> fn(T, S, R) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast3_ret<P, R, S, T>(&self) -> fn(T, S, R) -> P {
        mem::transmute(self.0)
    }

    pub unsafe fn cast4<P, R, S, T>(&self) -> fn(T, S, R, P) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast4_ret<O, P, R, S, T>(&self) -> fn(T, S, R, P) -> O {
        mem::transmute(self.0)
    }

    pub unsafe fn cast5<O, P, R, S, T>(&self) -> fn(T, S, R, P, O) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast5_ret<N, O, P, R, S, T>(&self) -> fn(T, S, R, P, O) -> N {
        mem::transmute(self.0)
    }

    pub unsafe fn cast6<N, O, P, R, S, T>(&self) -> fn(T, S, R, P, O, N) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast6_ret<M, N, O, P, R, S, T>(&self) -> fn(T, S, R, P, O, N) -> M {
        mem::transmute(self.0)
    }

    pub unsafe fn cast7<M, N, O, P, R, S, T>(&self) -> fn(T, S, R, P, O, N, M) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast7_ret<L, M, N, O, P, R, S, T>(&self) -> fn(T, S, R, P, O, N, M) -> L {
        mem::transmute(self.0)
    }

    pub unsafe fn cast8<L, M, N, O, P, R, S, T>(&self) -> fn(T, S, R, P, O, N, M, L) {
        mem::transmute(self.0)
    }

    pub unsafe fn cast8_ret<K, L, M, N, O, P, R, S, T>(&self) -> fn(T, S, R, P, O, N, M, L) -> K {
        mem::transmute(self.0)
    }
}
