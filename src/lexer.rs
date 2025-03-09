/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{
    collections::VecDeque,
    io::BufRead,
    iter::{Enumerate, Peekable, SkipWhile},
    str::Chars,
};

use log::{debug, error};

use crate::{
    Error,
    tokens::{Location, Token},
};

pub struct Lexer<R: BufRead> {
    source: R,
    current_location: Location,
    token_location: Location,
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
                self.scan_int_literal(&mut line, token_stream)?;
            } else
            // Scan punctuation
            if c.is_ascii_punctuation() {
                self.scan_punctuation(&mut line, token_stream)?;
            } else {
                match c {
                    _ => {
                        line.next();
                        token_stream.push_back(Token::Unknown {
                            l: self.token_location.clone(),
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

            line.next();
        }
    }

    fn scan_punctuation(
        &mut self,
        line: &mut Peekable<Chars>,
        token_stream: &mut VecDeque<Token>,
    ) -> Result<(), Error> {
        let Some(c) = line.peek() else { todo!() };
        let c = *c; // TODO clean that

        let token = match c {
            ',' => {
                line.next();
                Token::Comma {
                    l: self.token_location.clone(),
                }
            }
            ':' => {
                line.next();
                Token::Colon {
                    l: self.token_location.clone(),
                }
            }
            ';' => {
                line.next();
                Token::SemiColon {
                    l: self.token_location.clone(),
                }
            }
            '.' => {
                line.next();
                Token::Dot {
                    l: self.token_location.clone(),
                }
            }

            '+' => {
                line.next();
                Token::Plus {
                    l: self.token_location.clone(),
                }
            }
            '-' => {
                line.next();
                Token::Dash {
                    l: self.token_location.clone(),
                }
            }
            '*' => {
                line.next();
                Token::Asterix {
                    l: self.token_location.clone(),
                }
            }
            '/' => {
                line.next();
                Token::Slash {
                    l: self.token_location.clone(),
                }
            }
            '%' => {
                line.next();
                Token::Percent {
                    l: self.token_location.clone(),
                }
            }
            '^' => {
                line.next();
                Token::Head {
                    l: self.token_location.clone(),
                }
            }
            '{' => {
                line.next();
                Token::LBrace {
                    l: self.token_location.clone(),
                }
            }
            '}' => {
                line.next();
                Token::RBrace {
                    l: self.token_location.clone(),
                }
            }
            '(' => {
                line.next();
                Token::LPar {
                    l: self.token_location.clone(),
                }
            }
            ')' => {
                line.next();
                Token::RPar {
                    l: self.token_location.clone(),
                }
            }
            '[' => {
                line.next();
                Token::LBracket {
                    l: self.token_location.clone(),
                }
            }
            ']' => {
                line.next();
                Token::RBracket {
                    l: self.token_location.clone(),
                }
            }
            _ => self.scan_composed_punctuation(line)?,
        };

        token_stream.push_back(token);
        Ok(())
    }

    fn scan_composed_punctuation(&mut self, line: &mut Peekable<Chars>) -> Result<Token, Error> {
        match line.peek().unwrap() {
            '!' => {
                line.next();
                let mut token = Ok(Token::Exclamation {
                    l: self.token_location.clone(),
                });
                if let Some(c) = line.peek() {
                    if *c == '=' {
                        line.next();
                        token = Ok(Token::NotEqualOperator {
                            l: self.token_location.clone(),
                        });
                    }
                }
                token
            }
            '=' => {
                line.next();
                let mut token = Ok(Token::Equal {
                    l: self.token_location.clone(),
                });
                if let Some(c) = line.peek() {
                    if *c == '=' {
                        line.next();
                        token = Ok(Token::EqualOperator {
                            l: self.token_location.clone(),
                        });
                    }
                }
                token
            }
            '&' => {
                line.next();
                let mut token = Ok(Token::Ampersand {
                    l: self.token_location.clone(),
                });
                if let Some(c) = line.peek() {
                    if *c == '&' {
                        line.next();
                        token = Ok(Token::LogicalAnd {
                            l: self.token_location.clone(),
                        });
                    }
                }
                token
            }
            '|' => {
                line.next();
                let mut token = Ok(Token::Pipe {
                    l: self.token_location.clone(),
                });
                if let Some(c) = line.peek() {
                    if *c == '|' {
                        line.next();
                        token = Ok(Token::LogicalOr {
                            l: self.token_location.clone(),
                        });
                    }
                }
                token
            }
            '<' => {
                line.next();
                let mut token = Ok(Token::LAngle {
                    l: self.token_location.clone(),
                });
                if let Some(c) = line.peek() {
                    if *c == '=' {
                        line.next();
                        token = Ok(Token::Leq {
                            l: self.token_location.clone(),
                        });
                    } else if *c == '<' {
                        line.next();
                        token = Ok(Token::ShiftLeft {
                            l: self.token_location.clone(),
                        });
                    }
                }
                token
            }
            '>' => {
                line.next();
                let mut token = Ok(Token::RAngle {
                    l: self.token_location.clone(),
                });
                if let Some(c) = line.peek() {
                    if *c == '=' {
                        line.next();
                        token = Ok(Token::Geq {
                            l: self.token_location.clone(),
                        });
                    } else if *c == '>' {
                        line.next();
                        token = Ok(Token::ShiftRight {
                            l: self.token_location.clone(),
                        });
                    }
                }
                token
            }
            c => Err(Error::UnexpectedSymbol(*c, self.token_location.clone())),
        }
    }

    fn scan_int_literal(
        &mut self,
        line: &mut Peekable<Chars>,
        token_stream: &mut VecDeque<Token>,
    ) -> Result<(), Error> {
        let mut str = String::new();

        line.next();

        let mut radix = None;
        while let Some(c) = line.peek() {
            if radix.is_none() {
                if *c == 'x' || *c == 'X' {
                    radix = Some(16);
                } else if *c == 'o' || *c == 'O' {
                    radix = Some(8);
                } else if *c == 'b' || *c == 'B' {
                    radix = Some(2);
                } else if c.is_ascii_digit() {
                    radix = Some(10);
                    str.push(*c);
                } else {
                    return Err(Error::UnexpectedSymbol(*c, self.token_location.clone()));
                }

                line.next();
            } else if *c == '_' {
                line.next();
            } else if c.is_digit(radix.unwrap()) {
                str.push(*c);

                line.next();
            } else {
                token_stream.push_back(Token::Int {
                    int: str.parse()?,
                    l: self.token_location.clone(),
                });
                return Ok(());
            }
        }

        Err(Error::UnexpectedEOF(self.token_location.clone()))
    }

    fn scan_string_literal(
        &mut self,
        line: &mut Peekable<Chars>,
        token_stream: &mut VecDeque<Token>,
    ) -> Result<(), Error> {
        debug!("Start parsing string literal");

        let mut str = String::new();
        line.next();

        let mut escape = false;
        while let Some(c) = line.peek() {
            if escape {
                escape = !escape;
                str.push(*c);
                line.next();
            } else if *c == '\\' {
                escape = !escape;
                line.next();
            } else if *c == '"' {
                debug!("Finish parsing string literal str/{str}");
                line.next();
                token_stream.push_back(Token::Str {
                    str,
                    l: self.token_location.clone(),
                });
                return Ok(());
            } else if *c == '\n' {
                error!(
                    "Multiline Strings not supported @ {} {}; {}",
                    self.token_location.f, self.token_location.l, self.token_location.c
                );
                return Err(Error::UnexpectedLF(self.token_location.clone()));
            } else {
                str.push(*c);
                line.next();
            }
        }

        Err(Error::UnexpectedEOF(self.token_location.clone()))
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::VecDeque,
        io::{BufReader, Cursor},
    };

    use crate::{Error, tokens::Location};

    use super::Lexer;

    fn test_lexer() -> Lexer<BufReader<std::io::Cursor<&'static str>>> {
        Lexer {
            source: BufReader::new(Cursor::new("")),
            current_location: Location::default(),
            token_location: Location::default(),
        }
    }

    #[test]
    fn scan_string_literal() -> Result<(), Error> {
        // env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

        let mut input_string_0 = r#""A simple literal with nothing special!""#.chars().peekable();
        let mut input_string_1 = r#""A string literal with some escaped stuff \n \\ \" !""#
            .chars()
            .peekable();
        let mut invalid_input_string_2 = r#""A string without an ending!"#.chars().peekable();
        let mut invalid_input_string_3 = r#"Another invalid string with
        a line break
        "#
        .chars()
        .peekable();

        let mut lexer = test_lexer();

        let mut token_stream = VecDeque::new();

        lexer.scan_string_literal(&mut input_string_0, &mut token_stream)?;
        lexer.scan_string_literal(&mut input_string_1, &mut token_stream)?;
        let Err(Error::UnexpectedEOF(_)) =
            lexer.scan_string_literal(&mut invalid_input_string_2, &mut token_stream)
        else {
            panic!("expected unexpected eof")
        };

        let Err(Error::UnexpectedLF(_)) =
            lexer.scan_string_literal(&mut invalid_input_string_3, &mut token_stream)
        else {
            panic!("expected unexpected lf")
        };

        Ok(())
    }
}
