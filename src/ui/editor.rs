use dioxus::prelude::*;

use crate::pretext::{layout, prepare, FontMetrics, PrepareOptions, WhiteSpace};

#[derive(Clone, PartialEq, Props)]
pub struct EditorProps {
    pub markdown: String,
    pub on_change: EventHandler<String>,
    pub on_scroll: EventHandler<ScrollEvent>,
}

#[allow(non_snake_case)]
pub fn Editor(props: EditorProps) -> Element {
    let on_change = props.on_change;
    let on_scroll = props.on_scroll;
    let line_numbers = line_numbers_for(&props.markdown);
    let layout = editor_layout(&props.markdown);

    rsx! {
        div {
            class: "editor-shell",
            "data-layout-lines": "{layout.line_count}",
            "data-layout-height": "{layout.height}",
            pre { class: "line-gutter", "{line_numbers}" }
            textarea {
                class: "markdown-editor",
                spellcheck: "false",
                value: "{props.markdown}",
                placeholder: "Type Markdown here...",
                oninput: move |event| {
                    on_change.call(event.value());
                },
                onscroll: move |event| {
                    on_scroll.call(event);
                }
            }
        }
    }
}

fn editor_layout(markdown: &str) -> crate::pretext::LayoutResult {
    let prepared = prepare(
        markdown,
        FontMetrics::monospace(16.0),
        PrepareOptions {
            white_space: WhiteSpace::PreWrap,
            ..PrepareOptions::default()
        },
    );

    layout(&prepared, 760.0, 23.2)
}

fn line_numbers_for(markdown: &str) -> String {
    let line_count = markdown.lines().count().max(1);

    (1..=line_count)
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
