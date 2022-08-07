use std::{env::{args, Args}, fs::{File, self}, vec, io::Write};

enum BinOp {
    Add,
    Sub,
    Div,
    IDiv,
    Mod,
    Pow,
    Eq,
    And,
    Less,
    LessE,
    Greater,
    GreaterE,
    Strequal,
    Lsh,
    Rsh,
    Bor,
    Band,
    Bxor,
    Max,
    Min,
    Angle,
    Len,
    Noise
}

enum UnOp {
    Not,
    Flip,
    Abs,
    Log,
    Log10,
    Floor,
    Ceil,
    Sqrt,
    Rand,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan
}

enum Token {
    Num(u64),
    BinaryOp(BinOp),
    UnaryOp(UnOp),
    Bracket,
    LParen,
    RParen,
    Comma,
    Variable(String),
    Semicolon,
    Label(String)
}

fn main() {
    let mut tokens: Vec<Token> = vec![];
    let mut arg: Args = args();

    let fname: String = arg.next().expect("Enter a file name to parse!");
    let mut code: File = File::open(&fname).expect("File doesn't exist!");
    let len: u64 = fs::metadata(&fname).expect("You don't have access to the file!").len();

    let mut out: File = File::create(match arg.next() {
        Some(s) => s,
        None => String::from("out.msm")
    }).expect("Could not create file!");

    code.write("mindustc :)".as_bytes()).expect("Unable to write to output!");
    drop(code);
}
