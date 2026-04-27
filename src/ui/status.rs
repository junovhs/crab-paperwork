use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct StatusProps {
    pub notice: String,
}

#[allow(non_snake_case)]
pub fn Status(props: StatusProps) -> Element {
    rsx! {
        footer { class: "statusbar",
            "{props.notice}"
        }
    }
}
