extern crate rlvm;

mod ast;
mod error;
mod gen;
mod lexer;
mod parser;

use std::fs::File;
use std::io::{Write/*, stdin*/, stdout};

use rlvm::llvm_init;

use error::Result;
use gen::Generator;
use lexer::{Lexer, Token};
use parser::Parser;

#[no_mangle]
pub extern "C" fn printd(num: f64) -> f64 {
    println!("{}", num);
    0.0
}

#[no_mangle]
pub extern "C" fn putchard(char: f64) -> f64 {
    print!("{}", char as u8 as char);
    0.0
}

fn main() -> Result<()> {
    let _llvm = llvm_init();

    let file = File::open("tests/extern.kal")?;
    //let stdin = stdin();
    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);
    let mut generator = Generator::new();
    print!("ready> ");
    stdout().flush()?;
    loop {
        let token =
            match parser.lexer.peek() {
                Ok(ref token) => *token,
                Err(error) => {
                    eprintln!("Error: {:?}", error);
                    continue;
                },
            };
        match token {
            Token::Eof => break,
            Token::SemiColon => {
                parser.lexer.next_token()?;
                continue;
            },
            Token::Def => {
                match parser.definition().map(|definition| generator.function(definition)) {
                    Ok(_definition) => (),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            Token::Extern => {
                match parser.extern_().map(|prototype| generator.prototype(&prototype)) {
                    Ok(prototype) => println!("Prototype"),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            _ => {
                match parser.toplevel().map(|expr| generator.function(expr)) {
                    Ok(function) => println!("Function"),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
        }
        print!("ready> ");
        stdout().flush()?;
    }
    Ok(())
}
