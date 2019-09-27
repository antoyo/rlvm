#include <llvm-c/Target.h>

void LLVM_InitializeAllTargetInfos(void) {
    LLVMInitializeAllTargetInfos();
}

void LLVM_InitializeAllTargets(void) {
    LLVMInitializeAllTargets();
}

void LLVM_InitializeAllTargetMCs(void) {
    LLVMInitializeAllTargetMCs();
}

void LLVM_InitializeAllAsmParsers(void) {
    LLVMInitializeAllAsmParsers();
}

void LLVM_InitializeAllAsmPrinters(void) {
    LLVMInitializeAllAsmPrinters();
}

LLVMBool LLVM_InitializeNativeTarget(void) {
    return LLVMInitializeNativeTarget();
}

LLVMBool LLVM_InitializeNativeAsmPrinter(void) {
    return LLVMInitializeNativeAsmPrinter();
}
