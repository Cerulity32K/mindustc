use crate::lex::{BinOp, Token, UnOp};

pub fn is_identifier_char(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

/// Tokens whose names overlap the definitions of identifiers need to be checked and overidden.
pub fn is_keyword(kw: &str) -> Option<Token> {
    match kw {
        "ang" => Some(Token::BinaryOp(BinOp::Angle)),
        "max" => Some(Token::BinaryOp(BinOp::Max)),
        "min" => Some(Token::BinaryOp(BinOp::Min)),
        "len" => Some(Token::BinaryOp(BinOp::Len)),
        "noise" => Some(Token::BinaryOp(BinOp::Noise)),
        "if" => Some(Token::If),
        "while" => Some(Token::While),
        "else" => Some(Token::Else),
        "abs" => Some(Token::UnaryOp(UnOp::Abs)),
        "ln" => Some(Token::UnaryOp(UnOp::Log)),
        "log" => Some(Token::UnaryOp(UnOp::Log10)),
        "floor" => Some(Token::UnaryOp(UnOp::Floor)),
        "ciel" => Some(Token::UnaryOp(UnOp::Ceil)),
        "sqrt" => Some(Token::UnaryOp(UnOp::Sqrt)),
        "rand" => Some(Token::UnaryOp(UnOp::Rand)),
        "sin" => Some(Token::UnaryOp(UnOp::Sin)),
        "cos" => Some(Token::UnaryOp(UnOp::Cos)),
        "tan" => Some(Token::UnaryOp(UnOp::Tan)),
        "asin" => Some(Token::UnaryOp(UnOp::Asin)),
        "acos" => Some(Token::UnaryOp(UnOp::Acos)),
        "atan" => Some(Token::UnaryOp(UnOp::Atan)),
        _ => None,
    }
}
