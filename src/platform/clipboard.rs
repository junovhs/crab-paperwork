use anyhow::{Context, Result};
use arboard::Clipboard;

pub fn copy_text(text: &str) -> Result<()> {
    let mut clipboard = Clipboard::new().context("failed to open system clipboard")?;

    clipboard
        .set_text(text.to_string())
        .context("failed to copy text to clipboard")?;

    Ok(())
}
