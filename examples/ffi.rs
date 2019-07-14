extern crate rlvm;

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

use rlvm::ffi::{
    LLVMAddFunction,
    LLVMAppendBasicBlock,
    LLVMBuildAdd,
    LLVMBuildRet,
    LLVMCreateBuilder,
    LLVMCreateExecutionEngineForModule,
    LLVMDisposeBuilder,
    LLVMDisposeExecutionEngine,
    LLVMDisposeMessage,
    LLVMExecutionEngineRef,
    LLVMFunctionType,
    LLVMGetFunctionAddress,
    LLVMGetParam,
    LLVM_InitializeNativeAsmPrinter,
    LLVM_InitializeNativeTarget,
    LLVMInt32Type,
    LLVMLinkInMCJIT,
    LLVMModuleCreateWithName,
    LLVMPositionBuilderAtEnd,
    LLVMShutdown,
    LLVMVerifyModule,
    LLVMVerifierFailureAction,
};

fn main() {
    unsafe {
        let cstring = CString::new("module").expect("cstring");
        let module = LLVMModuleCreateWithName(cstring.as_ptr());
        let mut param_types = [LLVMInt32Type(), LLVMInt32Type()];
        let return_type = LLVMFunctionType(LLVMInt32Type(), param_types.as_mut_ptr(), 2, 0);
        let cstring = CString::new("sum").expect("cstring");
        let sum = LLVMAddFunction(module, cstring.as_ptr(), return_type);

        let cstring = CString::new("entry").expect("cstring");
        let entry = LLVMAppendBasicBlock(sum, cstring.as_ptr());

        let builder = LLVMCreateBuilder();
        LLVMPositionBuilderAtEnd(builder, entry);

        let cstring = CString::new("temp").expect("cstring");
        let temp = LLVMBuildAdd(builder, LLVMGetParam(sum, 0), LLVMGetParam(sum, 1), cstring.as_ptr());
        LLVMBuildRet(builder, temp);

        let mut error = ptr::null_mut();
        LLVMVerifyModule(module, LLVMVerifierFailureAction::LLVMAbortProcessAction, &mut error);
        LLVMDisposeMessage(error);

        LLVMLinkInMCJIT();
        LLVM_InitializeNativeAsmPrinter();
        LLVM_InitializeNativeTarget();
        let mut engine: LLVMExecutionEngineRef = ptr::null_mut();
        error = ptr::null_mut();
        if LLVMCreateExecutionEngineForModule(&mut engine, module, &mut error) != 0 {
            panic!("failed to create execution engine");
        }
        if error != ptr::null_mut() {
            LLVMDisposeMessage(error);
            let msg = CStr::from_ptr(error);
            panic!("error: {}", msg.to_str().expect("error message"));
        }

        let cstring = CString::new("sum").expect("cstring");
        let sum = LLVMGetFunctionAddress(engine, cstring.as_ptr());
        let sum: fn(i32, i32) -> i32 = mem::transmute(sum);
        println!("{}", sum(40, 2));

        LLVMDisposeBuilder(builder);
        LLVMDisposeExecutionEngine(engine);
        LLVMShutdown();
    }
}
