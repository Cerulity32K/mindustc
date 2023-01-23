//! Main file for MindustC. This will take in a MindustC script as an argument and output Mindustry Logic Processor assemblish.
//! Modules:
//!     types: Contains types like Token and BinOp.
//!     tokenize: Contains the code that turns a script into a list of tokens.
//!     is: Contains functions for checking script segments such as `is_keyword` and `is_iden_char`. Some return an Option, while some return a bool.
//!     next: Contains functions for stepping through the script such as `next_char` and `next_number`.
//! 

use std::{env::{args, Args}, fs::File, io::{Write, Read}};

use tokenize::tokenize;
use types::Token;

mod tokenize;
mod next;
mod is;
mod types;

fn main() {
    let mut arg: Args = args();
    arg.next();
    
    let fname: String = arg.next().expect("Enter a file name to parse!");
    let mut code: String = String::new();
    File::open(&fname).expect("File doesn't exist!").read_to_string(&mut code).unwrap();
    
    let mut out: File = File::create(match arg.next() {
        Some(s) => s,
        None => String::from("out.msm")
    }).expect("Could not create file!");
    
    let code_chars: Vec<char> = code.chars().collect();

    let tokens: Vec<Token> = tokenize(&code_chars);
    
    let mut output_str: String = String::new();
    for i in tokens {
        output_str.push_str(&format!("{i:?} "));
    }
    
    out.write(output_str.as_bytes()).expect("Unable to write to output!");
    drop(code);
}
