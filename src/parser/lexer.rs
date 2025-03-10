/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{collections::VecDeque, io::BufRead};

use lexer_gut::scan_line;
use lexer_state::LexerState;

use crate::{
    Error,
    parser::tokens::{Location, Token},
};

mod lexer_gut;
mod lexer_state;

pub struct Lexer<R: BufRead> {
    source: R,
    location: Location,
    current_location: Location,
    token_location: Location,
}

impl<R: BufRead> Lexer<R> {
    pub fn scan(&mut self) -> Result<VecDeque<Token>, Error> {
        let mut line_buffer = String::new();

        // EOF return
        if self.source.read_line(&mut line_buffer)? == 0 {
            return Ok(VecDeque::new());
        }

        let iter = line_buffer.chars().peekable();
        let mut lexer_state = LexerState::new(&self.location, iter);

        scan_line(&mut lexer_state)?;

        let (tokens, location) = lexer_state.into();
        self.location = location;
        Ok(tokens)
    }
}
