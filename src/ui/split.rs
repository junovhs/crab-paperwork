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
                        let ratio = scroll_ratio(
                            data.scroll_top(),
                            data.scroll_height() as f64,
                            data.client_height() as f64,
                        );

                        if let Some(element) = preview_element() {
                            spawn(async move {
                                let Ok(scroll_size) = element.get_scroll_size().await else {
                                    return;
                                };
                                let Ok(client_rect) = element.get_client_rect().await else {
                                    return;
                                };

                                let target = scroll_target(
                                    ratio,
                                    scroll_size.height,
                                    client_rect.size.height,
                                );

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

            section {
                class: "pane preview-pane",
                onmounted: move |event: MountedEvent| {
                    preview_element.set(Some(event.data()));
                },
                Preview {
                    rendered_html: props.rendered_html
                }
            }
        }
    }
}

fn scroll_ratio(scroll_top: f64, scroll_height: f64, client_height: f64) -> f64 {
    let max_scroll = (scroll_height - client_height).max(1.0);

    (scroll_top / max_scroll).clamp(0.0, 1.0)
}

fn scroll_target(ratio: f64, scroll_height: f64, client_height: f64) -> f64 {
    let max_scroll = (scroll_height - client_height).max(0.0);

    max_scroll * ratio.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_ratio_clamps_to_scrollable_range() {
        assert_eq!(scroll_ratio(50.0, 200.0, 100.0), 0.5);
        assert_eq!(scroll_ratio(-20.0, 200.0, 100.0), 0.0);
        assert_eq!(scroll_ratio(220.0, 200.0, 100.0), 1.0);
    }

    #[test]
    fn scroll_target_maps_ratio_to_preview_scroll_range() {
        assert_eq!(scroll_target(0.5, 600.0, 200.0), 200.0);
        assert_eq!(scroll_target(2.0, 600.0, 200.0), 400.0);
        assert_eq!(scroll_target(0.5, 200.0, 600.0), 0.0);
    }
}
