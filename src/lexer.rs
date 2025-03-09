use std::{
    collections::VecDeque,
    io::BufRead,
    iter::{Enumerate, Peekable, SkipWhile},
    str::Chars,
};

use log::error;

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

        let iter = line_buffer.chars().peekable();

        self.scan_line(iter, &mut token_stream)?;

        Ok(token_stream)
    }

    fn scan_line(
        &mut self,
        mut line: Peekable<Chars>,
        token_stream: &mut VecDeque<Token>,
    ) -> Result<(), Error> {
        self.location.next_line();

        while let Some(c) = line.peek() {
            let c = *c; // TODO make it clean

            // Skip whitespace
            if c.is_whitespace() {
                self.skip_whitespace(&mut line);
            } else
            // Scan string literal
            if c == '"' {
                self.scan_string_literal(&mut line, token_stream)?;
            } else
            // Scan int literal
            if c.is_ascii_digit() {
                self.scan_int_literal(&mut line, token_stream);
            } else
            // Scan punctuation
            if c.is_ascii_punctuation() {
                self.scan_punctuation(&mut line, token_stream);
            } else {
                match c {
                    _ => {
                        self.location.next_symbol();
                        line.next();
                        token_stream.push_back(Token::Unknown {
                            l: self.location.clone(),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    fn skip_whitespace(&mut self, line: &mut Peekable<Chars>) {
        while let Some(c) = line.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.location.next_symbol();
            line.next();
        }
    }

    fn scan_punctuation(
        &mut self,
        line: &mut Peekable<Chars>,
        _token_stream: &mut VecDeque<Token>,
    ) {
        let _ = match line.peek().unwrap() {
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
            '|' => Token::Pipe {
                l: self.location.clone(),
            },
            '<' => Token::LAngle {
                l: self.location.clone(),
            },
            '>' => Token::RAngle {
                l: self.location.clone(),
            },
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
            '[' => Token::LBracket {
                l: self.location.clone(),
            },
            ']' => Token::RBracket {
                l: self.location.clone(),
            },
            _ => panic!(),
        };

        todo!()
    }

    fn scan_int_literal(
        &mut self,
        _line: &mut Peekable<Chars>,
        _token_stream: &mut VecDeque<Token>,
    ) {
        todo!()
    }

    fn scan_string_literal(
        &mut self,
        line: &mut Peekable<Chars>,
        token_stream: &mut VecDeque<Token>,
    ) -> Result<(), Error> {
        let mut str = String::new();
        self.location.next_symbol();
        line.next();

        let mut escape = false;
        while let Some(c) = line.peek() {
            if escape {
                escape = !escape;
                str.push(*c);
                self.location.next_symbol();
                line.next();
            } else {
                if *c == '\\' {
                    escape = !escape;
                    self.location.next_symbol();
                    line.next();
                } else if *c == '"' {
                    self.location.next_symbol();
                    line.next();
                    token_stream.push_back(Token::Str {
                        str,
                        l: self.location.clone(),
                    });
                    return Ok(());
                } else if *c == '\n' {
                    error!(
                        "Multiline Strings not supported @ {} {}; {}",
                        self.location.f, self.location.l, self.location.c
                    );
                    return Err(Error::UnexpectedLF);
                }
            }
        }

        return Err(Error::UnexpectedEOF);
    }
}
