use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PreviewProps {
    pub rendered_html: String,
}

#[allow(non_snake_case)]
pub fn Preview(props: PreviewProps) -> Element {
    rsx! {
        article {
            class: "markdown-body preview-content",
            dangerous_inner_html: "{props.rendered_html}"
        }
    }
}
