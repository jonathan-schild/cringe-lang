/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use build_info::build_info;
use log::info;

mod build_info;
mod lexer;
mod tokens;

pub fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("{}", build_info());
}
