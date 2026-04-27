use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "CrabPaperwork";
const APPLICATION: &str = "Crab Paperwork";

pub fn project_dirs() -> Result<ProjectDirs> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .context("could not determine platform app directories")
}

pub fn config_dir() -> Result<PathBuf> {
    let dirs = project_dirs()?;
    let path = dirs.config_dir().to_path_buf();

    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create config directory: {}", path.display()))?;

    Ok(path)
}

pub fn data_dir() -> Result<PathBuf> {
    let dirs = project_dirs()?;
    let path = dirs.data_dir().to_path_buf();

    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create data directory: {}", path.display()))?;

    Ok(path)
}

pub fn config_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn session_file() -> Result<PathBuf> {
    Ok(data_dir()?.join("session.toml"))
}
