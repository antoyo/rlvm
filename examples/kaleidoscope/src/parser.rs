use std::collections::HashMap;
use std::io::Read;

use crate::ast::{
    BinaryOp,
    Declaration,
    Expr,
    Function,
    Prototype,
};
use crate::error::Result;
use crate::error::Error::{Undefined, Unexpected};
use crate::lexer::{Lexer, Token};

fn operator_token(token: &Token) -> Option<char> {
    let char =
        match token {
            Token::LessThan => '<',
            Token::Minus => '-',
            Token::Plus => '+',
            Token::Star => '*',
            Token::Equal => '=',
            Token::Exclamation => '!',
            Token::GreaterThan => '>',
            Token::Pipe => '|',
            Token::Ampersand => '&',
            Token::Colon => ':',
            _ => return None,
        };
    Some(char)
}

pub struct Parser<R: Read> {
    bin_precedence: HashMap<BinaryOp, i32>,
    index: usize,
    pub lexer: Lexer<R>,
}

impl<R: Read> Parser<R> {
    pub fn new(lexer: Lexer<R>) -> Self {
        let mut bin_precedence = HashMap::new();
        bin_precedence.insert(BinaryOp::Equal, 2);
        bin_precedence.insert(BinaryOp::LessThan, 10);
        bin_precedence.insert(BinaryOp::Plus, 20);
        bin_precedence.insert(BinaryOp::Minus, 20);
        bin_precedence.insert(BinaryOp::Times, 40);
        Self {
            bin_precedence,
            index: 0,
            lexer,
        }
    }

    fn args(&mut self) -> Result<Vec<Expr>> {
        let mut args = vec![self.expr()?];
        while *self.lexer.peek()? == Token::Comma {
            self.eat(Token::Comma)?;
            args.push(self.expr()?);
        }
        Ok(args)
    }

    fn binary_op(&mut self) -> Result<Option<BinaryOp>> {
        let op =
            match self.lexer.peek()? {
                Token::Equal => BinaryOp::Equal,
                Token::LessThan => BinaryOp::LessThan,
                Token::Minus => BinaryOp::Minus,
                Token::Plus => BinaryOp::Plus,
                Token::Star => BinaryOp::Times,
                token => {
                    return Ok(operator_token(token)
                        .map(|char| BinaryOp::Custom(char)));
                },
            };
        Ok(Some(op))
    }

    fn binary_right(&mut self, expr_precedence: i32, left: Expr) -> Result<Expr> {
        match self.binary_op()? {
            Some(op) => {
                let token_precedence = self.precedence(op)?;
                if token_precedence < expr_precedence {
                    Ok(left)
                }
                else {
                    self.lexer.next_token()?; // Eat binary operator.
                    let right = self.unary()?;
                    let right =
                        match self.binary_op()? {
                            Some(op) => {
                                if token_precedence < self.precedence(op)? {
                                    self.binary_right(token_precedence + 1, right)?
                                }
                                else {
                                    right
                                }
                            },
                            None => right,
                        };
                    let left = Expr::Binary(op, Box::new(left), Box::new(right));
                    self.binary_right(expr_precedence, left)
                }
            },
            None => Ok(left),
        }
    }

    pub fn definition(&mut self) -> Result<Function> {
        self.eat(Token::Def)?;
        let prototype = self.prototype()?;
        let body = self.expr()?;
        Ok(Function {
            body,
            prototype,
        })
    }

    fn eat(&mut self, token: Token) -> Result<()> {
        let current_token = self.lexer.next_token()?;
        if current_token != token {
            return Err(Unexpected("token"));
        }
        Ok(())
    }

    fn expr(&mut self) -> Result<Expr> {
        let left = self.unary()?;
        self.binary_right(0, left)
    }

    pub fn extern_(&mut self) -> Result<Prototype> {
        self.eat(Token::Extern)?;
        self.prototype()
    }

    fn ident(&mut self) -> Result<String> {
        match self.lexer.next_token()? {
            Token::Identifier(ident) => Ok(ident),
            _ => Err(Unexpected("token, expecting identifier")),
        }
    }

    fn ident_expr(&mut self) -> Result<Expr> {
        let name = self.ident()?;
        let ast =
            match self.lexer.peek()? {
                Token::OpenParen => {
                    self.eat(Token::OpenParen)?;
                    let args = self.args()?;
                    self.eat(Token::CloseParen)?;
                    Expr::Call(name, args)
                },
                _ => Expr::Variable(name),
            };
        Ok(ast)
    }

