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

    rsx! {
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
