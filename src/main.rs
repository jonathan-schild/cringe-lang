/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::io;

use log::info;
use thiserror::Error;

use build_info::build_info;

mod build_info;
mod lexer;
mod tokens;

pub fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("{}", build_info());
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Unexpected LF")]
    UnexpectedLF,
}
