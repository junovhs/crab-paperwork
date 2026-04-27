use std::path::PathBuf;

use rfd::FileDialog;

pub fn pick_markdown_open_path() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Open Markdown")
        .add_filter("Markdown", &["md", "markdown", "mdown", "mkd"])
        .add_filter("Text", &["txt"])
        .pick_file()
}

pub fn pick_markdown_save_path() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Save Markdown")
        .add_filter("Markdown", &["md", "markdown"])
        .set_file_name("paperwork.md")
        .save_file()
}

pub fn pick_export_html_save_path() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Export Preview")
        .add_filter("HTML", &["html", "htm"])
        .set_file_name("markdown-preview.html")
        .save_file()
}
