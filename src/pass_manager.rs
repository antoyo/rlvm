use Module;
use module::Function;
use ffi::{
    LLVMAddCFGSimplificationPass,
    LLVMAddFunctionInliningPass,
    LLVMAddGVNPass,
    LLVMAddInstructionCombiningPass,
    LLVMAddReassociatePass,
    LLVMCreateFunctionPassManagerForModule,
    LLVMCreatePassManager,
    LLVMDisposePassManager,
    LLVMPassManagerRef,
    LLVMAddPromoteMemoryToRegisterPass,
    LLVMRunFunctionPassManager,
    LLVMRunPassManager,
};

pub struct FunctionPassManager(LLVMPassManagerRef);

impl FunctionPassManager {
    pub fn new_for_module(module: &Module) -> Self {
        unsafe {
            FunctionPassManager(LLVMCreateFunctionPassManagerForModule(module.as_raw()))
        }
    }

    pub fn add_cfg_simplification_pass(&self) {
        unsafe {
            LLVMAddCFGSimplificationPass(self.as_raw());
        }
    }

    pub fn add_gvn_pass(&self) {
        unsafe {
            LLVMAddGVNPass(self.as_raw());
        }
    }

    pub fn add_instruction_combining_pass(&self) {
        unsafe {
            LLVMAddInstructionCombiningPass(self.as_raw());
        }
    }

    pub fn add_promote_memory_to_register_pass(&self) {
        unsafe {
            LLVMAddPromoteMemoryToRegisterPass(self.as_raw());
        }
    }

    pub fn add_reassociate_pass(&self) {
        unsafe {
            LLVMAddReassociatePass(self.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMPassManagerRef {
        self.0
    }

    pub fn run(&self, function: &Function) -> bool {
        unsafe {
            LLVMRunFunctionPassManager(self.as_raw(), function.as_raw()) != 0
        }
    }
}

impl Drop for FunctionPassManager {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassManager(self.as_raw());
        }
    }
}

pub struct ModulePassManager(LLVMPassManagerRef);

impl ModulePassManager {
    pub fn new() -> Self {
        unsafe {
            ModulePassManager(LLVMCreatePassManager())
        }
    }

    pub fn add_function_inlining_pass(&self) {
        unsafe {
            LLVMAddFunctionInliningPass(self.as_raw());
        }
    }

    pub fn as_raw(&self) -> LLVMPassManagerRef {
        self.0
    }

    pub fn run(&self, module: &Module) -> bool {
        unsafe {
            LLVMRunPassManager(self.as_raw(), module.as_raw()) != 0
        }
    }
}

impl Drop for ModulePassManager {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassManager(self.as_raw());
        }
    }
}
