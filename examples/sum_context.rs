extern crate rlvm;

use rlvm::{
    BasicBlock,
    Builder,
    Context,
    ExecutionEngine,
    VerifierFailureAction,
    initialize_native_asm_printer,
    initialize_native_target,
    link_mcjit,
    llvm_init,
};
use rlvm::types;

fn main() {
    let _llvm = llvm_init();

    link_mcjit();
    initialize_native_asm_printer();
    initialize_native_target();

    let context = Context::new();
    let module = context.new_module("module");
    let engine = ExecutionEngine::new_for_module(&module).expect("failed to create execution engine");
    let param_types = [context.int32(), context.int32()];
    let function_type = types::function::new(context.int32(), &param_types, false);
    let sum = module.add_function("sum", function_type);

    let entry = BasicBlock::append_in_context(&context, &sum, "entry");

    let builder = Builder::new_in_context(&context);
    builder.position_at_end(&entry);

    let temp = builder.add(&sum.get_param(0), &sum.get_param(1), "temp");
    builder.ret(&temp);

    module.verify(VerifierFailureAction::AbortProcess).expect("module verify");

    module.dump();

    let sum: fn(i32, i32) -> i32 = unsafe { engine.get_function_address("sum").expect("sum function").cast2_ret() };
    println!("{}", sum(40, 2));
}
