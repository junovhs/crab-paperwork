use crab_paperwork::markdown::render::render_markdown;

#[test]
fn renders_tables_and_task_lists() {
    let markdown = "- [x] Done\n\n| A | B |\n|---|---|\n| 1 | 2 |";
    let html = render_markdown(markdown);

    assert!(html.contains(r#"type="checkbox""#));
    assert!(html.contains("checked"));
    assert!(html.contains("disabled"));
    assert!(html.contains("<table>"));
    assert!(html.contains("<td>1</td>"));
}

#[test]
fn sanitizes_script_and_event_attributes() {
    let markdown = r#"<img src="x" onerror="alert(1)"><script>alert(2)</script>"#;
    let html = render_markdown(markdown);

    assert!(html.contains(r#"<img src="x">"#));
    assert!(!html.contains("onerror"));
    assert!(!html.contains("<script"));
    assert!(!html.contains("alert"));
}
