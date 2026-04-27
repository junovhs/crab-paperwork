use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn read_markdown(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();

    fs::read_to_string(path)
        .with_context(|| format!("failed to read Markdown file: {}", path.display()))
}

pub fn write_markdown(path: impl AsRef<Path>, markdown: &str) -> Result<()> {
    let path = path.as_ref();

    fs::write(path, markdown)
        .with_context(|| format!("failed to write Markdown file: {}", path.display()))
}
