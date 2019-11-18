use std::collections::HashMap;
use std::iter;

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
    Unexpected,
    WrongArgumentCount,
};

pub struct Generator {
    builder: Builder,
    context: Context,
    function_prototypes: HashMap<String, Prototype>,
    pub module: Module,
    pass_manager: FunctionPassManager,
    values: HashMap<String, Value>,
}

impl Generator {
    pub fn new(context: Context, module: Module, pass_manager: FunctionPassManager) -> Result<Self> {
        Ok(Self {
            builder: Builder::new_in_context(&context),
            context,
            function_prototypes: HashMap::new(),
            module,
            pass_manager,
            values: HashMap::new(),
        })
    }

    fn create_argument_allocas(&mut self, function: &Function, prototype: &Prototype) {
        for (index, variable_name) in prototype.parameters.iter().enumerate() {
            let arg = function.get_param(index);
            let alloca = self.create_entry_block_alloca(function, variable_name);
            self.builder.store(&arg, &alloca);
            self.values.insert(variable_name.clone(), alloca);
        }
    }

    fn create_entry_block_alloca(&self, function: &Function, variable_name: &str) -> Value {
        let basic_block = function.get_entry_basic_block();
        let instruction = basic_block.get_first_instruction();
        let builder = Builder::new_in_context(&self.context);
        builder.position(&basic_block, &instruction);
        builder.alloca(self.context.double(), variable_name)
    }

    fn expr(&mut self, expr: Expr) -> Result<Value> {
        let value =
            match expr {
                Expr::Number(num) => constant::real(self.context.double(), num),
                Expr::Variable(name) => {
                    match self.values.get(&name) {
                        Some(value) => self.builder.load(self.context.double(), value, &name),
                        None => return Err(Undefined(format!("variable {}", name))),
                    }
                },
                Expr::Binary(op, left, right) => {
                    if op == BinaryOp::Equal {
                        let name =
                            match *left {
                                Expr::Variable(ref name) => name,
                                _ => return Err(Unexpected("token, expecting variable name")),
                            };

                        let value = self.expr(*right)?;
                        let variable =
                            match self.values.get(name) {
                                Some(value) => value,
                                None => return Err(Undefined(format!("variable {}", name))),
                            };

                        self.builder.store(&value, variable);

                        return Ok(value);
                    }
                    let left = self.expr(*left)?;
                    let right = self.expr(*right)?;
                    match op {
                        BinaryOp::Plus => self.builder.fadd(&left, &right, "result"),
                        BinaryOp::Minus => self.builder.fsub(&left, &right, "result"),
                        BinaryOp::Times => self.builder.fmul(&left, &right, "result"),
                        BinaryOp::LessThan => {
                            let boolean = self.builder.fcmp(RealPredicate::UnorderedLesserThan, &left, &right, "cmptmp");
                            self.builder.unsigned_int_to_floating_point(&boolean, self.context.double(), "booltemp")
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
                        BinaryOp::Equal => unreachable!(),
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
                    let condition = self.builder.fcmp(RealPredicate::OrderedNotEqual, &condition, &constant::real(self.context.double(), 0.0), "ifcond");

                    let start_basic_block = self.builder.get_insert_block().expect("start basic block");

                    let function = start_basic_block.get_parent();

                    let then_basic_block = BasicBlock::append_in_context(&self.context, &function, "then");

                    self.builder.position_at_end(&then_basic_block);

                    let then_value = self.expr(*then)?;

                    let new_then_basic_block = self.builder.get_insert_block().expect("new then basic block");

                    let else_basic_block = BasicBlock::append_in_context(&self.context, &function, "else");
                    self.builder.position_at_end(&else_basic_block);

                    let else_value = self.expr(*else_)?;

                    let new_else_basic_block = self.builder.get_insert_block().expect("new else basic block");

                    let merge_basic_block = BasicBlock::append_in_context(&self.context, &function, "ifcont");
                    self.builder.position_at_end(&merge_basic_block);

                    let phi = self.builder.phi(self.context.double(), "result");
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
                    let function = self.builder.get_insert_block().expect("function basic block").get_parent();
                    let alloca = self.create_entry_block_alloca(&function, &variable_name);

                    let start_value = self.expr(*init_value)?;

                    self.builder.store(&start_value, &alloca);

                    let loop_basic_block = BasicBlock::append_in_context(&self.context, &function, "loop");

                    self.builder.br(&loop_basic_block);

                    self.builder.position_at_end(&loop_basic_block);

                    let old_value = self.values.insert(variable_name.clone(), alloca.clone());

                    self.expr(*body)?;

                    let step_value =
                        match step {
                            Some(step) => self.expr(*step)?,
                            None => constant::real(self.context.double(), 1.0),
                        };

                    let end_condition = self.expr(*condition)?;

                    let current_variable = self.builder.load(self.context.double(), &alloca, &variable_name);
                    let next_variable = self.builder.fadd(&current_variable, &step_value, "nextvar");
                    self.builder.store(&next_variable, &alloca);

                    let zero = constant::real(self.context.double(), 0.0);
                    let end_condition = self.builder.fcmp(RealPredicate::OrderedNotEqual, &end_condition, &zero, "loopcond");

                    let after_basic_block = BasicBlock::append_in_context(&self.context, &function, "afterloop");

                    self.builder.cond_br(&end_condition, &loop_basic_block, &after_basic_block);

                    self.builder.position_at_end(&after_basic_block);

                    if let Some(old_value) = old_value {
                         self.values.insert(variable_name, old_value);
                    }

                    constant::null(self.context.double())
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
                Expr::VariableDeclaration { body, declarations } => {
                    let mut old_bindings = vec![];

                    let function = self.builder.get_insert_block().expect("function basic block").get_parent();

                    for declaration in declarations {
                        let init_value =
                            match declaration.init_value {
                                Some(value) => self.expr(*value)?,
                                None => constant::real(self.context.double(), 0.0),
                            };

                        let alloca = self.create_entry_block_alloca(&function, &declaration.name);
                        self.builder.store(&init_value, &alloca);

                        if let Some(old_value) = self.values.get(&declaration.name) {
                            old_bindings.push((declaration.name.clone(), old_value.clone()));
                        }

                        self.values.insert(declaration.name.clone(), alloca);
                    }

                    let body_value = self.expr(*body)?;

                    for (variable_name, old_value) in old_bindings {
                        self.values.insert(variable_name, old_value);
                    }

                    body_value
                },
            };
        Ok(value)
    }

    pub fn function(&mut self, function: ast::Function) -> Result<()> {
        let name = function.prototype.function_name.clone();
        let llvm_function =
            match self.module.get_named_function(&name) {
                Some(llvm_function) => llvm_function,
                None => self.prototype(&function.prototype),
            };
        let entry = llvm_function.append_basic_block("entry");
        self.builder.position_at_end(&entry);
        self.values.clear();
        self.create_argument_allocas(&llvm_function, &function.prototype);

        let return_value =
            match self.expr(function.body) {
                Ok(value) => value,
                Err(error) => {
                    llvm_function.delete();
                    return Err(error);
                },
            };

        self.builder.ret(&return_value);
        llvm_function.verify(VerifierFailureAction::AbortProcess);

        self.pass_manager.run(&llvm_function);

        self.module.dump();

        Ok(())
    }

    pub fn prototype(&mut self, prototype: &Prototype) -> Function {
        let param_types: Vec<_> = iter::repeat(self.context.double()).take(prototype.parameters.len()).collect();
        let function_type = types::function::new(self.context.double(), &param_types, false);
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
