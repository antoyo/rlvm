use std::collections::HashMap;
use std::iter;
use std::mem;

use rlvm::{
    BasicBlock,
    Builder,
    Context,
    Module,
    FunctionPassManager,
    RealPredicate,
    Value,
    VerifierFailureAction,
};
use rlvm::module::Function;
use rlvm::types::{self, Type};
use rlvm::value::constant;

use ast::{
    self,
    BinaryOp,
    Expr,
    Prototype,
};
use error::Result;
use error::Error::{
    Undefined,
    WrongArgumentCount,
};

pub struct Generator {
    builder: Builder,
    context: Context,
    function_prototypes: HashMap<String, Prototype>,
    module: Module,
    pass_manager: FunctionPassManager,
    values: HashMap<String, Value>,
}

fn new_module() -> (Module, FunctionPassManager) {
    let module = Module::new_with_name("module");
    let pass_manager = FunctionPassManager::new_for_module(&module);
    pass_manager.add_instruction_combining_pass();
    pass_manager.add_reassociate_pass();
    pass_manager.add_gvn_pass();
    pass_manager.add_cfg_simplification_pass();
    (module, pass_manager)
}

impl Generator {
    pub fn new() -> Result<Self> {
        let (module, pass_manager) = new_module();
        let context = Context::new();
        Ok(Self {
            builder: Builder::new_in_context(&context),
            context,
            function_prototypes: HashMap::new(),
            module,
            pass_manager,
            values: HashMap::new(),
        })
    }

    fn expr(&mut self, expr: Expr) -> Result<Value> {
        let value =
            match expr {
                Expr::Number(num) => constant::real(types::double(), num),
                Expr::Variable(name) => {
                    match self.values.get(&name) {
                        Some(variable) => variable.clone(),
                        None => return Err(Undefined(format!("variable {}", name))),
                    }
                },
                Expr::Binary(op, left, right) => {
                    let left = self.expr(*left)?;
                    let right = self.expr(*right)?;
                    match op {
                        BinaryOp::Plus => self.builder.fadd(&left, &right, "result"),
                        BinaryOp::Minus => self.builder.fsub(left, right, "result"),
                        BinaryOp::Times => self.builder.fmul(left, right, "result"),
                        BinaryOp::LessThan => {
                            let boolean = self.builder.fcmp(RealPredicate::UnorderedLesserThan, &left, &right, "cmptmp");
                            self.builder.unsigned_int_to_floating_point(boolean, types::double(), "booltemp")
                        },
                        BinaryOp::Custom(char) => {
                            let callee = format!("binary{}", char);
                            let callee =
                                match self.get_named_function(&callee) {
                                    Some(function) => function,
                                    None => return Err(Undefined(format!("function {}", callee))),
                                };
                            self.builder.call(callee, &[left, right], "binop")
                        },
                        _ => unimplemented!(),
                    }
                },
                Expr::Call(name, args) => {
                    match self.get_named_function(&name) {
                        Some(func) => {
                            if func.param_count() != args.len() {
                                return Err(WrongArgumentCount);
                            }
                            let arguments: Result<Vec<_>> = args.into_iter().map(|arg| self.expr(arg)).collect();
                            let arguments = arguments?;
                            self.builder.call(func, &arguments, "func_call")
                        },
                        None => return Err(Undefined(format!("function {}", name))),
                    }
                },
                Expr::If { condition, then, else_ } => {
                    let condition = self.expr(*condition)?;
                    let condition = self.builder.fcmp(RealPredicate::OrderedNotEqual, &condition, &constant::real(types::double(), 0.0), "ifcond");

                    let start_basic_block = self.builder.get_insert_block();

                    let function = start_basic_block.get_parent();

                    let then_basic_block = BasicBlock::append_in_context(&self.context, &function, "then");

                    self.builder.position_at_end(&then_basic_block);

                    let then_value = self.expr(*then)?;

                    let new_then_basic_block = self.builder.get_insert_block();

                    let else_basic_block = BasicBlock::append_in_context(&self.context, &function, "else");
                    self.builder.position_at_end(&else_basic_block);

                    let else_value = self.expr(*else_)?;

                    let new_else_basic_block = self.builder.get_insert_block();

                    let merge_basic_block = BasicBlock::append_in_context(&self.context, &function, "ifcont");
                    self.builder.position_at_end(&merge_basic_block);

                    let phi = self.builder.phi(types::double(), "result");
                    phi.add_incoming(&[(&then_value, &new_then_basic_block), (&else_value, &new_else_basic_block)]);

                    self.builder.position_at_end(&start_basic_block);
                    self.builder.cond_br(&condition, &then_basic_block, &else_basic_block);

                    self.builder.position_at_end(&new_then_basic_block);
                    self.builder.br(&merge_basic_block);

                    self.builder.position_at_end(&new_else_basic_block);
                    self.builder.br(&merge_basic_block);

                    self.builder.position_at_end(&merge_basic_block);

                    phi
                },
                Expr::For { body, variable_name, init_value, condition, step } => {
                    let start_value = self.expr(*init_value)?;

                    let preheader_basic_block = self.builder.get_insert_block();
                    let function = preheader_basic_block.get_parent();
                    let loop_basic_block = BasicBlock::append_in_context(&self.context, &function, "loop");

                    self.builder.br(&loop_basic_block);

                    self.builder.position_at_end(&loop_basic_block);

                    let phi = self.builder.phi(types::double(), &variable_name);
                    phi.add_incoming(&[(&start_value, &preheader_basic_block)]);

                    let old_value = self.values.insert(variable_name.clone(), phi.clone());

                    self.expr(*body)?;

                    let step_value =
                        match step {
                            Some(step) => self.expr(*step)?,
                            None => constant::real(types::double(), 1.0),
                        };

                    let next_variable = self.builder.fadd(&phi, &step_value, "nextvar");

                    let end_condition = self.expr(*condition)?;

                    let zero = constant::real(types::double(), 0.0);
                    let end_condition = self.builder.fcmp(RealPredicate::OrderedNotEqual, &end_condition, &zero, "loopcond");

                    let loop_end_basic_block = self.builder.get_insert_block();
                    let after_basic_block = BasicBlock::append_in_context(&self.context, &function, "afterloop");

                    self.builder.cond_br(&end_condition, &loop_basic_block, &after_basic_block);

                    self.builder.position_at_end(&after_basic_block);

                    phi.add_incoming(&[(&next_variable, &loop_end_basic_block)]);

                    if let Some(old_value) = old_value {
                         self.values.insert(variable_name, old_value);
                    }

                    constant::null(types::double())
                },
                Expr::Unary(operator, operand) => {
                    let operand = self.expr(*operand)?;
                    let callee = format!("unary{}", operator);
                    let callee =
                        match self.get_named_function(&callee) {
                            Some(function) => function,
                            None => return Err(Undefined(format!("function {}", callee))),
                        };
                    self.builder.call(callee, &[operand], "unop")
                },
                _ => unimplemented!("{:?}", expr),
            };
        Ok(value)
    }

