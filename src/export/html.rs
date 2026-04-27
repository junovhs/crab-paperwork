use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn save_html(
    path: impl AsRef<Path>,
    title: &str,
    body_html: &str,
    light_css: &str,
) -> Result<()> {
    let path = path.as_ref();
    let document = format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{}</title>
<style>
body {{ margin: 32px; background: #fff; }}
{}
</style>
</head>
<body class="theme-light">
<main class="markdown-body">
{}
</main>
</body>
</html>
"#,
        escape_html(title),
        light_css,
        body_html
    );

    fs::write(path, document)
        .with_context(|| format!("failed to export preview HTML: {}", path.display()))
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
