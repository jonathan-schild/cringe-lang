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

#[derive(Debug)]
pub enum Token {
    Unknown { l: Location },
    // Single punctuation symbol
    Comma { l: Location },
    Colon { l: Location },
    SemiColon { l: Location },
    Dot { l: Location },
    Exclamation { l: Location },
    LableDecl { l: Location },
    // Single arithmetic symbol
    Plus { l: Location },
    Dash { l: Location },
    Asterix { l: Location },
    Slash { l: Location },
    Percent { l: Location },
    Equal { l: Location },
    // Single logical symbol
    Ampersand { l: Location },
    Head { l: Location },
    Pipe { l: Location },
    // Single grouping symbol
    LAngle { l: Location },
    RAngle { l: Location },
    LBrace { l: Location },
    RBrace { l: Location },
    LPar { l: Location },
    RPar { l: Location },
    LBracket { l: Location },
    RBracket { l: Location },
    // Literal
    Int { int: u64, l: Location },
    Str { str: String, l: Location },
    Bool { bool: bool, l: Location },
    Identifier { id: String, l: Location },
    // Composed symbols
    LogicalOr { l: Location },
    LogicalAnd { l: Location },
    EqualOperator { l: Location },
    NotEqualOperator { l: Location },
    Leq { l: Location },
    Geq { l: Location },
    ShiftLeft { l: Location },
    ShiftRight { l: Location },
    // Keywords
    Sizeof { l: Location },
    Namespace { l: Location },
    Struct { l: Location },
    Fn { l: Location },
    SelfKey { l: Location },
    If { l: Location },
    Else { l: Location },
    Loop { l: Location },
    For { l: Location },
    In { l: Location },
    Break { l: Location },
    Continue { l: Location },
    Var { l: Location },
    Val { l: Location },
    Return { l: Location },
    Unsigned { l: Location },
    IntKey { l: Location },
    StrKey { l: Location },
    BoolKey { l: Location },
}

