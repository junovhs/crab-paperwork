use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub theme: String,
    pub sync_scroll: bool,
    pub split_ratio: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            sync_scroll: false,
            split_ratio: 0.5,
        }
    }
}

impl AppConfig {
    pub fn normalized_split_ratio(&self) -> f32 {
        self.split_ratio.clamp(0.2, 0.8)
    }
}