    fn parameters(&mut self) -> Result<Vec<String>> {
        let mut params = vec![];
        loop {
            match *self.lexer.peek()? {
                Token::Identifier(_) => {
                    let ident =
                        match self.lexer.next_token()? {
                            Token::Identifier(ident) => ident,
                            _ => unreachable!(),
                        };
                    params.push(ident);
                },
                _ => break,
            }
        }
        Ok(params)
    }

    fn precedence(&self, op: BinaryOp) -> Result<i32> {
        match self.bin_precedence.get(&op) {
            Some(&precedence) => Ok(precedence),
            None => Err(Undefined("operator")),
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        match *self.lexer.peek()? {
            Token::Number(number) => {
                self.lexer.next_token()?;
                Ok(Expr::Number(number))
            },
            Token::OpenParen => {
                self.eat(Token::OpenParen)?;
                let expr = self.expr()?;
                self.eat(Token::CloseParen)?;
                Ok(expr)
            },
            Token::Identifier(_) => self.ident_expr(),
            Token::If => {
                self.eat(Token::If)?;
                let condition = Box::new(self.expr()?);
                self.eat(Token::Then)?;
                let then = Box::new(self.expr()?);
                self.eat(Token::Else)?;
                let else_ = Box::new(self.expr()?);
                Ok(Expr::If {
                    condition,
                    then,
                    else_,
                })
            },
            Token::For => {
                self.eat(Token::For)?;
                let variable_name = self.ident()?;
                self.eat(Token::Equal)?;
                let init_value = Box::new(self.expr()?);
                self.eat(Token::Comma)?;
                let condition = Box::new(self.expr()?);

                let step =
                    match self.lexer.peek()? {
                        Token::Comma => {
                            self.lexer.next_token()?;
                            Some(Box::new(self.expr()?))
                        },
                        _ => None,
                    };

                self.eat(Token::In)?;

                let body = Box::new(self.expr()?);

                Ok(Expr::For {
                    body,
                    condition,
                    init_value,
                    step,
                    variable_name,
                })
            },
            Token::Var => {
                self.eat(Token::Var)?;
                let mut declarations = vec![];
                loop {
                    let name = self.ident()?;
                    let init_value =
                        match self.lexer.peek()? {
                            Token::Equal => {
                                self.eat(Token::Equal)?;
                                Some(Box::new(self.expr()?))
                            },
                            _ => None,
                        };
                    declarations.push(Declaration {
                        name,
                        init_value,
                    });
                    if self.lexer.peek()? != &Token::Comma {
                        break;
                    }
                    self.eat(Token::Comma)?;
                }
                self.eat(Token::In)?;
                let body = Box::new(self.expr()?);
                Ok(Expr::VariableDeclaration {
                    declarations,
                    body,
                })
            },
            _ => Err(Unexpected("token when expecting an expression")),
        }
    }

    fn prototype(&mut self) -> Result<Prototype> {
        #[derive(PartialEq)]
        enum Type {
            Binary,
            Function,
            Unary,
        }

        let (mut function_name, typ) =
            match self.lexer.next_token()? {
                Token::Identifier(ident) => (ident, Type::Function),
                Token::Binary => ("binary".to_string(), Type::Binary),
                Token::Unary => ("unary".to_string(), Type::Unary),
                _ => return Err(Unexpected("token when expecting function name in prototype")),
            };

        let (precedence, op) =
            if typ == Type::Binary || typ == Type::Unary {
                let operator =
                    match operator_token(self.lexer.peek()?) {
                        Some(operator) => {
                            self.lexer.next_token()?;
                            function_name.push(operator);
                            operator
                        },
                        None => return Err(Unexpected("operator")),
                    };

                let precedence =
                    match *self.lexer.peek()? {
                        Token::Number(number) => {
                            self.lexer.next_token()?;
                            number as i32
                        },
                        _ => 30,
                    };
                (precedence, operator)
            }
            else {
                (30, '\0')
            };

        self.eat(Token::OpenParen)?;
        let parameters = self.parameters()?;
        self.eat(Token::CloseParen)?;

        if typ == Type::Binary {
            self.bin_precedence.insert(BinaryOp::Custom(op), precedence);
        }
        Ok(Prototype {
            function_name,
            parameters,
        })
    }

    pub fn toplevel(&mut self) -> Result<Function> {
        let body = self.expr()?;
        self.index += 1;
        Ok(Function {
            body,
            prototype: Prototype {
                function_name: format!("__anon_{}", self.index),
                parameters: vec![],
            },
        })
    }

    fn unary(&mut self) -> Result<Expr> {
        match operator_token(self.lexer.peek()?) {
            Some(op) => {
                self.lexer.next_token()?;
                Ok(Expr::Unary(op, Box::new(self.expr()?)))
            },
            None => self.primary(),
        }
    }
}
