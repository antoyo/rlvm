#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum BinaryOp {
    Equal,
    LessThan,
    Minus,
    Plus,
    Times,
    Custom(char),
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub init_value: Option<Box<Expr>>,
}

#[derive(Debug)]
pub enum Expr {
    Unary(char, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    For {
        body: Box<Expr>,
        variable_name: String,
        init_value: Box<Expr>,
        condition: Box<Expr>,
        step: Option<Box<Expr>>,
    },
    If {
        condition: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    Number(f64),
    Variable(String),
    VariableDeclaration {
        declarations: Vec<Declaration>,
        body: Box<Expr>,
    },
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expr,
}

#[derive(Debug)]
pub struct Prototype {
    pub function_name: String,
    pub parameters: Vec<String>,
}
