use std::{
    collections::VecDeque,
    io::BufRead,
    iter::{Enumerate, Peekable, SkipWhile},
    str::Chars,
};

use crate::{
    Error,
    tokens::{Location, Token},
};

pub struct Lexer<R: BufRead> {
    source: R,
    buf: String,
    location: Location,
}

impl<R: BufRead> Lexer<R> {
    pub fn scan(&mut self) -> Result<VecDeque<Token>, Error> {
        let mut token_stream = VecDeque::new();
        let mut line_buffer = String::new();

        // EOF return
        if self.source.read_line(&mut line_buffer)? == 0 {
            return Ok(token_stream);
        }

        let iter = line_buffer
            .chars()
            .enumerate()
            .skip_while(|(_, c)| c.is_whitespace())
            .peekable();

        self.scan_line(iter, &mut token_stream)?;

        Ok(token_stream)
    }

    fn scan_line<P: FnMut(&(usize, char)) -> bool>(
        &mut self,
        mut line: Peekable<SkipWhile<Enumerate<Chars>, P>>,
        token_stream: &mut VecDeque<Token>,
    ) -> Result<(), Error> {
        self.location.next_line();

        while let Some((i, c)) = line.peek() {
            self.location.update_symbol(*i);

            match *c {
                _ => {
                    self.location.next_symbol();
                    line.next();
                    token_stream.push_back(Token::Unknown {
                        l: self.location.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub fn next(&mut self) -> Result<Token, ()> {
        if self.buf.is_empty() {
            self.location.next_line();
            self.source.read_line(&mut self.buf).unwrap();
            self.buf = self.buf.chars().rev().collect();
        }

        let mut current;

        let res = loop {
            current = self.buf.pop().unwrap();
            if current.is_whitespace() {
                continue;
            }

            break match current {
                ',' => Token::Comma {
                    l: self.location.clone(),
                },
                ':' => Token::Colon {
                    l: self.location.clone(),
                },
                ';' => Token::SemiColon {
                    l: self.location.clone(),
                },
                '.' => Token::Dot {
                    l: self.location.clone(),
                },
                '!' => Token::Exclamation {
                    l: self.location.clone(),
                },
                '+' => Token::Plus {
                    l: self.location.clone(),
                },
                '-' => Token::Dash {
                    l: self.location.clone(),
                },
                '*' => Token::Asterix {
                    l: self.location.clone(),
                },
                '/' => Token::Slash {
                    l: self.location.clone(),
                },
                '%' => Token::Percent {
                    l: self.location.clone(),
                },
                '=' => Token::Equal {
                    l: self.location.clone(),
                },
                '&' => Token::Ampersand {
                    l: self.location.clone(),
                },
                '^' => Token::Head {
                    l: self.location.clone(),
                },
                '|' => self.lex_composed(Token::Pipe {
                    l: self.location.clone(),
                }),
                '<' => self.lex_composed(Token::LAngle {
                    l: self.location.clone(),
                }),
                '>' => self.lex_composed(Token::RAngle {
                    l: self.location.clone(),
                }),
                '{' => Token::LBrace {
                    l: self.location.clone(),
                },
                '}' => Token::RBrace {
                    l: self.location.clone(),
                },
                '(' => Token::LPar {
                    l: self.location.clone(),
                },
                ')' => Token::RPar {
                    l: self.location.clone(),
                },
                '[' => self.lex_composed(Token::LBracket {
                    l: self.location.clone(),
                }),
                ']' => Token::RBracket {
                    l: self.location.clone(),
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

    fn lex_composed(&mut self, _current: Token) -> Token {
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
            l: self.location.clone(),
        }
    }
}
