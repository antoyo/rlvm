use std::collections::HashMap;
use std::iter;
use std::mem;

use rlvm::{
    Builder,
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
        Ok(Self {
            builder: Builder::new(),
            function_prototypes: HashMap::new(),
            module,
            pass_manager,
            values: HashMap::new(),
        })
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
                    match self.get_named_function(&name) {
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

    pub fn function(&mut self, function: ast::Function) -> Result<(Module, String)> {
        let name = function.prototype.function_name.clone();
        let llvm_function =
            match self.module.get_named_function(&name) {
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

        self.function_prototypes.insert(name.clone(), function.prototype);

        self.builder.ret(return_value);
        llvm_function.verify(VerifierFailureAction::AbortProcess);

        self.pass_manager.run(&llvm_function);

        self.module.dump();

        let (module, pass_manager) = new_module();
        let module = mem::replace(&mut self.module, module);
        self.pass_manager = pass_manager;

        Ok((module, name))
    }

    pub fn prototype(&self, prototype: &Prototype) -> Function {
        let param_types: Vec<_> = iter::repeat(types::double()).take(prototype.parameters.len()).collect();
        let function_type = types::function::new(types::double(), &param_types, false);
        let function = self.add_function(&prototype.function_name, function_type);
        for (index, param) in prototype.parameters.iter().enumerate() {
            function.get_param(index).set_name(param);
        }
        function
    }

    fn add_function(&self, name: &str, typ: Type) -> Function {
        self.module.add_function(&name, typ)
    }

    fn get_named_function(&self, name: &str) -> Option<Function> {
        match self.module.get_named_function(&name) {
            Some(function) => Some(function),
            None => {
                if let Some(prototype) = self.function_prototypes.get(name) {
                    Some(self.prototype(prototype))
                }
                else {
                    None
                }
            },
        }
    }
}
