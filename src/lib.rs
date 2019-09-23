pub mod analysis;
pub mod basic_block;
pub mod builder;
pub mod exec_engine;
pub mod ffi;
pub mod module;
pub mod pass_manager;
pub mod target;
pub mod types;
pub mod value;

pub use analysis::VerifierFailureAction;
pub use builder::{Builder, RealPredicate};
pub use exec_engine::{ExecutionEngine, FunctionAddress, link_mcjit};
pub use module::Module;
pub use pass_manager::FunctionPassManager;
pub use target::{initialize_native_asm_printer, initialize_native_target};
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
