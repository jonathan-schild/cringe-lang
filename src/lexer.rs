use std::{
    io::{BufRead, Read},
    rc::Rc,
};

use crate::tokens::Token;

pub struct Lexer<R: BufRead> {
    source: R,
    buf: String,
    l: usize,
    c: usize,
    f: Rc<String>,
}

impl<R: BufRead> Lexer<R> {
    pub fn next(&mut self) -> Result<Token, ()> {
        if self.buf.is_empty() {
            self.l += 1;
            self.c = 0;
            self.source.read_line(&mut self.buf).unwrap();
            self.buf = self.buf.chars().into_iter().rev().collect();
        }

        let mut current;

        let res = loop {
            current = self.buf.pop().unwrap();
            if current.is_whitespace() {
                continue;
            }

            break match current {
                ',' => Token::Comma {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                ':' => Token::Colon {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                ';' => Token::SemiColon {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '.' => Token::Dot {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '!' => Token::Exclamation {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '+' => Token::Plus {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '-' => Token::Dash {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '*' => Token::Asterix {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '/' => Token::Slash {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '%' => Token::Percent {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '=' => Token::Equal {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '&' => Token::Ampersand {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '^' => Token::Head {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '|' => Token::Pipe {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '<' => Token::LAngle {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '>' => Token::RAngle {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '{' => Token::LBrace {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '}' => Token::RBrace {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '(' => Token::LPar {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                ')' => Token::RPar {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '[' => Token::LBracket {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                ']' => Token::RBracket {
                    l: self.l,
                    c: self.c,
                    f: self.f.clone(),
                },
                '0' => self.lex_int_literal(current),
                '"' => self.lex_string_literal(current),
                _ => self.lex_complex(current),
            };
        };

        Ok(res)
    }

    fn lex_complex(&mut self, _current: char) -> Token {
        todo!()
    }

    fn lex_string_literal(&mut self, _current: char) -> Token {
        todo!()
    }

    fn lex_int_literal(&mut self, mut current: char) -> Token {
        let mut literal = String::from(current);

        loop {
            current = self.buf.pop().unwrap();

            if current.is_whitespace() {
                break;
            }

            literal.push(current);
        }

        let int: u64 = literal.parse().unwrap();

        Token::Int {
            int,
            l: self.l,
            c: self.c,
            f: self.f.clone(),
        }
    }
}
