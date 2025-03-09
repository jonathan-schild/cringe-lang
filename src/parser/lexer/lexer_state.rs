/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{collections::VecDeque, iter::Peekable, str::Chars};

use crate::parser::tokens::{Location, Token};

pub struct LexerState<'a> {
    token_stream: VecDeque<Token>,
    current_location: Location,
    token_location: Location,
    buffer_line: Peekable<Chars<'a>>,
    buffer: String,
}

impl<'a> LexerState<'a> {
    pub fn new(location: &Location, line: Peekable<Chars<'a>>) -> Self {
        Self {
            token_stream: VecDeque::new(),
            current_location: location.clone(), // TODO Update Line
            token_location: location.clone(),   // TODO same here
            buffer_line: line,
            buffer: String::new(),
        }
    }

    pub fn peek<'b, 'c>(&'b mut self) -> Option<&'c char>
    where
        'a: 'b,
        'b: 'c,
    {
        self.buffer_line.peek()
    }

    pub fn buffer(&mut self) -> bool {
        if let Some(c) = self.buffer_line.next() {
            self.buffer.push(c);
            true
        } else {
            false
        }
    }

    pub fn take(&mut self) -> Option<char> {
        self.buffer_line.next()
    }

    pub fn skip(&mut self) -> bool {
        self.buffer_line.next().is_some()
    }

    pub fn clear_string_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn is_string_buffer_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn string_buffer(&self) -> &str {
        self.buffer.as_str()
    }

    pub fn accept(&mut self, token: Token) {
        self.buffer.clear();
        self.token_stream.push_back(token);
        self.token_location = self.current_location.clone();
    }
}

impl From<LexerState<'_>> for (VecDeque<Token>, Location) {
    fn from(value: LexerState<'_>) -> Self {
        (value.token_stream, value.current_location)
    }
}
