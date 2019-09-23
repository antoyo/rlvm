extern crate rlvm;

mod ast;
mod error;
mod gen;
mod lexer;
mod parser;

//use std::fs::File;
use std::io::{Write, stdin, stdout};

use rlvm::{
    ExecutionEngine,
    Module,
    initialize_native_asm_printer,
    initialize_native_target,
    link_mcjit,
    llvm_init
};

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

    link_mcjit();
    initialize_native_asm_printer();
    initialize_native_target();

    //let file = File::open("tests/extern.kal")?;
    let file = stdin();
    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);
    let mut generator = Generator::new().expect("generator");
    let module = Module::new_with_name("__empty");
    let engine = ExecutionEngine::new_for_module(&module)?;
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
                match parser.definition().and_then(|definition| generator.function(definition)) {
                    Ok((module, _)) => {
                        engine.add_module(&module);
                    },
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            Token::Extern => {
                match parser.extern_().map(|prototype| generator.prototype(&prototype)) {
                    Ok(_prototype) => println!("Prototype"),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            _ => {
                match parser.toplevel().and_then(|expr| generator.function(expr)) {
                    Ok((module, function_name)) => {
                        engine.add_module(&module);
                        if let Some(function_address) = engine.get_function_address(&function_name) {
                            let func: fn() -> f64 = unsafe { function_address.cast0_ret() };
                            println!("Evaluated to {}", func());
                            engine.remove_module(&module).expect("remove module");
                        }
                        else {
                            panic!("Function not generated");
                        }
                    },
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