    pub fn function(&mut self, function: ast::Function) -> Result<(Module, String)> {
        let name = function.prototype.function_name.clone();
        let llvm_function =
            match self.module.get_named_function(&name) {
                Some(llvm_function) => llvm_function,
                None => self.prototype(&function.prototype),
            };
        let entry = llvm_function.append_basic_block("entry");
        self.builder.position_at_end(&entry);
        self.values.clear();
        for (index, arg) in function.prototype.parameters.iter().enumerate() {
            self.values.insert(arg.clone(), llvm_function.get_param(index));
        }

        let return_value =
            match self.expr(function.body) {
                Ok(value) => value,
                Err(error) => {
                    llvm_function.delete();
                    return Err(error);
                },
            };

        self.builder.ret(return_value);
        llvm_function.verify(VerifierFailureAction::AbortProcess);

        self.pass_manager.run(&llvm_function);

        self.module.dump();

        let (module, pass_manager) = new_module();
        let module = mem::replace(&mut self.module, module);
        self.pass_manager = pass_manager;

        Ok((module, name))
    }

    pub fn prototype(&mut self, prototype: &Prototype) -> Function {
        let param_types: Vec<_> = iter::repeat(types::double()).take(prototype.parameters.len()).collect();
        let function_type = types::function::new(types::double(), &param_types, false);
        let function = self.add_function(&prototype.function_name, function_type);
        for (index, param) in prototype.parameters.iter().enumerate() {
            function.get_param(index).set_name(param);
        }
        self.function_prototypes.insert(prototype.function_name.clone(), prototype.clone());
        function
    }

    fn add_function(&self, name: &str, typ: Type) -> Function {
        self.module.add_function(&name, typ)
    }

    fn get_named_function(&mut self, name: &str) -> Option<Function> {
        match self.module.get_named_function(&name) {
            Some(function) => Some(function),
            None => {
                // If it's not in the current module, fetch it from other modules.
                if let Some(prototype) = self.function_prototypes.get(name).map(Clone::clone) {
                    Some(self.prototype(&prototype))
                }
                else {
                    None
                }
            },
        }
    }
}
