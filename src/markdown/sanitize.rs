use ammonia::Builder;

pub fn sanitize_html(html: &str) -> String {
    Builder::default()
        .add_tags([
            "article",
            "section",
            "header",
            "footer",
            "main",
            "figure",
            "figcaption",
            "details",
            "summary",
            "input",
            "mark",
            "kbd",
            "samp",
        ])
        .add_generic_attributes(["class", "id", "title", "aria-label", "aria-hidden", "role"])
        .add_tag_attributes(
            "input",
            [
                "type",
                "checked",
                "disabled",
                "readonly",
                "aria-label",
                "class",
            ],
        )
        .add_tag_attributes("a", ["href", "title", "target"])
        .add_tag_attributes("img", ["src", "alt", "title", "width", "height"])
        .clean(html)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_script_tags() {
        let clean = sanitize_html("<h1>Hello</h1><script>alert('bad')</script>");

        assert!(clean.contains("<h1>Hello</h1>"));
        assert!(!clean.contains("<script"));
        assert!(!clean.contains("alert"));
    }

    #[test]
    fn preserves_markdown_classes() {
        let clean = sanitize_html(r#"<code class="language-rust">fn main(){}</code>"#);

        assert!(clean.contains("language-rust"));
    }
}
