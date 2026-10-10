use std::{error::Error, fs, io};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::keybindings;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub keybindings: Keybindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Keybindings {
    pub quit: String,
    pub back: String,
    pub navigate_up: String,
    pub navigate_down: String,
    pub select: String,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: "q".into(),
            back: "esc".into(),
            navigate_up: "up".into(),
            navigate_down: "down".into(),
            select: "enter".into(),
        }
    }
}

pub fn config_path() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let dirs = ProjectDirs::from("com", "qalmurri", "srawung").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory",
        )
    })?;

    Ok(dirs.config_dir().join("config.toml"))
}

pub fn load_or_create() -> Result<Config, Box<dyn Error>> {
    let path = config_path()?;

    if !path.exists() {
        fs::create_dir_all(
            path.parent().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "Invalid config path")
            })?,
        )?;

        let default_config = Config::default();
        let contents = toml::to_string_pretty(&default_config)?;

        fs::write(&path, contents)?;
    }

    let contents = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&contents)?;

    keybindings::validate(&config.keybindings)?;

    Ok(config)
}
