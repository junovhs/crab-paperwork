use dioxus::prelude::*;

use crate::app::actions::{reduce, AppAction};
use crate::app::state::{AppState, ThemeMode};
use crate::storage::config::AppConfig;
use crate::storage::session::SessionState;
use crate::ui::split::Split;
use crate::ui::status::Status;
use crate::ui::toolbar::Toolbar;

pub mod actions;
pub mod defaults;
pub mod state;

static MAIN_CSS: Asset = asset!("/assets/main.css");
static MARKDOWN_LIGHT_CSS: Asset = asset!("/assets/github_markdown_light.css");
static MARKDOWN_DARK_CSS: Asset = asset!("/assets/github_markdown_dark.css");

#[allow(non_snake_case)]
pub fn App() -> Element {
    let mut app_state = use_signal(load_initial_state);
    let current = app_state();

    let theme_class = current.theme.css_class();
    let dark_mode = matches!(current.theme, ThemeMode::Dark);

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: MARKDOWN_LIGHT_CSS }
        document::Stylesheet { href: MARKDOWN_DARK_CSS }

        div { class: "app-shell {theme_class}",
            Toolbar {
                title: "Crab Paperwork".to_string(),
                sync_scroll: current.sync_scroll,
                dark_mode,
                on_open: move |_| {
                    let mut next = app_state();

                    let message = match crate::platform::dialogs::pick_markdown_open_path() {
                        Some(path) => match crate::platform::files::read_markdown(&path) {
                            Ok(markdown) => {
                                next = reduce(next, AppAction::OpenMarkdown { path, markdown });
                                persist_state(&next);
                                app_state.set(next);
                                return;
                            }
                            Err(error) => format!("{error:#}"),
                        },
                        None => "Open canceled".to_string(),
                    };

                    next = reduce(next, AppAction::SetNotice(message));
                    persist_state(&next);
                    app_state.set(next);
                },
                on_save: move |_| {
                    let mut next = app_state();
                    let save_path = next
                        .current_file
                        .clone()
                        .or_else(crate::platform::dialogs::pick_markdown_save_path);

                    let message = match save_path {
                        Some(path) => match crate::platform::files::write_markdown(
                            &path,
                            &next.markdown,
                        ) {
                            Ok(()) => {
                                next = reduce(next, AppAction::SaveMarkdown { path });
                                persist_state(&next);
                                app_state.set(next);
                                return;
                            }
                            Err(error) => format!("{error:#}"),
                        },
                        None => "Save canceled".to_string(),
                    };

                    next = reduce(next, AppAction::SetNotice(message));
                    persist_state(&next);
                    app_state.set(next);
                },
                on_reset: move |_| {
                    let next = reduce(app_state(), AppAction::Reset);
                    persist_state(&next);
                    app_state.set(next);
                },
                on_copy: move |_| {
                    let mut next = app_state();

                    let message = match crate::platform::clipboard::copy_text(&next.markdown) {
                        Ok(()) => "Copied Markdown to clipboard".to_string(),
                        Err(error) => format!("Copy failed: {error}"),
                    };

                    next = reduce(next, AppAction::SetNotice(message));
                    persist_state(&next);
                    app_state.set(next);
                },
                on_export: move |_| {
                    let mut next = app_state();

                    let message = match crate::platform::dialogs::pick_export_html_save_path() {
                        Some(path) => {
                            let css = include_str!("../assets/github_markdown_light.css");

                            match crate::export::html::save_html(
                                &path,
                                "Crab Paperwork Preview",
                                &next.rendered_html,
                                css,
                            ) {
                                Ok(()) => format!("Exported PDF: {}", path.display()),
                                Err(error) => format!("{error}"),
                            }
                        }
                        None => "Export canceled".to_string(),
                    };

                    next = reduce(next, AppAction::SetNotice(message));
                    persist_state(&next);
                    app_state.set(next);
                },
                on_toggle_sync_scroll: move |_| {
                    let next = reduce(app_state(), AppAction::ToggleSyncScroll);
                    persist_state(&next);
                    app_state.set(next);
                },
                on_toggle_theme: move |_| {
                    let next = reduce(app_state(), AppAction::ToggleTheme);
                    persist_state(&next);
                    app_state.set(next);
                }
            }

            Split {
                markdown: current.markdown,
                rendered_html: current.rendered_html,
                sync_scroll: current.sync_scroll,
                on_markdown_change: move |value| {
                    let next = reduce(app_state(), AppAction::UpdateMarkdown(value));
                    persist_state(&next);
                    app_state.set(next);
                }
            }

            Status {
                notice: current.notice
            }
        }
    }
}

fn load_initial_state() -> AppState {
    let mut state = actions::initial_state();

    if let Ok(config) = crate::storage::load_config() {
        state.theme = ThemeMode::from_config_value(&config.theme);
        state.sync_scroll = config.sync_scroll;
        state.split_ratio = config.normalized_split_ratio();
    }

    if let Ok(session) = crate::storage::load_session() {
        if !session.last_markdown.trim().is_empty() {
            state.markdown = session.last_markdown;
            state.rendered_html = crate::markdown::render::render_markdown(&state.markdown);
        }

        state.current_file = session.current_file.map(Into::into);
    }

    state
}

fn persist_state(state: &AppState) {
    let config = AppConfig {
        theme: state.theme.as_str().to_string(),
        sync_scroll: state.sync_scroll,
        split_ratio: state.split_ratio,
    };

    let session = SessionState {
        last_markdown: state.markdown.clone(),
        current_file: state
            .current_file
            .as_ref()
            .map(|path| path.display().to_string()),
    };

    let _ = crate::storage::save_config(&config);
    let _ = crate::storage::save_session(&session);
}
