use dioxus::prelude::*;

use crate::ui::editor::Editor;
use crate::ui::preview::Preview;

#[derive(Clone, PartialEq, Props)]
pub struct SplitProps {
    pub markdown: String,
    pub rendered_html: String,
    pub on_markdown_change: EventHandler<String>,
}

#[allow(non_snake_case)]
pub fn Split(props: SplitProps) -> Element {
    let on_markdown_change = props.on_markdown_change;

    rsx! {
        main { class: "split-shell",
            section { class: "pane editor-pane",
                Editor {
                    markdown: props.markdown,
                    on_change: move |value| {
                        on_markdown_change.call(value);
                    }
                }
            }

            div { class: "split-divider" }

            section { class: "pane preview-pane",
                Preview {
                    rendered_html: props.rendered_html
                }
            }
        }
    }
}
