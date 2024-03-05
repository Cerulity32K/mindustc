use crate::is;
use crate::next;

pub fn lex(code_chars: &[char]) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut st: String = String::new();
    let mut i: usize = 0;
    while i <= code_chars.len() {
        if let Some(kw) = is::is_keyword(&st) {
            tokens.push(kw);
        } else {
            match match &st[..] {
                "#" => {
                    let start = i;
                    let end = loop {
                        if let Some('\\') = next::next_char(&mut i, &code_chars) { i += 1; continue; }
                        if let Some('#') = next::next_char(&mut i, &code_chars) {
                            break i - 1;
                        }
                    };
                    Some(Token::Preproc(st[start..end].to_string()))
                }
                "$" => {
                    let start = i;
                    let end = loop {
                        if let Some('\\') = next::next_char(&mut i, &code_chars) { i += 1; continue; }
                        if let Some('$') = next::next_char(&mut i, &code_chars) {
                            break i - 1;
                        }
                    };
                    Some(Token::InlineAsm(code_chars[start..end].iter().collect::<String>()))
                }
                "+" => Some(Token::BinaryOp(BinOp::Add)),
                "-" => Some(Token::BinaryOp(BinOp::Sub)),
                "*" => Some(Token::BinaryOp(BinOp::Mul)),
                "/" => {
                    if let Some('/') = next::next_char(&mut i, &code_chars) {
                        Some(Token::BinaryOp(BinOp::IDiv))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Div))
                    }
                }
                "%" => Some(Token::BinaryOp(BinOp::Mod)),
                "&" => {
                    if let Some('/') = next::next_char(&mut i, &code_chars) {
                        Some(Token::BinaryOp(BinOp::And))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Band))
                    }
                }
                "|" => Some(Token::BinaryOp(BinOp::Bor)),
                "^" => {
                    if let Some('^') = next::next_char(&mut i, &code_chars) {
                        Some(Token::BinaryOp(BinOp::Pow))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Bxor))
                    }
                }
                "=" => {
                    //==
                    if let Some('=') = next::next_char(&mut i, &code_chars) {
                        //===
                        if let Some('=') = next::next_char(&mut i, &code_chars) {
                            Some(Token::BinaryOp(BinOp::Streq))
                        } else {
                            i -= 1;
                            Some(Token::BinaryOp(BinOp::Eq))
                        }
                    //=
                    } else {
                        i -= 1;
                        Some(Token::Assignment)
                    }
                }
                ">" => {
                    let c: Option<char> = next::next_char(&mut i, &code_chars);
                    if let Some('=') = c {
                        Some(Token::BinaryOp(BinOp::GreaterE))
                    } else if let Some('>') = c {
                        Some(Token::BinaryOp(BinOp::Rsh))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Greater))
                    }
                }
                "<" => {
                    let c: Option<char> = next::next_char(&mut i, &code_chars);
                    if let Some('=') = c {
                        Some(Token::BinaryOp(BinOp::LessE))
                    } else if let Some('<') = c {
                        Some(Token::BinaryOp(BinOp::Lsh))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Less))
                    }
                }
                "{" => Some(Token::LBrace),
                "}" => Some(Token::RBrace),
                "[" => Some(Token::LBracket),
                "]" => Some(Token::RBracket),
                "(" => Some(Token::LParen),
                ")" => Some(Token::RParen),
                "," => Some(Token::Comma),
                ";" => Some(Token::Semicolon),
                // May be implemented as a special operator (op notEqual a a b)
                //"!" => Some(Token::UnaryOp(UnOp::Not)),
                "~" => Some(Token::UnaryOp(UnOp::Flip)),
                _ => None,
            } {
                Some(tok) => {
                    //dbg!(&tok);
                    tokens.push(tok);
                    st = String::new();
                }
                None => {
                    if let Some(ch) = next::next_char(&mut i, &code_chars) {
                        match ch {
                            '0'..='9' => {
                                i -= 1;
                                tokens.push(Token::Num(next::next_number(
                                    &mut i,
                                    &code_chars,
                                    false,
                                )));
                            }
                            '-' => {
                                while let Some(' ') = next::next_char(&mut i, &code_chars) { }
                                i -= 1;
                                if let Some(c) = next::next_char(&mut i, &code_chars) {
                                    if ('0'..'9').contains(&c) {
                                        i -= 1;
                                        tokens.push(Token::Num(next::next_number(
                                            &mut i,
                                            &code_chars,
                                            true,
                                        )));
                                    } else {
                                        tokens.push(Token::BinaryOp(BinOp::Sub));
                                        i -= 1;
                                    }
                                } else {
                                    tokens.push(Token::BinaryOp(BinOp::Sub));
                                }
                            }
                            'a'..='z' | 'A'..='Z' | '_' => {
                                i -= 1;
                                let next_raw_iden = &next::next_identifier(&mut i, &code_chars)[..];
                                if let Some(kw) = is::is_keyword(next_raw_iden) {
                                    tokens.push(kw);
                                } else {
                                    tokens.push(Token::Identifier(next_raw_iden.to_string()))
                                }
                            }
                            ' ' => st = String::new(),
                            '\n' | '\r' => {},
                            c => st.push(c),
                        }
                    }
                }
            }
        }
    }
    
    // Fix tokens (example: 20-10 is recognized as `num num`, not `num op num`)
    let mut t = 0;
    while t <= tokens.len() {
        match next::next_token(&mut t, &tokens) {
            Some(Token::Num(_) | Token::Identifier(_)) => {
                if let Some(Token::Num(n)) = next::next_token(&mut t, &tokens) {
                    if n.is_sign_negative() {
                        tokens.insert(t - 1, Token::BinaryOp(BinOp::Sub));
                        tokens[t] = Token::Num(n.abs());
                    }
                }
            }
            None => break,
            _ => {}
        }
    }
    
    tokens
}

