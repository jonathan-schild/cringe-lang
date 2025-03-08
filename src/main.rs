/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use build_info::build_info;
use log::info;

mod build_info;

pub fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("{}", build_info());
}
