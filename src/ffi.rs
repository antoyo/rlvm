use std::os::raw::{
    c_char,
    c_uint,
    c_void,
};

pub type LLVMModuleRef = *mut c_void;
pub type LLVMTypeRef = *mut c_void;
pub type LLVMValueRef = *mut c_void;
pub type LLVMBasicBlockRef = *mut c_void;
pub type LLVMBuilderRef = *mut c_void;
pub type LLVMExecutionEngineRef = *mut c_void;
pub type LLVMGenericValueRef = *mut c_void;
pub type LLVMBool = i32;
pub type LLVMPassManagerRef = *mut c_void;
pub type LLVMContextRef = *mut c_void;
pub type LLVMTargetDataRef = *mut c_void;
pub type LLVMTargetRef = *mut c_void;
pub type LLVMTargetMachineRef = *mut c_void;

#[repr(C)]
pub enum LLVMCodeGenFileType {
    LLVMAssemblyFile,
    LLVMObjectFile
}

#[repr(C)]
pub enum LLVMVerifierFailureAction {
    LLVMAbortProcessAction,
    LLVMPrintMessageAction,
    LLVMReturnStatusAction,
}

#[repr(C)]
pub enum LLVMRealPredicate {
    LLVMRealPredicateFalse,
    LLVMRealOEQ,
    LLVMRealOGT,
    LLVMRealOGE,
    LLVMRealOLT,
    LLVMRealOLE,
    LLVMRealONE,
    LLVMRealORD,
    LLVMRealUNO,
    LLVMRealUEQ,
    LLVMRealUGT,
    LLVMRealUGE,
    LLVMRealULT,
    LLVMRealULE,
    LLVMRealUNE,
    LLVMRealPredicateTrue,
}

#[repr(C)]
pub enum LLVMIntPredicate {
  LLVMIntEQ = 32,
  LLVMIntNE,
  LLVMIntUGT,
  LLVMIntUGE,
  LLVMIntULT,
  LLVMIntULE,
  LLVMIntSGT,
  LLVMIntSGE,
  LLVMIntSLT,
  LLVMIntSLE,
}

