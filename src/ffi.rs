use std::os::raw::{c_char, c_void};

pub type LLVMModuleRef = *mut c_void;
pub type LLVMTypeRef = *mut c_void;
pub type LLVMValueRef = *mut c_void;
pub type LLVMBasicBlockRef = *mut c_void;
pub type LLVMBuilderRef = *mut c_void;
pub type LLVMExecutionEngineRef = *mut c_void;
pub type LLVMGenericValueRef = *mut c_void;
pub type LLVMBool = i32;

#[repr(C)]
pub enum LLVMVerifierFailureAction {
  LLVMAbortProcessAction,
  LLVMPrintMessageAction,
  LLVMReturnStatusAction,
}

#[link(name="LLVM-8")]
extern "C" {
    pub fn LLVMModuleCreateWithName(ModuleID: *const c_char) -> LLVMModuleRef;
    pub fn LLVMInt32Type() -> LLVMTypeRef;
    pub fn LLVMFunctionType(ReturnType: LLVMTypeRef, ParamTypes: *mut LLVMTypeRef, ParamCount: u32, IsVarArg: LLVMBool) -> LLVMTypeRef;
    pub fn LLVMAddFunction(M: LLVMModuleRef, Name: *const c_char, FunctionTy: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMAppendBasicBlock(Fn: LLVMValueRef, Name: *const c_char) -> LLVMBasicBlockRef;
    pub fn LLVMCreateBuilder() -> LLVMBuilderRef;
    pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef);
    pub fn LLVMBuildAdd(builder: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildRet(builder: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetParam(Fn: LLVMValueRef, Index: u32) -> LLVMValueRef;
    pub fn LLVMVerifyModule(M: LLVMModuleRef, Action: LLVMVerifierFailureAction, OutMessage: *mut *mut c_char) -> LLVMBool;
    pub fn LLVMDisposeMessage(Message: *mut c_char);
    pub fn LLVMLinkInMCJIT();
    pub fn LLVM_InitializeNativeTarget() -> LLVMBool;
    pub fn LLVMCreateExecutionEngineForModule(OutEE: *mut LLVMExecutionEngineRef, M: LLVMModuleRef, OutError: *mut *mut c_char) -> LLVMBool;
    pub fn LLVMCreateGenericValueOfInt(Ty: LLVMTypeRef, N: u64, IsSigned: LLVMBool) -> LLVMGenericValueRef;
    pub fn LLVMRunFunction(EE: LLVMExecutionEngineRef, F: LLVMValueRef, NumArgs: u32, Args: *mut LLVMGenericValueRef)-> LLVMGenericValueRef;
    pub fn LLVM_InitializeNativeAsmPrinter() -> LLVMBool;
    pub fn LLVMGetFunctionAddress(EE: LLVMExecutionEngineRef, Name: *const c_char) -> u64;
    pub fn LLVMDisposeBuilder(Builder: LLVMBuilderRef);
    pub fn LLVMDisposeExecutionEngine(EE: LLVMExecutionEngineRef);
    pub fn LLVMDisposeModule(M: LLVMModuleRef);
    pub fn LLVMRemoveModule(EE: LLVMExecutionEngineRef, M: LLVMModuleRef, OutMod: *mut LLVMModuleRef, OutError: *mut *mut c_char) -> LLVMBool;
    pub fn LLVMShutdown();
    pub fn LLVMConstInt(IntTy: LLVMTypeRef, N: u64, SignExtend: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstReal(RealTy: LLVMTypeRef , N: f64) -> LLVMValueRef;
    pub fn LLVMDoubleType() -> LLVMTypeRef;
}
