use std::{collections::BTreeMap, fmt::Display};

use crate::lex::{BinOp, Token, UnOp};

use self::expr::Expression;

pub enum FunctionType {
    Intern,
    Macro,
}

pub struct Function {
    ftype: FunctionType,
    fname: String
}

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    Num(f64),
}
impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::Identifier(ident) => ident.clone(),
            Self::Num(n) => n.to_string(),
        }
    }
}

pub enum Statement {
    Assignment(String, Box<Expression>),
    Expression(Expression),
}
pub mod expr {
    use std::{fmt::Display, collections::HashMap};

    use crate::lex::{BinOp, Token, UnOp};

    use super::Value;

    pub enum VarStorage {
        Identifier(String),
        Register(usize)
    }
    impl VarStorage {
        pub fn next(&self) -> VarStorage {
            match self {
                Self::Identifier(_) => Self::Register(0),
                Self::Register(r) => Self::Register(r + 1)
            }
        }
    }
    impl Display for VarStorage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Self::Identifier(i) => i.clone(),
                Self::Register(r) => format!("r{r}")
            })
        }
    }
    #[derive(Debug)]
    pub enum IR {
        Bop(BinOp, String, String, String),
        Uop(UnOp, String, String),
        Set(String, String),
        CallJump(String),
        InlineLogic(String),
    }
    impl IR {
        pub fn make_code(ir: &[IR], functions: &HashMap<String, usize>) -> String {
            ir.iter().map(|fragment| {
                match fragment {
                    IR::Bop(op, dest, left, right) => format!("op {} {dest} {left} {right}", op.code()),
                    IR::Uop(op, dest, operand) => format!("op {} {dest} {operand} _", op.code()),
                    IR::Set(dest, src) => format!("set {dest} {src}"),
                    IR::CallJump(fname) => format!("set @counter {}", functions[fname]),
                    IR::InlineLogic(logic) => logic.clone(),
                }
            }).intersperse(format!("\n")).collect::<String>()
        }
    }
    #[derive(Debug, Clone)]
    pub enum Expression {
        /// A leaf in an expression tree. Either an identifier or constant.
        Value(Value),
        /// A binary operation between two expression fragments.
        Binary(Box<Expression>, BinOp, Box<Expression>),
        /// A unary operation on an expression fragment.
        Unary(UnOp, Box<Expression>),
        /// A call to a function with a list of expressions
        Call(String, Vec<Expression>),
        /// Inline logic
        InlineLogic(String),
        
        /// Not used for actual expressions, meant for tree building. Represents a floating binary operation.
        BinaryOp(BinOp),
        /// Not used for actual expressions, meant for tree building. Represents a floating unary operation.
        UnaryOp(UnOp),
    }
    impl Expression {
        pub fn make_ast(tokens: &[Token]) -> Option<Expression> {
            let precedence_order: Vec<&[BinOp]> = vec![
                &[BinOp::Pow],
                &[BinOp::Mul, BinOp::Div, BinOp::IDiv, BinOp::Mod],
                &[BinOp::Add, BinOp::Sub],
                &[BinOp::Lsh, BinOp::Rsh],
                &[BinOp::Less, BinOp::LessE, BinOp::Greater, BinOp::GreaterE],
                &[BinOp::Eq, BinOp::Streq],
                &[BinOp::Band],
                &[BinOp::Bxor],
                &[BinOp::Bor],
                &[BinOp::And],
                &[BinOp::Max, BinOp::Min, BinOp::Angle, BinOp::Len, BinOp::Noise]
            ];
            
            let mut exprs = Vec::with_capacity(tokens.len());
            let mut tok_idx = 0;
            // Converts tokens into expression tokens, grouping up parentheses.
            while let Some(token) = tokens.get(tok_idx) {
                tok_idx += 1;
                match token {
                    Token::Identifier(ident) => {
                        // IPEC...CEP is a function call
                        if let Some(Token::LParen) = tokens.get(tok_idx) {
                            tok_idx += 1;
                            let mut start = tok_idx;
                            let mut nesting = 0;
                            let mut call_args = vec![];
                            loop {
                                match dbg!(tokens.get(tok_idx)) {
                                    Some(Token::LParen) => { nesting += 1; }
                                    Some(Token::RParen) => {
                                        if nesting == 0 {
                                            call_args.push(Self::make_ast(&tokens[start..tok_idx])?);
                                            break;
                                        } else {
                                            nesting -= 1;
                                        }
                                    }
                                    Some(Token::Comma) => {
                                        // Topmost parentheses pair
                                        if nesting == 0 {
                                            call_args.push(Self::make_ast(&tokens[start..tok_idx])?);
                                            start = tok_idx;
                                        }
                                    }
                                    None => break,
                                    _ => {}
                                }
                                tok_idx += 1;
                            }
                            exprs.push(Self::Call(ident.clone(), call_args));
                        } else {
                            exprs.push(Expression::Value(Value::Identifier(ident.clone())));
                        }
                    },
                    Token::Num(n) => exprs.push(Expression::Value(Value::Num(*n))),
    
                    Token::BinaryOp(op) => exprs.push(Expression::BinaryOp(*op)),
                    Token::UnaryOp(op) => exprs.push(Expression::UnaryOp(*op)),
    
                    Token::LParen => {
                        let start = tok_idx;
                        let mut nesting = 0;
                        loop {
                            match tokens.get(tok_idx) {
                                Some(Token::LParen) => { nesting += 1; }
                                Some(Token::RParen) => {
                                    if nesting == 0 {
                                        exprs.push(Self::make_ast(&tokens[start..tok_idx])?);
                                    } else {
                                        nesting -= 1;
                                    }
                                }
                                None => break,
                                _ => {}
                            }
                            tok_idx += 1;
                        }
                    }

                    Token::InlineAsm(logic) => exprs.push(Expression::InlineLogic(logic.to_string())),
    
                    _ => continue
                };
            }
            Self::merge_exprs(exprs, &precedence_order)
        }
        pub fn merge_exprs(mut exprs: Vec<Expression>, precedence_order: &[&[BinOp]]) -> Option<Expression> {
            // Merge unary before binary operations
            // Merge binary operations
            for &precedence_group in precedence_order {
                let mut expr_idx = 1;
                while expr_idx < exprs.len() {
                    match exprs[expr_idx] {
                        Expression::BinaryOp(op) => {
                            if precedence_group.contains(&op) {
                                let (left, right) = (
                                    exprs.remove(expr_idx - 1),
                                    // dont add 1, the right element was moved back
                                    exprs.remove(expr_idx),
                                );
                                exprs[expr_idx - 1] = Expression::Binary(Box::new(left), op, Box::new(right));
                                expr_idx -= 1;
                            }
                        }
                        _ => {}
                    }
                    expr_idx += 1;
                }
            }
            Some(exprs.remove(0))
        }
        pub fn generate_ir(&self, storage: &VarStorage) -> Vec<IR> {
            match self {
                Self::Binary(left, op, right) => {
                    let mut collected_ir: Vec<IR> = vec![];
    
                    // If the arguments are `Value`s, put them directly into the operations
                    // instead of in registers first
                    let left_arg = if let Expression::Value(v) = &**left {
                        v.to_string()
                    } else {
                        collected_ir.append(&mut left.generate_ir(storage));
                        format!("{storage}")
                    };
    
                    let right_arg = if let Expression::Value(v) = &**right {
                        v.to_string()
                    } else {
                        collected_ir.append(&mut right.generate_ir(&storage.next()));
                        format!("{}", storage.next())
                    };
    
                    collected_ir.push(IR::Bop(*op, storage.to_string(), left_arg, right_arg));
                    collected_ir
                }
                Self::Unary(op, val) => {
                    if let Expression::Value(v) = &**val {
                        vec![IR::Uop(*op, storage.to_string(), v.to_string())]
                    } else {
                        let mut v = val.generate_ir(storage);
                        v.push(IR::Uop(*op, storage.to_string(), storage.next().to_string()));
                        v
                    }
                }
                Self::Call(ident, args) => {
                    let mut call_ir = vec![];
                    for (idx, expr) in args.iter().enumerate() {
                        call_ir.append(&mut expr.generate_ir(&VarStorage::Identifier(format!("s{idx}"))));
                    }
                    call_ir.push(IR::Bop(BinOp::Add, "ret".to_string(), "@counter".to_string(), "2".to_string()));
                    call_ir.push(IR::CallJump(ident.clone()));
                    call_ir
                }
                Self::Value(v) => vec![IR::Set(storage.to_string(), v.to_string())],
                Self::InlineLogic(logic) => vec![IR::InlineLogic(logic.clone())],
                // We should not come across values as their generation is suppressed.
                Self::BinaryOp(_) | Self::UnaryOp(_) => unreachable!()
            }
    
            /*match self {
                Self::Binary(left, op, right) => format!(
                    "{}\n{}\n{}",
                    left.generate_code(register),
                    right.generate_code(register + 1),
                    format!("op {} r{register} r{register} r{}", op.code(), register + 1)
                ),
                Self::Unary(op, ex) => format!("op {} r{register} {}\n{}", op.code(), ex.repr(register + 1), ex.generate_code(register + 1)),
                Self::Value(v) => format!("set r{register} {}", v.to_string()),
                _ => unreachable!()
            }*/
        }
        pub fn repr(&self, register: usize) -> String {
            match self {
                Self::Value(v) => v.to_string(),
                _ => format!("r{register}")
            }
        }
    }
}
