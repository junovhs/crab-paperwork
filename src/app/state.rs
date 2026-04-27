use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn css_class(self) -> &'static str {
        match self {
            Self::Light => "theme-light",
            Self::Dark => "theme-dark",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "dark" => Self::Dark,
            _ => Self::Light,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppState {
    pub markdown: String,
    pub rendered_html: String,
    pub theme: ThemeMode,
    pub sync_scroll: bool,
    pub split_ratio: f32,
    pub dirty: bool,
    pub notice: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            markdown: String::new(),
            rendered_html: String::new(),
            theme: ThemeMode::Light,
            sync_scroll: false,
            split_ratio: 0.5,
            dirty: false,
            notice: "Ready".to_string(),
        }
    }
}
