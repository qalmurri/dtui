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
    let config: Config = toml::from_str(&contents)?;

    if config.runtime.poll_interval_ms == 0 {
        return Err("runtime.poll_interval_ms harus lebih besar dari 0".into());
    }

    Ok(config)
}

#[cfg(test)]
#[path = "../../tests/unit/config.rs"]
mod tests;
