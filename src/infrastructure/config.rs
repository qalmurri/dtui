pub use crate::application::settings::{
    Config, DesktopConfig, Keybindings, LuaConfig, RuntimeConfig,
};
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
    eprintln!("tuicli config directory: {}", path.display());

    Ok(path)
}

pub fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn keybindings_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("keybindings.toml"))
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
    let keybindings_file = keybindings_path()?;

    ensure_file(&config_file, &Config::default())?;
    ensure_file(&keybindings_file, &Keybindings::default())?;

    let contents = fs::read_to_string(config_file)?;
    let mut config: Config = toml::from_str(&contents)?;

    let keybindings_contents = fs::read_to_string(&keybindings_file)?;
    config.keybindings = toml::from_str(&keybindings_contents)?;

    // Migrasikan keybindings bawaan lama agar perubahan default juga berlaku
    // bagi pengguna yang sudah memiliki keybindings.toml.
    if config.keybindings.move_left == "h"
        && config.keybindings.move_down == "j"
        && config.keybindings.move_up == "k"
        && config.keybindings.move_right == "l"
    {
        config.keybindings.move_left = "left".into();
        config.keybindings.move_down = "down".into();
        config.keybindings.move_up = "up".into();
        config.keybindings.move_right = "right".into();
    }

    if config.keybindings.resize_left == "alt+h" {
        config.keybindings.resize_left = "alt+left".into();
    }
    if config.keybindings.resize_down == "alt+j" {
        config.keybindings.resize_down = "alt+down".into();
    }
    if config.keybindings.resize_up == "alt+k" {
        config.keybindings.resize_up = "alt+up".into();
    }
    if config.keybindings.resize_right == "alt+l" {
        config.keybindings.resize_right = "alt+right".into();
    }

    if config.keybindings.fit_left == "win+left" || config.keybindings.fit_left == "super+left" {
        config.keybindings.fit_left = "ctrl+left".into();
    }
    if config.keybindings.fit_down == "win+down" || config.keybindings.fit_down == "super+down" {
        config.keybindings.fit_down = "ctrl+down".into();
    }
    if config.keybindings.fit_up == "win+up" || config.keybindings.fit_up == "super+up" {
        config.keybindings.fit_up = "ctrl+up".into();
    }
    if config.keybindings.fit_right == "win+right" || config.keybindings.fit_right == "super+right"
    {
        config.keybindings.fit_right = "ctrl+right".into();
    }

    if config.runtime.poll_interval_ms == 0 {
        return Err("runtime.poll_interval_ms harus lebih besar dari 0".into());
    }

    Ok(config)
}

#[cfg(test)]
#[path = "../../tests/unit/config.rs"]
mod tests;
