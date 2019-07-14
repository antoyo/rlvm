use ffi::LLVMBasicBlockRef;

pub struct BasicBlock(LLVMBasicBlockRef);

impl BasicBlock {
    pub fn from_raw(basic_block: LLVMBasicBlockRef) -> Self {
        Self(basic_block)
    }

    pub fn as_raw(&self) -> LLVMBasicBlockRef {
        self.0
    }
}