impl Token {
    pub fn to_debug_string(&self) -> String {
        // TODO create string at the end
        match self {
            Token::Unknown { l: _ } => "UNKNOWN".to_string(),
            Token::Comma { l: _ } => ",".to_string(),
            Token::Colon { l: _ } => ":".to_string(),
            Token::SemiColon { l: _ } => ";".to_string(),
            Token::Dot { l: _ } => ".".to_string(),
            Token::Exclamation { l: _ } => "!".to_string(),
            Token::LableDecl { l: _ } => "'".to_string(),
            Token::Plus { l: _ } => "+".to_string(),
            Token::Dash { l: _ } => "-".to_string(),
            Token::Asterix { l: _ } => "*".to_string(),
            Token::Slash { l: _ } => "/".to_string(),
            Token::Percent { l: _ } => "%".to_string(),
            Token::Equal { l: _ } => "=".to_string(),
            Token::Ampersand { l: _ } => "&".to_string(),
            Token::Head { l: _ } => "^".to_string(),
            Token::Pipe { l: _ } => "|".to_string(),
            Token::LAngle { l: _ } => "<".to_string(),
            Token::RAngle { l: _ } => ">".to_string(),
            Token::LBrace { l: _ } => "{".to_string(),
            Token::RBrace { l: _ } => "}".to_string(),
            Token::LPar { l: _ } => "(".to_string(),
            Token::RPar { l: _ } => ")".to_string(),
            Token::LBracket { l: _ } => "[".to_string(),
            Token::RBracket { l: _ } => "]".to_string(),
            Token::Int { int, l: _ } => format!("int/{int}"),
            Token::Str { str, l: _ } => format!("str/\"{str}\""),
            Token::Bool { bool, l: _ } => {
                if *bool {
                    "bool/true".to_string()
                } else {
                    "bool/false".to_string()
                }
            }
            Token::Identifier { id, l: _ } => format!("id/{id}"),
            Token::LogicalOr { l: _ } => "||".to_string(),
            Token::LogicalAnd { l: _ } => "&&".to_string(),
            Token::EqualOperator { l: _ } => "==".to_string(),
            Token::NotEqualOperator { l: _ } => "!=".to_string(),
            Token::Leq { l: _ } => "<=".to_string(),
            Token::Geq { l: _ } => ">=".to_string(),
            Token::ShiftLeft { l: _ } => "<<".to_string(),
            Token::ShiftRight { l: _ } => ">>".to_string(),
            Token::Sizeof { l: _ } => "sizeof".to_string(),
            Token::Namespace { l: _ } => "namespace".to_string(),
            Token::Struct { l: _ } => "struct".to_string(),
            Token::Fn { l: _ } => "fn".to_string(),
            Token::SelfKey { l: _ } => "self".to_string(),
            Token::If { l: _ } => "if".to_string(),
            Token::Else { l: _ } => "else".to_string(),
            Token::Loop { l: _ } => "loop".to_string(),
            Token::For { l: _ } => "for".to_string(),
            Token::In { l: _ } => "in".to_string(),
            Token::Break { l: _ } => "break".to_string(),
            Token::Continue { l: _ } => "continue".to_string(),
            Token::Var { l: _ } => "var".to_string(),
            Token::Val { l: _ } => "val".to_string(),
            Token::Return { l: _ } => "return".to_string(),
            Token::Unsigned { l: _ } => "unsigned".to_string(),
            Token::IntKey { l: _ } => "int".to_string(),
            Token::StrKey { l: _ } => "str".to_string(),
            Token::BoolKey { l: _ } => "bool".to_string(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unknown { l: _ }, Self::Unknown { l: _ }) => true,
            (Self::Comma { l: _ }, Self::Comma { l: _ }) => true,
            (Self::Colon { l: _ }, Self::Colon { l: _ }) => true,
            (Self::SemiColon { l: _ }, Self::SemiColon { l: _ }) => true,
            (Self::Dot { l: _ }, Self::Dot { l: _ }) => true,
            (Self::Exclamation { l: _ }, Self::Exclamation { l: _ }) => true,
            (Self::LableDecl { l: _ }, Self::LableDecl { l: _ }) => true,
            (Self::Plus { l: _ }, Self::Plus { l: _ }) => true,
            (Self::Dash { l: _ }, Self::Dash { l: _ }) => true,
            (Self::Asterix { l: _ }, Self::Asterix { l: _ }) => true,
            (Self::Slash { l: _ }, Self::Slash { l: _ }) => true,
            (Self::Percent { l: _ }, Self::Percent { l: _ }) => true,
            (Self::Equal { l: _ }, Self::Equal { l: _ }) => true,
            (Self::Ampersand { l: _ }, Self::Ampersand { l: _ }) => true,
            (Self::Head { l: _ }, Self::Head { l: _ }) => true,
            (Self::Pipe { l: _ }, Self::Pipe { l: _ }) => true,
            (Self::LAngle { l: _ }, Self::LAngle { l: _ }) => true,
            (Self::RAngle { l: _ }, Self::RAngle { l: _ }) => true,
            (Self::LBrace { l: _ }, Self::LBrace { l: _ }) => true,
            (Self::RBrace { l: _ }, Self::RBrace { l: _ }) => true,
            (Self::LPar { l: _ }, Self::LPar { l: _ }) => true,
            (Self::RPar { l: _ }, Self::RPar { l: _ }) => true,
            (Self::LBracket { l: _ }, Self::LBracket { l: _ }) => true,
            (Self::RBracket { l: _ }, Self::RBracket { l: _ }) => true,
            (Self::Int { int: l_int, l: _ }, Self::Int { int: r_int, l: _ }) => l_int == r_int,
            (Self::Str { str: l_str, l: _ }, Self::Str { str: r_str, l: _ }) => l_str == r_str,
            (Self::Bool { bool: l_bool, l: _ }, Self::Bool { bool: r_bool, l: _ }) => {
                l_bool == r_bool
            }
            (Self::Identifier { id: l_id, l: _ }, Self::Identifier { id: r_id, l: _ }) => {
                l_id == r_id
            }
            (Self::LogicalOr { l: _ }, Self::LogicalOr { l: _ }) => true,
            (Self::LogicalAnd { l: _ }, Self::LogicalAnd { l: _ }) => true,
            (Self::EqualOperator { l: _ }, Self::EqualOperator { l: _ }) => true,
            (Self::NotEqualOperator { l: _ }, Self::NotEqualOperator { l: _ }) => true,
            (Self::Leq { l: _ }, Self::Leq { l: _ }) => true,
            (Self::Geq { l: _ }, Self::Geq { l: _ }) => true,
            (Self::ShiftLeft { l: _ }, Self::ShiftLeft { l: _ }) => true,
            (Self::ShiftRight { l: _ }, Self::ShiftRight { l: _ }) => true,
            (Self::Sizeof { l: _ }, Self::Sizeof { l: _ }) => true,
            (Self::Namespace { l: _ }, Self::Namespace { l: _ }) => true,
            (Self::Struct { l: _ }, Self::Struct { l: _ }) => true,
            (Self::Fn { l: _ }, Self::Fn { l: _ }) => true,
            (Self::SelfKey { l: _ }, Self::SelfKey { l: _ }) => true,
            (Self::If { l: _ }, Self::If { l: _ }) => true,
            (Self::Else { l: _ }, Self::Else { l: _ }) => true,
            (Self::Loop { l: _ }, Self::Loop { l: _ }) => true,
            (Self::For { l: _ }, Self::For { l: _ }) => true,
            (Self::In { l: _ }, Self::In { l: _ }) => true,
            (Self::Break { l: _ }, Self::Break { l: _ }) => true,
            (Self::Continue { l: _ }, Self::Continue { l: _ }) => true,
            (Self::Var { l: _ }, Self::Var { l: _ }) => true,
            (Self::Val { l: _ }, Self::Val { l: _ }) => true,
            (Self::Return { l: _ }, Self::Return { l: _ }) => true,
            (Self::Unsigned { l: _ }, Self::Unsigned { l: _ }) => true,
            (Self::IntKey { l: _ }, Self::IntKey { l: _ }) => true,
            (Self::StrKey { l: _ }, Self::StrKey { l: _ }) => true,
            (Self::BoolKey { l: _ }, Self::BoolKey { l: _ }) => true,
            _ => false,
        }
    }
}
