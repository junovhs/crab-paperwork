use crate::app::defaults;
use crate::app::state::{AppState, ThemeMode};
use crate::markdown;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppAction {
    UpdateMarkdown(String),
    OpenMarkdown { path: PathBuf, markdown: String },
    SaveMarkdown { path: PathBuf },
    Reset,
    ToggleTheme,
    ToggleSyncScroll,
    SetNotice(String),
}

pub fn reduce(mut state: AppState, action: AppAction) -> AppState {
    match action {
        AppAction::UpdateMarkdown(markdown_text) => {
            let rendered_html = markdown::render::render_markdown(&markdown_text);

            state.markdown = markdown_text;
            state.rendered_html = rendered_html;
            state.dirty = true;
            state.notice = "Editing".to_string();
        }

        AppAction::OpenMarkdown { path, markdown } => {
            let rendered_html = markdown::render::render_markdown(&markdown);
            let display = path.display().to_string();

            state.markdown = markdown;
            state.rendered_html = rendered_html;
            state.current_file = Some(path);
            state.dirty = false;
            state.notice = format!("Opened {display}");
        }

        AppAction::SaveMarkdown { path } => {
            let display = path.display().to_string();

            state.current_file = Some(path);
            state.dirty = false;
            state.notice = format!("Saved {display}");
        }

        AppAction::Reset => {
            let markdown_text = defaults::DEFAULT_MARKDOWN.to_string();
            let rendered_html = markdown::render::render_markdown(&markdown_text);

            state.markdown = markdown_text;
            state.rendered_html = rendered_html;
            state.current_file = None;
            state.dirty = true;
            state.notice = "Reset to sample Markdown".to_string();
        }

        AppAction::ToggleTheme => {
            state.theme = match state.theme {
                ThemeMode::Light => ThemeMode::Dark,
                ThemeMode::Dark => ThemeMode::Light,
            };

            state.dirty = true;
            state.notice = format!("Theme: {}", state.theme.as_str());
        }

        AppAction::ToggleSyncScroll => {
            state.sync_scroll = !state.sync_scroll;
            state.dirty = true;

            state.notice = if state.sync_scroll {
                "Sync scroll enabled".to_string()
            } else {
                "Sync scroll disabled".to_string()
            };
        }

        AppAction::SetNotice(message) => {
            state.notice = message;
        }
    }

    state
}

pub fn initial_state() -> AppState {
    let markdown = defaults::DEFAULT_MARKDOWN.to_string();
    let rendered_html = markdown::render::render_markdown(&markdown);

    AppState {
        markdown,
        rendered_html,
        ..AppState::default()
    }
}
