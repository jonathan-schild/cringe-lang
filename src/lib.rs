/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::{io, num::ParseIntError};

use thiserror::Error;

pub mod build_info;
pub mod parser;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("Unexpected symbol {0}")]
    UnexpectedSymbol(char),
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Unexpected LF")]
    UnexpectedLF,
    #[error("Invalid identifier: {0}")]
    InvalidID(String),
}
