use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SessionState {
    pub last_markdown: String,
    pub current_file: Option<String>,
}
