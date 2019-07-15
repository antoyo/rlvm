use std::ffi::CStr;
use std::ptr;

use ffi::{
    LLVMDisposeMessage,
    LLVMVerifierFailureAction,
    LLVMVerifyModule,
};
use module::Module;

pub enum VerifierFailureAction {
    AbortProcess,
    PrintMessage,
    ReturnStatus,
}

impl VerifierFailureAction {
    pub fn as_raw(&self) -> LLVMVerifierFailureAction {
        match *self {
            VerifierFailureAction::AbortProcess => LLVMVerifierFailureAction::LLVMAbortProcessAction,
            VerifierFailureAction::PrintMessage => LLVMVerifierFailureAction::LLVMPrintMessageAction,
            VerifierFailureAction::ReturnStatus => LLVMVerifierFailureAction::LLVMReturnStatusAction,
        }
    }
}

impl Module {
    pub fn verify(&self, action: VerifierFailureAction) -> Result<(), String> {
        let mut error = ptr::null_mut();
        unsafe {
            let result = LLVMVerifyModule(self.as_raw(), action.as_raw(), &mut error);

            if result != 0 {
                let cstr = CStr::from_ptr(error);
                let verify_error = cstr.to_str().expect("error cstr").to_string();
                LLVMDisposeMessage(error);
                Err(verify_error)
            }
            else {
                Ok(())
            }
        }
    }
}
