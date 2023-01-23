use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
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
    Noise/* */
}

#[derive(Debug)]
pub enum UnOp {
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

#[derive(Debug)]
pub enum Token {
    Num(f32),
    BinaryOp(BinOp),
    UnaryOp(UnOp),
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Comma,
    Identifier(String),
    Semicolon,
    Assignment,
    If,
    While,
    Else
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
