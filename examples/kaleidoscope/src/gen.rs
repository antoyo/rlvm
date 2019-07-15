use std::collections::HashMap;
use std::iter;

use rlvm::{
    Builder,
    Module,
    RealPredicate,
    Value,
    VerifierFailureAction,
};
use rlvm::module::Function;
use rlvm::types;
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
    module: Module,
    values: HashMap<String, Value>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            builder: Builder::new(),
            module: Module::new_with_name("module"),
            values: HashMap::new(),
        }
    }

    fn expr(&self, expr: Expr) -> Result<Value> {
        let value =
            match expr {
                Expr::Number(num) => constant::real(types::double(), num),
                Expr::Variable(name) => {
                    match self.values.get(&name) {
                        Some(variable) => variable.clone(),
                        None => return Err(Undefined("variable")),
                    }
                },
                Expr::Binary(op, left, right) => {
                    let left = self.expr(*left)?;
                    let right = self.expr(*right)?;
                    match op {
                        BinaryOp::Plus => self.builder.fadd(left, right, "result"),
                        BinaryOp::Minus => self.builder.fsub(left, right, "result"),
                        BinaryOp::Times => self.builder.fmul(left, right, "result"),
                        BinaryOp::LessThan => {
                            let boolean = self.builder.fcmp(RealPredicate::UnorderedLesserThan, left, right, "cmptmp");
                            self.builder.unsigned_int_to_floating_point(boolean, types::double(), "booltemp")
                        },
                        _ => unimplemented!(),
                    }
                },
                Expr::Call(name, args) => {
                    match self.module.get_named_function(&name) {
                        Some(func) => {
                            if func.param_count() != args.len() {
                                return Err(WrongArgumentCount);
                            }
                            let arguments: Result<Vec<_>> = args.into_iter().map(|arg| self.expr(arg)).collect();
                            let arguments = arguments?;
                            self.builder.call(func, &arguments, "func_call")
                        },
                        None => return Err(Undefined("function")),
                    }
                },
                _ => unimplemented!("{:?}", expr),
            };
        Ok(value)
    }

    pub fn function(&mut self, function: ast::Function) -> Result<Function> {
        let llvm_function =
            match self.module.get_named_function(&function.prototype.function_name) {
                Some(llvm_function) => llvm_function,
                None => self.prototype(&function.prototype),
            };
        let entry = llvm_function.append_basic_block("entry");
        self.builder.position_at_end(entry);
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

        Ok(llvm_function)
    }

    pub fn prototype(&self, prototype: &Prototype) -> Function {
        let param_types: Vec<_> = iter::repeat(types::double()).take(prototype.parameters.len()).collect();
        let function_type = types::function::new(types::double(), &param_types, false);
        let function = self.module.add_function(&prototype.function_name, function_type);
        for (index, param) in prototype.parameters.iter().enumerate() {
            function.get_param(index).set_name(param);
        }
        function
    }
}