use std::fmt::{Display, Formatter};

// Separated and sorted by precedence
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOp {
    Pow,

    Mul,
    Div,
    IDiv,
    Mod,

    Add,
    Sub,

    Lsh,
    Rsh,

    Less,
    LessE,
    Greater,
    GreaterE,

    Eq,
    Neq,
    Streq,

    Band,

    Bxor,

    Bor,

    And,

    Max,
    Min,
    Angle,
    Len,
    Noise,
}
impl BinOp {
    pub fn code(&self) -> &'static str {
        match self {
            BinOp::Pow => "pow",
            BinOp::Mul => "mul",
            BinOp::Div => "div",
            BinOp::IDiv => "idiv",
            BinOp::Mod => "mod",
            BinOp::Add => "add",
            BinOp::Sub => "sub",
            BinOp::Lsh => "shl",
            BinOp::Rsh => "shr",
            BinOp::Less => "lessThan",
            BinOp::LessE => "lessThenEq",
            BinOp::Greater => "greaterThan",
            BinOp::GreaterE => "greaterThanEq",
            BinOp::Eq => "equal",
            BinOp::Neq => "notEqual",
            BinOp::Streq => "strictEqual",
            BinOp::Band => "and",
            BinOp::Bxor => "xor",
            BinOp::Bor => "or",
            BinOp::And => "land",
            BinOp::Max => "max",
            BinOp::Min => "min",
            BinOp::Angle => "angle",
            BinOp::Len => "len",
            BinOp::Noise => "noise",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnOp {
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
    Atan,
}
impl UnOp {
    pub fn code(&self) -> &'static str {
        match self {
            UnOp::Flip => "not",
            UnOp::Abs => "abs",
            UnOp::Log => "log",
            UnOp::Log10 => "log10",
            UnOp::Floor => "floor",
            UnOp::Ceil => "ceil",
            UnOp::Sqrt => "sqrt",
            UnOp::Rand => "rand",
            UnOp::Sin => "sin",
            UnOp::Cos => "cos",
            UnOp::Tan => "tan",
            UnOp::Asin => "asin",
            UnOp::Acos => "acos",
            UnOp::Atan => "atan",
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Num(f64),
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

    Preproc(String),
    InlineAsm(String),

    If,
    While,
    Else,
}
impl Token {
    pub fn icon(&self) -> &'static str {
        match self {
            Token::InlineAsm(_) => "$",
            Token::Preproc(_) => "P",
            Token::Num(_) => "#",
            Token::BinaryOp(_) => "B",
            Token::UnaryOp(_) => "U",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::LBracket => "[",
            Token::RBracket => "]",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::Comma => ",",
            Token::Identifier(_) => "N",
            Token::Semicolon => ";",
            Token::Assignment => "=",
            Token::If => "I",
            Token::While => "W",
            Token::Else => "E",
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
