use dioxus::prelude::*;
use dioxus_elements::geometry::PixelsVector2D;
use std::rc::Rc;

use crate::ui::editor::Editor;
use crate::ui::preview::Preview;

#[derive(Clone, PartialEq, Props)]
pub struct SplitProps {
    pub markdown: String,
    pub rendered_html: String,
    pub sync_scroll: bool,
    pub on_markdown_change: EventHandler<String>,
}

#[allow(non_snake_case)]
pub fn Split(props: SplitProps) -> Element {
    let on_markdown_change = props.on_markdown_change;
    let sync_scroll_class = if props.sync_scroll {
        " sync-scroll"
    } else {
        ""
    };
    let mut preview_element = use_signal(|| None::<Rc<MountedData>>);
    let sync_scroll = props.sync_scroll;

    rsx! {
        main { class: "split-shell{sync_scroll_class}",
            section { class: "pane editor-pane",
                Editor {
                    markdown: props.markdown,
                    on_change: move |value| {
                        on_markdown_change.call(value);
                    },
                    on_scroll: move |event: ScrollEvent| {
                        if !sync_scroll {
                            return;
                        }

                        let data = event.data();
                        let max_scroll = (data.scroll_height() - data.client_height()).max(1) as f64;
                        let ratio = (data.scroll_top() / max_scroll).clamp(0.0, 1.0);

                        if let Some(element) = preview_element() {
                            spawn(async move {
                                let Ok(scroll_size) = element.get_scroll_size().await else {
                                    return;
                                };
                                let Ok(client_rect) = element.get_client_rect().await else {
                                    return;
                                };

                                let max_preview_scroll =
                                    (scroll_size.height - client_rect.size.height).max(0.0);
                                let target = max_preview_scroll * ratio;

                                let _ = element
                                    .scroll(
                                        PixelsVector2D::new(0.0, target),
                                        ScrollBehavior::Instant,
                                    )
                                    .await;
                            });
                        }
                    }
                }
            }

            div { class: "split-divider" }

            section { class: "pane preview-pane",
                Preview {
                    rendered_html: props.rendered_html,
                    on_mounted: move |event: MountedEvent| {
                        preview_element.set(Some(event.data()));
                    }
                }
            }
        }
    }
}
