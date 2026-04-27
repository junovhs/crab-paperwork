use std::path::PathBuf;

use rfd::FileDialog;

pub fn pick_pdf_save_path() -> Option<PathBuf> {
    FileDialog::new()
        .set_title("Export PDF")
        .add_filter("PDF", &["pdf"])
        .set_file_name("preview.pdf")
        .save_file()
}
