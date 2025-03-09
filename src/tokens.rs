use std::rc::Rc;

#[derive(Debug, Clone)]
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
}

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
    // TODO
    // Composed symbols
    LogicalOr { l: Location },
    LogicalAnd { l: Location },
    EqualOperator { l: Location },
    NotEqualOperator { l: Location },
    Leq { l: Location },
    Geq { l: Location },
    ArrayType { l: Location },
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
            Token::ArrayType { l: _ } => "[]".to_string(),
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
