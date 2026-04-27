use anyhow::{bail, Result};
use std::path::Path;

pub fn save_pdf(_path: impl AsRef<Path>, _title: &str, _body_html: &str, _css: &str) -> Result<()> {
    bail!(
        "PDF export is not implemented yet. It must be implemented Rust-side; no JavaScript exporter."
    )
}
