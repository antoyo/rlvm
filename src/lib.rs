/*
 * TODO: change from assert to debug_assert?
 */

pub mod analysis;
pub mod basic_block;
pub mod builder;
pub mod context;
pub mod exec_engine;
pub mod ffi;
pub mod global;
pub mod module;
pub mod pass_manager;
pub mod target;
pub mod types;
pub mod value;

pub use analysis::VerifierFailureAction;
pub use basic_block::BasicBlock;
pub use builder::{Builder, IntPredicate, RealPredicate};
pub use context::Context;
pub use exec_engine::{ExecutionEngine, FunctionAddress, link_mcjit};
pub use global::GlobalVariable;
pub use module::Module;
pub use pass_manager::{FunctionPassManager, ModulePassManager};
pub use target::{
    CodeGenFileType,
    CodeGenOptLevel,
    CodeModel,
    RelocMode,
    Target,
    get_default_target_triple,
    initialize_all_asm_parsers,
    initialize_all_asm_printers,
    initialize_all_target_infos,
    initialize_all_target_mcs,
    initialize_all_targets,
    initialize_native_asm_printer,
    initialize_native_target,
};
pub use value::Value;

use std::cell::Cell;
use std::marker::PhantomData;

use ffi::LLVMShutdown;

thread_local! {
    static INITIALIZED: Cell<bool> = Cell::new(false);
}

pub struct LLVM {
    _phantom: PhantomData<*mut ()>,
}

fn assert_llvm_initialized() {
    INITIALIZED.with(|initialized| {
        if !initialized.get() {
            panic!("LLVM not initialized: call llvm_init()");
        }
    });
}

pub fn llvm_init() -> LLVM {
    INITIALIZED.with(|initialized| {
        initialized.set(true);
    });
    LLVM {
        _phantom: PhantomData,
    }
}

impl Drop for LLVM {
    fn drop(&mut self) {
        unsafe {
            LLVMShutdown();
        }
        INITIALIZED.with(|initialized| {
            initialized.set(false);
        });
    }
}
