use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr;

use Module;
use exec_engine::TargetData;
use ffi::{
    LLVMCodeGenFileType,
    LLVMCodeGenOptLevel,
    LLVMCodeModel,
    LLVMCreateTargetDataLayout,
    LLVMCreateTargetMachine,
    LLVMDisposeMessage,
    LLVMDisposeTargetMachine,
    LLVMGetDefaultTargetTriple,
    LLVMGetTargetFromName,
    LLVMGetTargetFromTriple,
    LLVM_InitializeAllTargetInfos,
    LLVM_InitializeAllTargets,
    LLVM_InitializeAllTargetMCs,
    LLVM_InitializeAllAsmParsers,
    LLVM_InitializeAllAsmPrinters,
    LLVM_InitializeNativeAsmPrinter,
    LLVM_InitializeNativeTarget,
    LLVMRelocMode,
    LLVMTargetRef,
    LLVMTargetMachineRef,
    LLVMTargetMachineEmitToFile,
};

pub enum CodeGenFileType {
    AssemblyFile,
    ObjectFile
}

impl CodeGenFileType {
    fn as_raw(&self) -> LLVMCodeGenFileType {
        match *self {
            CodeGenFileType::AssemblyFile => LLVMCodeGenFileType::LLVMAssemblyFile,
            CodeGenFileType::ObjectFile => LLVMCodeGenFileType::LLVMObjectFile,
        }
    }
}

pub struct TargetMachine(LLVMTargetMachineRef);

impl TargetMachine {
    pub fn as_raw(&self) -> LLVMTargetMachineRef {
        self.0
    }

    pub fn create_data_layout(&self) -> TargetData {
        unsafe {
            TargetData::from_raw(LLVMCreateTargetDataLayout(self.as_raw()))
        }
    }

    pub fn emit_to_file(&self, module: &Module, filename: &str, codegen: CodeGenFileType) -> Result<(), String> {
        let mut error = ptr::null_mut();
        let filename = CString::new(filename).expect("cstring");
        unsafe {
            let result = LLVMTargetMachineEmitToFile(self.as_raw(), module.as_raw(), filename.as_ptr() as *mut _, codegen.as_raw(), &mut error);
            if result != 0 {
                let cstr = CStr::from_ptr(error);
                let emit_error = cstr.to_str().expect("error cstr").to_string();
                LLVMDisposeMessage(error);
                Err(emit_error)
            }
            else {
                Ok(())
            }
        }
    }

    pub fn from_raw(target_machine: LLVMTargetMachineRef) -> Self {
        Self(target_machine)
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetMachine(self.as_raw())
        }
    }
}

pub enum CodeGenOptLevel {
    None,
    Less,
    Default,
    Aggressive
}

impl CodeGenOptLevel {
    fn as_raw(&self) -> LLVMCodeGenOptLevel {
        match *self {
            CodeGenOptLevel::None => LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
            CodeGenOptLevel::Less => LLVMCodeGenOptLevel::LLVMCodeGenLevelLess,
            CodeGenOptLevel::Default => LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            CodeGenOptLevel::Aggressive => LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
        }
    }
}

pub enum RelocMode {
    Default,
    Static,
    PIC,
    DynamicNoPic,
    ROPI,
    RWPI,
    RopiRwpi
}

impl RelocMode {
    fn as_raw(&self) -> LLVMRelocMode {
        match *self {
            RelocMode::Default => LLVMRelocMode::LLVMRelocDefault,
            RelocMode::Static => LLVMRelocMode::LLVMRelocStatic,
            RelocMode::PIC => LLVMRelocMode::LLVMRelocPIC,
            RelocMode::DynamicNoPic => LLVMRelocMode::LLVMRelocDynamicNoPic,
            RelocMode::ROPI => LLVMRelocMode::LLVMRelocROPI,
            RelocMode::RWPI => LLVMRelocMode::LLVMRelocRWPI,
            RelocMode::RopiRwpi => LLVMRelocMode::LLVMRelocROPI_RWPI,
        }
    }
}

