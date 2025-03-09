use std::rc::Rc;

pub enum Token {
    Unknown {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    // Single punctuation symbol
    Comma {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Colon {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    SemiColon {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Dot {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Exclamation {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    LableDecl {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    // Single arithmetic symbol
    Plus {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Dash {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Asterix {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Slash {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Percent {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Equal {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    // Single logical symbol
    Ampersand {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Head {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Pipe {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    // Single grouping symbol
    LAngle {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    RAngle {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    LBrace {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    RBrace {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    LPar {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    RPar {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    LBracket {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    RBracket {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    // Literal
    Int {
        int: u64,
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Str {
        str: String,
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Bool {
        bool: bool,
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Identifier {
        id: String,
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    // TODO
    // Composed symbols
    LogicalOr {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    LogicalAnd {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    EqualOperator {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    NotEqualOperator {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Leq {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Geq {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    ArrayType {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    // Keywords
    Sizeof {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Namespace {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Struct {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Fn {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    SelfKey {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    If {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Else {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Loop {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    For {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    In {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Break {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Continue {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Var {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Val {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Return {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    Unsigned {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    IntKey {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    StrKey {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
    BoolKey {
        l: usize,
        c: usize,
        f: Rc<String>,
    },
}

impl Token {
    pub fn to_debug_string(&self) -> String {
        match self {
            Token::Unknown { l: _, c: _, f: _ } => "UNKNOWN".to_string(),
            Token::Comma { l: _, c: _, f: _ } => ",".to_string(),
            Token::Colon { l: _, c: _, f: _ } => ":".to_string(),
            Token::SemiColon { l: _, c: _, f: _ } => ";".to_string(),
            Token::Dot { l: _, c: _, f: _ } => ".".to_string(),
            Token::Exclamation { l: _, c: _, f: _ } => "!".to_string(),
            Token::LableDecl { l: _, c: _, f: _ } => "'".to_string(),
            Token::Plus { l: _, c: _, f: _ } => "+".to_string(),
            Token::Dash { l: _, c: _, f: _ } => "-".to_string(),
            Token::Asterix { l: _, c: _, f: _ } => "*".to_string(),
            Token::Slash { l: _, c: _, f: _ } => "/".to_string(),
            Token::Percent { l: _, c: _, f: _ } => "%".to_string(),
            Token::Equal { l: _, c: _, f: _ } => "=".to_string(),
            Token::Ampersand { l: _, c: _, f: _ } => "&".to_string(),
            Token::Head { l: _, c: _, f: _ } => "^".to_string(),
            Token::Pipe { l: _, c: _, f: _ } => "|".to_string(),
            Token::LAngle { l: _, c: _, f: _ } => "<".to_string(),
            Token::RAngle { l: _, c: _, f: _ } => ">".to_string(),
            Token::LBrace { l: _, c: _, f: _ } => "{".to_string(),
            Token::RBrace { l: _, c: _, f: _ } => "}".to_string(),
            Token::LPar { l: _, c: _, f: _ } => "(".to_string(),
            Token::RPar { l: _, c: _, f: _ } => ")".to_string(),
            Token::LBracket { l: _, c: _, f: _ } => "[".to_string(),
            Token::RBracket { l: _, c: _, f: _ } => "]".to_string(),
            Token::Int {
                int,
                l: _,
                c: _,
                f: _,
            } => format!("int/{int}"),
            Token::Str {
                str,
                l: _,
                c: _,
                f: _,
            } => format!("str/\"{str}\""),
            Token::Bool {
                bool,
                l: _,
                c: _,
                f: _,
            } => {
                if *bool {
                    "bool/true".to_string()
                } else {
                    "bool/false".to_string()
                }
            }
            Token::Identifier {
                id,
                l: _,
                c: _,
                f: _,
            } => format!("id/{id}"),
            Token::LogicalOr { l: _, c: _, f: _ } => "||".to_string(),
            Token::LogicalAnd { l: _, c: _, f: _ } => "&&".to_string(),
            Token::EqualOperator { l: _, c: _, f: _ } => "==".to_string(),
            Token::NotEqualOperator { l: _, c: _, f: _ } => "!=".to_string(),
            Token::Leq { l: _, c: _, f: _ } => "<=".to_string(),
            Token::Geq { l: _, c: _, f: _ } => ">=".to_string(),
            Token::ArrayType { l: _, c: _, f: _ } => "[]".to_string(),
            Token::Sizeof { l: _, c: _, f: _ } => "sizeof".to_string(),
            Token::Namespace { l: _, c: _, f: _ } => "namespace".to_string(),
            Token::Struct { l: _, c: _, f: _ } => "struct".to_string(),
            Token::Fn { l: _, c: _, f: _ } => "fn".to_string(),
            Token::SelfKey { l: _, c: _, f: _ } => "self".to_string(),
            Token::If { l: _, c: _, f: _ } => "if".to_string(),
            Token::Else { l: _, c: _, f: _ } => "else".to_string(),
            Token::Loop { l: _, c: _, f: _ } => "loop".to_string(),
            Token::For { l: _, c: _, f: _ } => "for".to_string(),
            Token::In { l: _, c: _, f: _ } => "in".to_string(),
            Token::Break { l: _, c: _, f: _ } => "break".to_string(),
            Token::Continue { l: _, c: _, f: _ } => "continue".to_string(),
            Token::Var { l: _, c: _, f: _ } => "var".to_string(),
            Token::Val { l: _, c: _, f: _ } => "val".to_string(),
            Token::Return { l: _, c: _, f: _ } => "return".to_string(),
            Token::Unsigned { l: _, c: _, f: _ } => "unsigned".to_string(),
            Token::IntKey { l: _, c: _, f: _ } => "int".to_string(),
            Token::StrKey { l: _, c: _, f: _ } => "str".to_string(),
            Token::BoolKey { l: _, c: _, f: _ } => "bool".to_string(),
        }
    }
}
