use crate::next::next_char;
use crate::types::Token;
use crate::next;
use crate::is;
use crate::types::BinOp;
use crate::types::UnOp;

pub fn tokenize(code_chars: &Vec<char>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut st: String = String::new();
    let mut i: usize = 0;
    while i <= code_chars.len() {
        if let Some(kw) = is::is_keyword(&st) {
            tokens.push(kw);
        } else {
            match match &st[..] {
                "+" => Some(Token::BinaryOp(BinOp::Add)),
                "-" => Some(Token::BinaryOp(BinOp::Sub)),
                "*" => Some(Token::BinaryOp(BinOp::Mul)),
                "/" => {
                    if let Some('/') = next_char(&mut i, &code_chars) {
                        Some(Token::BinaryOp(BinOp::IDiv))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Div))
                    }
                },
                "%" => Some(Token::BinaryOp(BinOp::Mod)),
                "&" => {
                    if let Some('/') = next_char(&mut i, &code_chars) {
                        Some(Token::BinaryOp(BinOp::And))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Band))
                    }
                },
                "|" => Some(Token::BinaryOp(BinOp::Bor)),
                "^" => {
                    if let Some('^') = next_char(&mut i, &code_chars) {
                        Some(Token::BinaryOp(BinOp::Pow))
                    } else {
                        i -= 1;
                        Some(Token::BinaryOp(BinOp::Bxor))
                    }
                },
                "=" => {
                    //==
                    if let Some('=') = next_char(&mut i, &code_chars) {
                        //===
                        if let Some('=') = next_char(&mut i, &code_chars) {
                            Some(Token::BinaryOp(BinOp::Strequal))
                        } else {
                            i -= 1;
                            Some(Token::BinaryOp(BinOp::Eq))
                        }
                    //=
                    } else {
                        i -= 1;
                        Some(Token::Assignment)
                    }
                },
                ">" => {
                    let c: Option<char> = next_char(&mut i, &code_chars);
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
                    let c: Option<char> = next_char(&mut i, &code_chars);
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
                "!" => Some(Token::UnaryOp(UnOp::Not)),
                "~" => Some(Token::UnaryOp(UnOp::Flip)),
                _ => None
            } {
                Some(tok) => {
                    dbg!(&tok);
                    tokens.push(tok);
                    st = String::new();
                }
                None => {
                    if let Some(ch) = next_char(&mut i, &code_chars) {
                        match ch {
                            '0'..='9' => {
                                i -= 1;
                                tokens.push(Token::Num(next::next_number(&mut i, &code_chars, false)));
                            },
                            '-' => {
                                if let Some(c) = next_char(&mut i, &code_chars) {
                                    if ('0'..'9').contains(&c) {
                                        i -= 1;
                                        tokens.push(Token::Num(next::next_number(&mut i, &code_chars, true)));
                                    } else {
                                        i -= 1;
                                    }
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
                            c => st.push(c)
                        }
                    }
                }
            }
        }
    }
    tokens
}