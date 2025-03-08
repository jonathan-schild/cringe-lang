use std::rc::Rc;

pub enum Token {
    Unknown {
        l: usize,
        c: usize,
        f: Rc<String>,
    },

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

    Int {
        int: u64,
        l: usize,
        c: usize,
        f: Rc<String>,
    },
}