pub enum CodeModel {
    Default,
    JITDefault,
    Tiny,
    Small,
    Kernel,
    Medium,
    Large
}

impl CodeModel {
    fn as_raw(&self) -> LLVMCodeModel {
        match *self {
            CodeModel::Default => LLVMCodeModel::LLVMCodeModelDefault,
            CodeModel::JITDefault => LLVMCodeModel::LLVMCodeModelJITDefault,
            CodeModel::Tiny => LLVMCodeModel::LLVMCodeModelTiny,
            CodeModel::Small => LLVMCodeModel::LLVMCodeModelSmall,
            CodeModel::Kernel => LLVMCodeModel::LLVMCodeModelKernel,
            CodeModel::Medium => LLVMCodeModel::LLVMCodeModelMedium,
            CodeModel::Large => LLVMCodeModel::LLVMCodeModelLarge,
        }
    }
}

pub struct Target(LLVMTargetRef);

impl Target {
    pub fn as_raw(&self) -> LLVMTargetRef {
        self.0
    }

    pub fn create_target_machine(&self, triple: &TargetTriple, cpu: &str, features: &str, level: CodeGenOptLevel, reloc: RelocMode, code_model: CodeModel) -> TargetMachine {
        let cpu = CString::new(cpu).expect("cstring");
        let features = CString::new(features).expect("cstring");
        unsafe {
            TargetMachine::from_raw(LLVMCreateTargetMachine(self.as_raw(), triple.as_raw(), cpu.as_ptr(), features.as_ptr(), level.as_raw(), reloc.as_raw(), code_model.as_raw()))
        }
    }

    pub fn from_raw(target: LLVMTargetRef) -> Self {
        Self(target)
    }

    pub fn get_from_name(name: &TargetTriple) -> Option<Self> {
        unsafe {
            let ptr = LLVMGetTargetFromName(name.as_raw());
            if ptr.is_null() {
                return None;
            }
            Some(Self::from_raw(ptr))
        }
    }

    pub fn get_from_triple(triple: &TargetTriple) -> Result<Self, String> {
        let mut error = ptr::null_mut();

        let mut target = MaybeUninit::<LLVMTargetRef>::uninit();
        unsafe {
            let result = LLVMGetTargetFromTriple(triple.as_raw(), target.as_mut_ptr(), &mut error);
            if result != 0 {
                let cstr = CStr::from_ptr(error);
                let verify_error = cstr.to_str().expect("error cstr").to_string();
                LLVMDisposeMessage(error);
                Err(verify_error)
            }
            else {
                Ok(Self::from_raw(target.assume_init()))
            }
        }
    }
}

pub struct TargetTriple(*const c_char);

impl TargetTriple {
    pub fn as_raw(&self) -> *const c_char {
        self.0
    }

    pub fn from_raw(triple: *const c_char) -> TargetTriple {
        Self(triple)
    }
}

impl Drop for TargetTriple {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeMessage(self.0 as *mut _);
        }
    }
}

pub fn initialize_native_asm_printer() -> bool {
    unsafe { LLVM_InitializeNativeAsmPrinter() != 0 }
}

pub fn initialize_native_target() -> bool {
    unsafe { LLVM_InitializeNativeTarget() != 0 }
}

pub fn initialize_all_target_infos() {
    unsafe { LLVM_InitializeAllTargetInfos(); }
}

pub fn initialize_all_targets() {
    unsafe { LLVM_InitializeAllTargets(); }
}

pub fn initialize_all_target_mcs() {
    unsafe { LLVM_InitializeAllTargetMCs(); }
}

pub fn initialize_all_asm_parsers() {
    unsafe { LLVM_InitializeAllAsmParsers(); }
}

pub fn initialize_all_asm_printers() {
    unsafe { LLVM_InitializeAllAsmPrinters(); }
}

pub fn get_default_target_triple() -> TargetTriple {
    unsafe { TargetTriple::from_raw(LLVMGetDefaultTargetTriple()) }
}
