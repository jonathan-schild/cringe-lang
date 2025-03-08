/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn build_info() -> String {
    let name = format!(
        "{}: Copyright (c) 2025 Jonathan \"Nath\" Schild. Licensed under the EUPL-1.2",
        env!("CARGO_PKG_NAME")
    );

    let version = if !env!("BUILD_HASH").is_empty() {
        format!(
            "Version: {}+{}.{} at {}",
            env!("CARGO_PKG_VERSION"),
            env!("BUILD_HASH"),
            env!("BUILD_EPOCH"),
            env!("BUILD_DATE")
        )
    } else {
        format!(
            "Version: {}.{} at {}",
            env!("CARGO_PKG_VERSION"),
            env!("BUILD_EPOCH"),
            env!("BUILD_DATE")
        )
    };

    let rustc = format!("Rust Version: {}", env!("BUILD_RUSTC"));

    let repo = format!("Source Code: {}", env!("CARGO_PKG_REPOSITORY"));

    let max = *[name.len(), version.len(), rustc.len(), repo.len()]
        .iter()
        .max()
        .unwrap(); // returns None if iter is empty, which should never occur

    let w = max + 2;
    format!(
        "\n#{}#\n#{}#\n#  {name:w$}#\n#  {version:w$}#\n#  {rustc:w$}#\n#  {repo:w$}#\n#{1}#\n#{0}#",
        "=".repeat(w + 2),
        " ".repeat(w + 2)
    )
}
