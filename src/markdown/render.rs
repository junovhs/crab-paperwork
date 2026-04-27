use pulldown_cmark::{html, Options, Parser};

use crate::markdown::sanitize;

pub fn render_markdown(markdown: &str) -> String {
    let unsafe_html = render_markdown_without_sanitizing(markdown);
    sanitize::sanitize_html(&unsafe_html)
}

fn render_markdown_without_sanitizing(markdown: &str) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_basic_markdown() {
        let html = render_markdown("# Hello");

        assert!(html.contains("<h1"));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn renders_tables() {
        let markdown = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown(markdown);

        assert!(html.contains("<table>"));
        assert!(html.contains("<td>1</td>"));
    }
}
