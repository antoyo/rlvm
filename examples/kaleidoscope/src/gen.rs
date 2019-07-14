use rlvm::{
    Builder,
    Module,
    Value,
};
use rlvm::types;
use rlvm::value::constant;

use ast::{
    Expr,
    Function,
    Prototype,
};
use error::Result;

pub struct Generator {
    builder: Builder,
}

impl Generator {
    pub fn new() -> Self {
        let module = Module::new_with_name("module");
        Self {
            builder: Builder::new(),
        }
    }

    fn expr(&self, expr: Expr) -> Result<Value> {
        let value =
            match expr {
                Expr::Number(num) => constant::real(types::double(), num),
                Expr::Variable(name) => {
                    match self.values.get(&name) {
                        Some(&variable) => self.builder.use_var(variable),
                        None => return Err(Undefined("variable")),
                    }
                },
                Expr::Binary(op, left, right) => {
                    let left = self.expr(*left)?;
                    let right = self.expr(*right)?;
                    match op {
                        BinaryOp::Plus => self.builder.ins().fadd(left, right),
                        BinaryOp::Minus => self.builder.ins().fsub(left, right),
                        BinaryOp::Times => self.builder.ins().fmul(left, right),
                        BinaryOp::LessThan => {
                            let boolean = self.builder.ins().fcmp(FloatCC::LessThan, left, right);
                            let int = self.builder.ins().bint(types::I32, boolean);
                            self.builder.ins().fcvt_from_sint(types::F64, int)
                        },
                    }
                },
                Expr::Call(name, args) => {
                    match self.functions.get(&name) {
                        Some(func) => {
                            if func.param_count != args.len() {
                                return Err(WrongArgumentCount);
                            }
                            let local_func = self.module.declare_func_in_func(func.id, &mut self.builder.func);
                            let arguments: Result<Vec<_>> = args.into_iter().map(|arg| self.expr(arg)).collect();
                            let arguments = arguments?;
                            let call = self.builder.ins().call(local_func, &arguments);
                            self.builder.inst_results(call)[0]
                        },
                        None => return Err(Undefined("function")),
                    }
                },
            };
        Ok(value)
    }

    pub fn function(&self, function: Function) -> Result<fn() -> f64> {
        Ok(|| 0.0)
    }

    pub fn prototype(&self, prototype: &Prototype) -> Result<i32> {
        Ok(0)
    }
}
