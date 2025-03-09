/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use crate::{Error, parser::tokens::Token};

use super::lexer_state::LexerState;

pub fn scan_line(state: &mut LexerState) -> Result<(), Error> {
    while let Some(c) = state.peek() {
        let c = *c; // TODO make it clean

        // Skip whitespace
        if c.is_whitespace() {
            scan_keyword_or_identifier(state)?;
            skip_whitespace(state);
        } else
        // Scan string literal
        if c == '"' {
            scan_keyword_or_identifier(state)?;
            scan_string_literal(state)?;
        } else
        // Scan punctuation
        if state.is_string_buffer_empty() && c.is_ascii_punctuation() {
            scan_punctuation(state)?;
        } else
        // Scan int literal
        if state.is_string_buffer_empty() && c.is_ascii_digit() {
            scan_int_literal(state)?;
        } else {
            state.buffer();
        }
    }

    scan_keyword_or_identifier(state)?;

    Ok(())
}

fn skip_whitespace(state: &mut LexerState) {
    while let Some(c) = state.peek() {
        if !c.is_whitespace() {
            break;
        }
        state.skip();
    }
}

fn scan_string_literal(state: &mut LexerState) -> Result<(), Error> {
    debug_assert_eq!(state.peek(), Some('"').as_ref());
    state.skip();

    let mut escape = false;
    while let Some(c) = state.peek() {
        if escape {
            escape = !escape;
            state.buffer();
        } else if *c == '\\' {
            escape = !escape;
            state.skip();
        } else if *c == '"' {
            state.skip();
            let buffer = state.string_buffer().to_string();
            state.accept(Token::Str(buffer));
            return Ok(());
        } else if *c == '\n' {
            return Err(Error::UnexpectedLF);
        } else {
            state.buffer();
        }
    }

    Err(Error::UnexpectedEOF)
}

fn scan_punctuation(state: &mut LexerState) -> Result<(), Error> {
    let Some(c) = state.peek() else { todo!() };
    let c = *c; // TODO clean that

    match c {
        ',' => {
            state.skip();
            state.accept(Token::Comma);
        }
        ':' => {
            state.skip();
            state.accept(Token::Colon);
        }
        ';' => {
            state.skip();
            state.accept(Token::SemiColon);
        }
        '.' => {
            state.skip();
            state.accept(Token::Dot);
        }

        '+' => {
            state.skip();
            state.accept(Token::Plus);
        }
        '-' => {
            state.skip();
            state.accept(Token::Dash);
        }
        '*' => {
            state.skip();
            state.accept(Token::Asterix);
        }
        '/' => {
            state.skip();
            state.accept(Token::Slash);
        }
        '%' => {
            state.skip();
            state.accept(Token::Percent);
        }
        '^' => {
            state.skip();
            state.accept(Token::Hat);
        }
        '{' => {
            state.skip();
            state.accept(Token::LBrace);
        }
        '}' => {
            state.skip();
            state.accept(Token::RBrace);
        }
        '(' => {
            state.skip();
            state.accept(Token::LPar);
        }
        ')' => {
            state.skip();
            state.accept(Token::RPar);
        }
        '[' => {
            state.skip();
            state.accept(Token::LBracket);
        }
        ']' => {
            state.skip();
            state.accept(Token::RBracket);
        }
        _ => scan_composed_punctuation(state)?,
    };

    Ok(())
}

fn scan_composed_punctuation(state: &mut LexerState) -> Result<(), Error> {
    match state.peek().unwrap() {
        '!' => {
            state.skip();
            let mut token = Token::Exclamation;
            if let Some(c) = state.peek() {
                if *c == '=' {
                    state.skip();
                    token = Token::NotEqualOperator;
                }
            }
            state.accept(token);
        }
        '=' => {
            state.skip();
            let mut token = Token::Equal;
            if let Some(c) = state.peek() {
                if *c == '=' {
                    state.skip();
                    token = Token::EqualOperator;
                }
            }
            state.accept(token);
        }
        '&' => {
            state.skip();
            let mut token = Token::Ampersand;
            if let Some(c) = state.peek() {
                if *c == '&' {
                    state.skip();
                    token = Token::LogicalAnd;
                }
            }
            state.accept(token);
        }
        '|' => {
            state.skip();
            let mut token = Token::Pipe;
            if let Some(c) = state.peek() {
                if *c == '|' {
                    state.skip();
                    token = Token::LogicalOr;
                }
            }
            state.accept(token);
        }
        '<' => {
            state.skip();
            let mut token = Token::LAngle;
            if let Some(c) = state.peek() {
                if *c == '=' {
                    state.skip();
                    token = Token::Leq;
                } else if *c == '<' {
                    state.skip();
                    token = Token::ShiftLeft;
                }
            }
            state.accept(token);
        }
        '>' => {
            state.skip();
            let mut token = Token::RAngle;
            if let Some(c) = state.peek() {
                if *c == '=' {
                    state.skip();
                    token = Token::Geq;
                } else if *c == '>' {
                    state.skip();
                    token = Token::ShiftRight;
                }
            }
            state.accept(token);
        }
        c => return Err(Error::UnexpectedSymbol(*c)),
    };
    Ok(())
}

fn scan_int_literal(state: &mut LexerState) -> Result<(), Error> {
    let mut radix = None;
    if let Some(c) = state.peek() {
        if *c == '0' {
            state.skip();
        } else {
            radix = Some(10);
            state.buffer();
        }
    }

    while let Some(c) = state.peek() {
        if let Some(radix) = radix {
            if *c == '_' {
                state.skip();
            } else if c.is_digit(radix) {
                state.buffer();
            } else {
                break;
            }
        } else if *c == 'x' || *c == 'X' {
            radix = Some(16);
            state.skip();
        } else if *c == 'o' || *c == 'O' {
            radix = Some(8);
            state.skip();
        } else if *c == 'b' || *c == 'B' {
            radix = Some(2);
            state.skip();
        } else if c.is_ascii_digit() {
            radix = Some(10);
            state.buffer();
        } else {
            return Err(Error::UnexpectedSymbol(*c));
        }
    }

    let buffer = state.string_buffer();
    let int = u64::from_str_radix(buffer, radix.unwrap())?;

    state.accept(Token::Int(int));

    Ok(())
}

fn scan_keyword_or_identifier(state: &mut LexerState) -> Result<(), Error> {
    let token = match state.string_buffer() {
        "namespace" => Token::Namespace,
        "struct" => Token::Struct,
        "fn" => Token::Fn,
        "if" => Token::If,
        "else" => Token::Else,
        "loop" => Token::Loop,
        "for" => Token::For,
        "in" => Token::In,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "var" => Token::Var,
        "val" => Token::Val,
        "sizeof" => Token::Sizeof,
        "return" => Token::Return,
        "unsigned" => Token::Unsigned,
        "int" => Token::IntKey,
        "str" => Token::StrKey,
        "bool" => Token::BoolKey,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        _ => scan_identifier(state)?,
    };

    state.accept(token);

    Ok(())
}

fn scan_identifier(state: &mut LexerState) -> Result<Token, Error> {
    for c in state.string_buffer().chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
        } else {
            return Err(Error::InvalidID(state.string_buffer().to_string()));
        }
    }
    Ok(Token::Identifier(state.string_buffer().to_string()))
}
