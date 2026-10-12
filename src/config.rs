use std::{error::Error, fs, io, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub desktop: DesktopConfig,
    pub appearance: AppearanceConfig,
    pub layout: LayoutConfig,
    pub components: ComponentsConfig,
    pub menu: MenuConfig,
    pub behavior: BehaviorConfig,
    pub runtime: RuntimeConfig,
    pub lua: LuaConfig,

    #[serde(skip)]
    pub keybindings: Keybindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DesktopConfig {
    pub title: String,
    pub subtitle: String,
    pub startup_screen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub theme: String,
    pub accent: String,
    pub muted: String,
    pub foreground: String,
    pub background: String,
    pub show_borders: bool,
    pub border_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LayoutConfig {
    pub kind: String,
    pub show_header: bool,
    pub show_footer: bool,
    pub horizontal_padding: u16,
    pub vertical_padding: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ComponentsConfig {
    pub header: HeaderConfig,
    pub status: StatusConfig,
    pub shortcut_help: ShortcutHelpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HeaderConfig {
    pub enabled: bool,
    pub alignment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StatusConfig {
    pub enabled: bool,
    pub position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutHelpConfig {
    pub enabled: bool,
    pub trigger: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MenuConfig {
    pub enabled: bool,
    pub trigger: String,
    pub close_on_select: bool,
    pub items: Vec<MenuItemConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemConfig {
    pub id: String,
    pub label: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BehaviorConfig {
    pub confirm_quit: bool,
    pub restore_terminal_on_exit: bool,
    pub close_menu_on_escape: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RuntimeConfig {
    pub poll_interval_ms: u64,
    pub redraw_on_input: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LuaConfig {
    pub enabled: bool,
    pub init_script: String,
    pub stop_on_error: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            desktop: DesktopConfig::default(),
            appearance: AppearanceConfig::default(),
            layout: LayoutConfig::default(),
            components: ComponentsConfig::default(),
            menu: MenuConfig::default(),
            behavior: BehaviorConfig::default(),
            runtime: RuntimeConfig::default(),
            lua: LuaConfig::default(),
            keybindings: Keybindings::default(),
        }
    }
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            title: "DTUI".into(),
            subtitle: "A customizable terminal desktop".into(),
            startup_screen: "desktop".into(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: "default".into(),
            accent: "cyan".into(),
            muted: "gray".into(),
            foreground: "white".into(),
            background: "black".into(),
            show_borders: false,
            border_style: "plain".into(),
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            kind: "fullscreen".into(),
            show_header: true,
            show_footer: false,
            horizontal_padding: 1,
            vertical_padding: 0,
        }
    }
}

impl Default for ComponentsConfig {
    fn default() -> Self {
        Self {
            header: HeaderConfig::default(),
            status: StatusConfig::default(),
            shortcut_help: ShortcutHelpConfig::default(),
        }
    }
}

impl Default for HeaderConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            alignment: "left".into(),
        }
    }
}

impl Default for StatusConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            position: "top".into(),
        }
    }
}

impl Default for ShortcutHelpConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            trigger: "?".into(),
        }
    }
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            trigger: "enter".into(),
            close_on_select: true,
            items: vec![
                MenuItemConfig {
                    id: "terminal".into(),
                    label: "Terminal".into(),
                    enabled: true,
                },
                MenuItemConfig {
                    id: "file_browser".into(),
                    label: "File Browser".into(),
                    enabled: true,
                },
                MenuItemConfig {
                    id: "system_monitor".into(),
                    label: "System Monitor".into(),
                    enabled: true,
                },
                MenuItemConfig {
                    id: "settings".into(),
                    label: "Settings".into(),
                    enabled: true,
                },
            ],
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            confirm_quit: false,
            restore_terminal_on_exit: true,
            close_menu_on_escape: true,
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 100,
            redraw_on_input: true,
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

pub fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dirs = ProjectDirs::from("com", "qalmurri", "dtui").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Tidak dapat menentukan direktori konfigurasi",
        )
    })?;

    Ok(dirs.config_dir().to_path_buf())
}

pub fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn keybindings_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(config_dir()?.join("keybindings.toml"))
}

fn ensure_file<T: Serialize>(path: &PathBuf, value: &T) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, toml::to_string_pretty(value)?)?;
    }

    Ok(())
}

pub fn load_or_create() -> Result<Config, Box<dyn Error>> {
    let config_file = config_path()?;
    let keybindings_file = keybindings_path()?;

    let default_config = Config::default();
    ensure_file(&config_file, &default_config)?;

    let default_keybindings = Keybindings::default();
    ensure_file(&keybindings_file, &default_keybindings)?;

    let config_contents = fs::read_to_string(&config_file)?;
    let mut config: Config = toml::from_str(&config_contents)?;

    let keybindings_contents = fs::read_to_string(&keybindings_file)?;
    config.keybindings = toml::from_str(&keybindings_contents)?;

    crate::keybindings::validate(&config.keybindings)?;

    validate_config(&config)?;

    Ok(config)
}

fn validate_config(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.runtime.poll_interval_ms == 0 {
        return Err("runtime.poll_interval_ms harus lebih besar dari 0".into());
    }

    if config.desktop.title.trim().is_empty() {
        return Err("desktop.title tidak boleh kosong".into());
    }

    if config
        .menu
        .items
        .iter()
        .any(|item| item.id.trim().is_empty() || item.label.trim().is_empty())
    {
        return Err("ID dan label menu tidak boleh kosong".into());
    }

    let mut ids = std::collections::HashSet::new();

    for item in &config.menu.items {
        if !ids.insert(&item.id) {
            return Err(format!("ID menu duplikat: '{}'", item.id).into());
        }
    }

    Ok(())
}
