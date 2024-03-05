//! Main file for MindustC. This will take in a MindustC script as an argument and output Mindustry Logic Processor logic.
//! Modules:
//!     types: Contains types like Token and BinOp.
//!     tokenize: Contains the code that turns a script into a list of tokens.
//!     is: Contains functions for checking script segments such as `is_keyword` and `is_iden_char`. Some return an Option, while some return a bool.
//!     next: Contains functions for stepping through the script such as `next_char` and `next_number`.
//!

#![feature(iter_intersperse)]

use std::{
    collections::HashMap, env::{args, Args}, error::Error, fs::File, io::{BufWriter, Read, Write}
};

use lex::{Token, lex};
use parse::expr::{Expression, VarStorage, IR};

mod is;
mod next;
mod lex;
mod parse;
mod stmt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut argv: Args = args();
    argv.next();

    let fname = argv.next().expect("Enter a file name to parse!");
    let mut code = String::new();
    File::open(&fname)
        .expect("File doesn't exist!")
        .read_to_string(&mut code)?;

    let mut out = BufWriter::new(File::create(
        argv.next().unwrap_or_else(||format!("out.msm"))
    )?);

    let code_chars = code.chars().collect::<Vec<char>>();

    let tokens = lex(&code_chars);

    let mut function_table = HashMap::new();
    function_table.insert("sq".to_string(), 0);

    for i in tokens.split(|t| t == &Token::Semicolon) {
        match i {
            [Token::Identifier(ident), Token::Assignment, expr @ ..] => {
                out.write(IR::make_code(&Expression::make_ast(expr).unwrap().generate_ir(&VarStorage::Identifier(ident.clone())), &function_table).as_bytes())?;
                out.write(&[b'\n'])?;
            }
            [Token::InlineAsm(logic)] => {
                out.write(logic.as_bytes())?;
                out.write(&[b'\n'])?;
            }
            _ => {}
        }
    }

    /*println!("Making AST...");
    let st = "6/2*(1+sq(2, 4))";
    let expr = Expression::make_ast(&lex(&st.chars().collect::<Vec<char>>()));
    println!("Input: {st}");
    println!("Generated AST: {expr:?}");
    println!("Output: {}", expr.unwrap().generate_code(&VarStorage::Identifier(String::from("result")), &function_table));*/

    /*for i in tokens {
        write!(out, "{i:?} ")?;
    }*/

    /*out.write(output_str.as_bytes())
        .expect("Unable to write to output!");
    drop(code);*/
    Ok(())
}
