/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub l: usize,
    pub c: usize,
    pub f: Rc<String>,
}

impl Location {
    pub fn next_line(&mut self) {
        todo!()
    }

    pub fn next_symbol(&mut self) {
        todo!()
    }

    pub fn update_symbol(&mut self, _index: usize) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Unknown,
    // Single punctuation symbol
    Comma,
    Colon,
    SemiColon,
    Dot,
    Exclamation,
    LableDecl,
    // Single arithmetic symbol
    Plus,
    Dash,
    Asterix,
    Slash,
    Percent,
    Equal,
    // Single logical symbol
    Ampersand,
    Hat,
    Pipe,
    // Single grouping symbol
    LAngle,
    RAngle,
    LBrace,
    RBrace,
    LPar,
    RPar,
    LBracket,
    RBracket,
    // Literal
    Int(u64),
    Str(String),
    Bool(bool),
    Identifier(String),
    // Composed symbols
    LogicalOr,
    LogicalAnd,
    EqualOperator,
    NotEqualOperator,
    Leq,
    Geq,
    ShiftLeft,
    ShiftRight,
    // Keywords
    Sizeof,
    Namespace,
    Struct,
    Fn,
    SelfKey,
    If,
    Else,
    Loop,
    For,
    In,
    Break,
    Continue,
    Var,
    Val,
    Return,
    Unsigned,
    IntKey,
    StrKey,
    BoolKey,
}

impl Token {
    pub fn to_debug_string(&self) -> String {
        // TODO create string at the end
        match self {
            Token::Unknown => "UNKNOWN".to_string(),
            Token::Comma => ",".to_string(),
            Token::Colon => ":".to_string(),
            Token::SemiColon => ";".to_string(),
            Token::Dot => ".".to_string(),
            Token::Exclamation => "!".to_string(),
            Token::LableDecl => "'".to_string(),
            Token::Plus => "+".to_string(),
            Token::Dash => "-".to_string(),
            Token::Asterix => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Percent => "%".to_string(),
            Token::Equal => "=".to_string(),
            Token::Ampersand => "&".to_string(),
            Token::Hat => "^".to_string(),
            Token::Pipe => "|".to_string(),
            Token::LAngle => "<".to_string(),
            Token::RAngle => ">".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::LPar => "(".to_string(),
            Token::RPar => ")".to_string(),
            Token::LBracket => "[".to_string(),
            Token::RBracket => "]".to_string(),
            Token::Int(int) => format!("int/{int}"),
            Token::Str(str) => format!("str/\"{str}\""),
            Token::Bool(bool) => {
                if *bool {
                    "bool/true".to_string()
                } else {
                    "bool/false".to_string()
                }
            }
            Token::Identifier(id) => format!("id/{id}"),
            Token::LogicalOr => "||".to_string(),
            Token::LogicalAnd => "&&".to_string(),
            Token::EqualOperator => "==".to_string(),
            Token::NotEqualOperator => "!=".to_string(),
            Token::Leq => "<=".to_string(),
            Token::Geq => ">=".to_string(),
            Token::ShiftLeft => "<<".to_string(),
            Token::ShiftRight => ">>".to_string(),
            Token::Sizeof => "sizeof".to_string(),
            Token::Namespace => "namespace".to_string(),
            Token::Struct => "struct".to_string(),
            Token::Fn => "fn".to_string(),
            Token::SelfKey => "self".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Loop => "loop".to_string(),
            Token::For => "for".to_string(),
            Token::In => "in".to_string(),
            Token::Break => "break".to_string(),
            Token::Continue => "continue".to_string(),
            Token::Var => "var".to_string(),
            Token::Val => "val".to_string(),
            Token::Return => "return".to_string(),
            Token::Unsigned => "unsigned".to_string(),
            Token::IntKey => "int".to_string(),
            Token::StrKey => "str".to_string(),
            Token::BoolKey => "bool".to_string(),
        }
    }
}
