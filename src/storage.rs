pub mod config;
pub mod paths;
pub mod session;

use anyhow::{Context, Result};
use std::fs;

use config::AppConfig;
use session::SessionState;

pub fn load_config() -> Result<AppConfig> {
    let path = paths::config_file()?;

    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;

    let config = toml::from_str(&raw)
        .with_context(|| format!("failed to parse config file: {}", path.display()))?;

    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = paths::config_file()?;
    let raw = toml::to_string_pretty(config).context("failed to serialize config")?;

    fs::write(&path, raw)
        .with_context(|| format!("failed to write config file: {}", path.display()))?;

    Ok(())
}

pub fn load_session() -> Result<SessionState> {
    let path = paths::session_file()?;

    if !path.exists() {
        return Ok(SessionState::default());
    }

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read session file: {}", path.display()))?;

    let session = toml::from_str(&raw)
        .with_context(|| format!("failed to parse session file: {}", path.display()))?;

    Ok(session)
}

pub fn save_session(session: &SessionState) -> Result<()> {
    let path = paths::session_file()?;
    let raw = toml::to_string_pretty(session).context("failed to serialize session")?;

    fs::write(&path, raw)
        .with_context(|| format!("failed to write session file: {}", path.display()))?;

    Ok(())
}