#[repr(C)]
pub enum LLVMCodeGenOptLevel {
    LLVMCodeGenLevelNone,
    LLVMCodeGenLevelLess,
    LLVMCodeGenLevelDefault,
    LLVMCodeGenLevelAggressive
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum LLVMRelocMode {
    LLVMRelocDefault,
    LLVMRelocStatic,
    LLVMRelocPIC,
    LLVMRelocDynamicNoPic,
    LLVMRelocROPI,
    LLVMRelocRWPI,
    LLVMRelocROPI_RWPI
}

#[repr(C)]
pub enum LLVMCodeModel {
    LLVMCodeModelDefault,
    LLVMCodeModelJITDefault,
    LLVMCodeModelTiny,
    LLVMCodeModelSmall,
    LLVMCodeModelKernel,
    LLVMCodeModelMedium,
    LLVMCodeModelLarge
}

#[link(name="LLVM-9")]
extern "C" {
    pub fn LLVMModuleCreateWithName(ModuleID: *const c_char) -> LLVMModuleRef;
    pub fn LLVMInt8Type() -> LLVMTypeRef;
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
    // Ownership: dispose the message.
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
    // Ownership: remove itself from the context, delete the globals, the functions, the aliases
    // and metadata.
    pub fn LLVMDisposeModule(M: LLVMModuleRef);
    pub fn LLVMRemoveModule(EE: LLVMExecutionEngineRef, M: LLVMModuleRef, OutMod: *mut LLVMModuleRef, OutError: *mut *mut c_char) -> LLVMBool;
    pub fn LLVMShutdown();
    pub fn LLVMConstInt(IntTy: LLVMTypeRef, N: u64, SignExtend: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstReal(RealTy: LLVMTypeRef , N: f64) -> LLVMValueRef;
    pub fn LLVMDoubleType() -> LLVMTypeRef;
    pub fn LLVMBuildFCmp(builder: LLVMBuilderRef, Op: LLVMRealPredicate, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildUIToFP(builder: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildFAdd(builder: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildFMul(builder: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildFSub(builder: LLVMBuilderRef, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMGetNamedFunction(M: LLVMModuleRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMCountParams(Fn: LLVMValueRef) -> u32;
    pub fn LLVMBuildCall(builder: LLVMBuilderRef, Fn: LLVMValueRef, Args: *mut LLVMValueRef, NumArgs: u32, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMSetValueName2(Val: LLVMValueRef, Name: *const c_char, NameLen: usize);
    pub fn LLVMVerifyFunction(Fn: LLVMValueRef, Action: LLVMVerifierFailureAction) -> LLVMBool;
    pub fn LLVMDeleteFunction(Fn: LLVMValueRef);
    pub fn LLVMDumpModule(M: LLVMModuleRef);
    pub fn LLVMAddModule(EE: LLVMExecutionEngineRef, M: LLVMModuleRef);
    pub fn LLVMDumpValue(Val: LLVMValueRef);
    pub fn LLVMCreateFunctionPassManagerForModule(M: LLVMModuleRef) -> LLVMPassManagerRef;
    pub fn LLVMDisposePassManager(PM: LLVMPassManagerRef);
    pub fn LLVMRunPassManager(PM: LLVMPassManagerRef, M: LLVMModuleRef) -> LLVMBool;
    pub fn LLVMRunFunctionPassManager(FPM: LLVMPassManagerRef, F: LLVMValueRef) -> LLVMBool;
    pub fn LLVMAddInstructionCombiningPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddReassociatePass(PM: LLVMPassManagerRef);
    pub fn LLVMAddGVNPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddCFGSimplificationPass(PM: LLVMPassManagerRef);
    pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
    pub fn LLVMContextCreate() -> LLVMContextRef;
    pub fn LLVMCreateBuilderInContext(C: LLVMContextRef) -> LLVMBuilderRef;
    pub fn LLVMCreateBasicBlockInContext(C: LLVMContextRef, Name: *const c_char) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlockInContext(C: LLVMContextRef, Fn: LLVMValueRef, Name: *const c_char) -> LLVMBasicBlockRef;
    pub fn LLVMGetBasicBlockParent(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMBuildCondBr(builder: LLVMBuilderRef, If: LLVMValueRef, Then: LLVMBasicBlockRef, Else: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMBuildBr(builder: LLVMBuilderRef, Dest: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMBuildPhi(builder: LLVMBuilderRef, Ty: LLVMTypeRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMAddIncoming(PhiNode: LLVMValueRef, IncomingValues: *mut LLVMValueRef, IncomingBlocks: *mut LLVMBasicBlockRef, Count: c_uint);
    pub fn LLVMConstNull(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMGetEntryBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetFirstInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMPositionBuilder(Builder: LLVMBuilderRef, Block: LLVMBasicBlockRef, Instr: LLVMValueRef);
    pub fn LLVMBuildAlloca(builder: LLVMBuilderRef, Ty:  LLVMTypeRef, Name: *const c_char) ->  LLVMValueRef;
    pub fn LLVMBuildLoad2(builder: LLVMBuilderRef, Ty: LLVMTypeRef, PointerVal: LLVMValueRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildStore(builder: LLVMBuilderRef, Val: LLVMValueRef, Ptr: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetExecutionEngineTargetData(EE: LLVMExecutionEngineRef) -> LLVMTargetDataRef;
    pub fn LLVMAddPromoteMemoryToRegisterPass(PM: LLVMPassManagerRef);
    pub fn LLVMGetDefaultTargetTriple() -> *mut c_char;
    pub fn LLVM_InitializeAllTargetInfos();
    pub fn LLVM_InitializeAllTargets();
    pub fn LLVM_InitializeAllTargetMCs();
    pub fn LLVM_InitializeAllAsmParsers();
    pub fn LLVM_InitializeAllAsmPrinters();
    pub fn LLVMSetTarget(M: LLVMModuleRef, Triple: *const c_char);
    pub fn LLVMCreateTargetMachine(T: LLVMTargetRef, Triple: *const c_char, CPU: *const c_char, Features: *const c_char, Level: LLVMCodeGenOptLevel, Reloc: LLVMRelocMode, CodeModel: LLVMCodeModel) -> LLVMTargetMachineRef;
    pub fn LLVMGetTargetFromName(Name: *const c_char) -> LLVMTargetRef;
    pub fn LLVMSetDataLayout(M: LLVMModuleRef, DataLayoutStr: *const c_char);
    pub fn LLVMCreateTargetDataLayout(T: LLVMTargetMachineRef) -> LLVMTargetDataRef;
    pub fn LLVMTargetMachineEmitToFile(T: LLVMTargetMachineRef, M: LLVMModuleRef, Filename: *mut c_char, codegen: LLVMCodeGenFileType, ErrorMessage: *mut *mut c_char) -> LLVMBool;
    pub fn LLVMGetTargetFromTriple(Triple: *const c_char, T: *mut LLVMTargetRef, ErrorMessage: *mut *mut c_char) -> LLVMBool;
    pub fn LLVMDisposeTargetMachine(T: LLVMTargetMachineRef);
    pub fn LLVMDisposeTargetData(TD: LLVMTargetDataRef);
    // Ownership: dispose the modules.
    pub fn LLVMContextDispose(C: LLVMContextRef);
    pub fn LLVMModuleCreateWithNameInContext(ModuleID: *const c_char, C: LLVMContextRef) -> LLVMModuleRef;
    pub fn LLVMIntTypeInContext(C: LLVMContextRef, NumBits: c_uint) -> LLVMTypeRef;
    pub fn LLVMInt32TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMDoubleTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMConstString(Str: *const c_char, Length: u32, DontNullTerminate: LLVMBool) -> LLVMValueRef;
    pub fn LLVMArrayType(ElementType: LLVMTypeRef, ElementCount: u32) -> LLVMTypeRef;
    pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: u32) -> LLVMTypeRef;
    pub fn LLVMBuildBitCast(builder: LLVMBuilderRef, Val: LLVMValueRef, DestTy: LLVMTypeRef, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildGEP(B: LLVMBuilderRef, Pointer: LLVMValueRef, Indices: *mut LLVMValueRef, NumIndices: u32, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMBuildGlobalStringPtr(B: LLVMBuilderRef, Str: *const c_char, Name: *const c_char) -> LLVMValueRef;
    pub fn LLVMVoidType() -> LLVMTypeRef;
    pub fn LLVMCountBasicBlocks(Fn: LLVMValueRef) -> c_uint;
    pub fn LLVMBuildICmp(Builder: LLVMBuilderRef, Op: LLVMIntPredicate, LHS: LLVMValueRef, RHS: LLVMValueRef, Name: *const c_char) -> LLVMValueRef;
}
