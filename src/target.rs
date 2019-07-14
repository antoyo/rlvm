use ffi::{
    LLVM_InitializeNativeAsmPrinter,
    LLVM_InitializeNativeTarget,
};

pub fn initialize_native_asm_printer() -> bool {
    unsafe { LLVM_InitializeNativeAsmPrinter() != 0 }
}

pub fn initialize_native_target() -> bool {
    unsafe { LLVM_InitializeNativeTarget() != 0 }
}
