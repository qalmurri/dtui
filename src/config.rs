use crate::keybindings::validate;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, io, path::PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub desktop: DesktopConfig,
    pub runtime: RuntimeConfig,
    pub lua: LuaConfig,

    #[serde(skip)]
    pub keybindings: Keybindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DesktopConfig {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RuntimeConfig {
    pub poll_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LuaConfig {
    pub enabled: bool,
    pub init_script: String,
    pub stop_on_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Keybindings {
    pub quit: String,
    pub help: String,

    pub move_left: String,
    pub move_down: String,
    pub move_up: String,
    pub move_right: String,

    pub resize_left: String,
    pub resize_down: String,
    pub resize_up: String,
    pub resize_right: String,

    pub focus_next: String,
    pub close_window: String,
    pub new_window: String,

    pub workspace_1: String,
    pub workspace_2: String,
    pub workspace_3: String,
    pub workspace_4: String,
    pub workspace_5: String,
    pub workspace_6: String,
    pub workspace_7: String,
    pub workspace_8: String,
    pub workspace_9: String,

    pub workspace_previous: String,
    pub workspace_next: String,
    pub workspace_create: String,
    pub workspace_rename: String,
    pub workspace_delete: String,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            title: "TUICLI".into(),
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 8,
        }
    }
}

impl Default for LuaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            init_script: "init.lua".into(),
            stop_on_error: false,
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: "q".into(),
            help: "?".into(),

            move_left: "h".into(),
            move_down: "j".into(),
            move_up: "k".into(),
            move_right: "l".into(),

            resize_left: "alt+h".into(),
            resize_down: "alt+j".into(),
            resize_up: "alt+k".into(),
            resize_right: "alt+l".into(),

            focus_next: "tab".into(),
            close_window: "x".into(),
            new_window: "n".into(),

            workspace_1: "1".into(),
            workspace_2: "2".into(),
            workspace_3: "3".into(),
            workspace_4: "4".into(),
            workspace_5: "5".into(),
            workspace_6: "6".into(),
            workspace_7: "7".into(),
            workspace_8: "8".into(),
            workspace_9: "9".into(),

            workspace_previous: "[".into(),
            workspace_next: "]".into(),
            workspace_create: "c".into(),
            workspace_rename: "r".into(),
            workspace_delete: "d".into(),
        }
    }
}

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

    let contents = fs::read_to_string(keybindings_file)?;
    config.keybindings = toml::from_str(&contents)?;

    if config.runtime.poll_interval_ms == 0 {
        return Err("runtime.poll_interval_ms harus lebih besar dari 0".into());
    }

    validate(&config.keybindings)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_uses_tuicli_path() {
        let path = config_dir().expect("config directory should exist");

        assert!(
            path.ends_with("tuicli"),
            "Unexpected config directory: {}",
            path.display()
        );
    }
}
