/*
 * Compile the object file with tests/main.c with:
 * gcc tests/main.c output.o -o main
 */

extern crate rlvm;

mod ast;
mod error;
mod gen;
mod lexer;
mod parser;

//use std::fs::File;
use std::io::{Write, stdin, stdout};

use rlvm::{
    CodeGenFileType,
    CodeGenOptLevel,
    CodeModel,
    Context,
    FunctionPassManager,
    ModulePassManager,
    RelocMode,
    Target,
    get_default_target_triple,
    initialize_native_target,
    initialize_all_target_infos,
    initialize_all_targets,
    initialize_all_target_mcs,
    initialize_all_asm_parsers,
    initialize_all_asm_printers,
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

    initialize_all_target_infos();
    initialize_all_targets();
    initialize_all_target_mcs();
    initialize_all_asm_parsers();
    initialize_all_asm_printers();
    initialize_native_target();

    let target_triple = get_default_target_triple();
    let target = Target::get_from_triple(&target_triple).expect("get target");

    let target_machine = target.create_target_machine(&target_triple, "generic", "", CodeGenOptLevel::Aggressive, RelocMode::Default, CodeModel::Default);

    //let file = File::open("tests/extern.kal")?;
    let file = stdin();
    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);
    let context = Context::new();
    let module = context.new_module("module");
    let function_pass_manager = FunctionPassManager::new_for_module(&module);
    function_pass_manager.add_promote_memory_to_register_pass();
    function_pass_manager.add_instruction_combining_pass();
    function_pass_manager.add_reassociate_pass();
    function_pass_manager.add_gvn_pass();
    function_pass_manager.add_cfg_simplification_pass();
    let module_pass_manager = ModulePassManager::new();
    module_pass_manager.add_function_inlining_pass();
    module.set_data_layout(target_machine.create_data_layout());
    module.set_target(target_triple);
    let mut generator = Generator::new(context, module, function_pass_manager, module_pass_manager).expect("generator");
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
                    Ok(()) => (),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            Token::Extern => {
                match parser.extern_().map(|prototype| generator.prototype(&prototype)) {
                    Ok(prototype) => prototype.dump(),
                    Err(error) => {
                        parser.lexer.next_token()?;
                        eprintln!("Error: {:?}", error);
                    },
                }
            },
            _ => {
                match parser.toplevel().and_then(|expr| generator.function(expr)) {
                    Ok(()) => (),
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
    println!("Writing output.o");
    if let Err(error) = target_machine.emit_to_file(&generator.module, "output.o", CodeGenFileType::ObjectFile) {
        eprintln!("Cannot emit to object: {}", error);
    }
    Ok(())
}
