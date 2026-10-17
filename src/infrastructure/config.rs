pub use crate::application::settings::{Config, DesktopConfig, Keybindings, RuntimeConfig};
use directories::ProjectDirs;
use serde::Serialize;
use std::{error::Error, fs, io, path::PathBuf};

pub fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dirs = ProjectDirs::from("com", "qalmurri", "tuicli").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Tidak dapat menentukan direktori konfigurasi",
        )
    })?;

    let path = dirs.config_dir().to_path_buf();
    Ok(path)
}

pub fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("config.toml"))
}

fn ensure_file<T: Serialize>(path: &PathBuf, value: &T) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, toml::to_string_pretty(value)?)?;
    Ok(())
}

pub fn load_or_create() -> Result<Config, Box<dyn Error>> {
    let config_file = config_path()?;
    ensure_file(&config_file, &Config::default())?;
    let contents = fs::read_to_string(config_file)?;
    parse_config(&contents)
}

/// Load existing configuration without creating files or directories.
pub fn load() -> Result<Config, Box<dyn Error>> {
    let path = config_path()?;
    match fs::read_to_string(path) {
        Ok(contents) => parse_config(&contents),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Config::default()),
        Err(error) => Err(error.into()),
    }
}

fn parse_config(contents: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(contents)?;

    if !(16..=250).contains(&config.runtime.poll_interval_ms) {
        return Err("runtime.poll_interval_ms harus berada pada rentang 16 hingga 250 ms".into());
    }

    Ok(config)
}

#[cfg(test)]
#[path = "../../tests/unit/config.rs"]
mod tests;
