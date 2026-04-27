use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct PreviewProps {
    pub rendered_html: String,
    pub on_mounted: EventHandler<MountedEvent>,
}

#[allow(non_snake_case)]
pub fn Preview(props: PreviewProps) -> Element {
    let on_mounted = props.on_mounted;

    rsx! {
        article {
            class: "markdown-body preview-content",
            onmounted: move |event| on_mounted.call(event),
            dangerous_inner_html: "{props.rendered_html}"
        }
    }
}
