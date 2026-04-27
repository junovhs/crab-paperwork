use dioxus::prelude::*;

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

    rsx! {
        div { class: "editor-shell",
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

fn line_numbers_for(markdown: &str) -> String {
    let line_count = markdown.lines().count().max(1);

    (1..=line_count)
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
