use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ToolbarProps {
    pub title: String,
    pub sync_scroll: bool,
    pub dark_mode: bool,
    pub on_reset: EventHandler<()>,
    pub on_copy: EventHandler<()>,
    pub on_export_pdf: EventHandler<()>,
    pub on_toggle_sync_scroll: EventHandler<()>,
    pub on_toggle_theme: EventHandler<()>,
}

#[allow(non_snake_case)]
pub fn Toolbar(props: ToolbarProps) -> Element {
    let on_reset = props.on_reset;
    let on_copy = props.on_copy;
    let on_export_pdf = props.on_export_pdf;
    let on_toggle_sync_scroll = props.on_toggle_sync_scroll;
    let on_toggle_theme = props.on_toggle_theme;

    rsx! {
        header { class: "topbar",
            nav { class: "menu",
                span { class: "brand", "{props.title}" }

                button {
                    class: "menu-button",
                    onclick: move |_| on_reset.call(()),
                    "Reset"
                }

                button {
                    class: "menu-button",
                    onclick: move |_| on_copy.call(()),
                    "Copy"
                }

                button {
                    class: "menu-button",
                    onclick: move |_| on_export_pdf.call(()),
                    "Export PDF"
                }

                label { class: "menu-check",
                    input {
                        r#type: "checkbox",
                        checked: props.sync_scroll,
                        onchange: move |_| on_toggle_sync_scroll.call(())
                    }
                    span { "Sync scroll" }
                }

                label { class: "menu-check",
                    input {
                        r#type: "checkbox",
                        checked: props.dark_mode,
                        onchange: move |_| on_toggle_theme.call(())
                    }
                    span { "Dark mode" }
                }
            }
        }
    }
}
