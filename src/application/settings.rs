use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub desktop: DesktopConfig,
    pub runtime: RuntimeConfig,

    #[serde(default, skip_serializing)]
    pub keybindings: Keybindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DesktopConfig {
    /// Local time format using `strftime` codes; empty displays the Unix timestamp.
    pub time_format: String,
    /// Show the average CPU frequency beside the clock.
    pub show_cpu_frequency: bool,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            time_format: String::new(),
            show_cpu_frequency: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RuntimeConfig {
    pub poll_interval_ms: u64,
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

    pub fit_left: String,
    pub fit_down: String,
    pub fit_up: String,
    pub fit_right: String,

    pub focus_next: String,
    pub toggle_mode: String,
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

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 33,
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: "q".into(),
            help: "?".into(),

            move_left: "left".into(),
            move_down: "down".into(),
            move_up: "up".into(),
            move_right: "right".into(),

            resize_left: "alt+left".into(),
            resize_down: "alt+down".into(),
            resize_up: "alt+up".into(),
            resize_right: "alt+right".into(),

            fit_left: "ctrl+left".into(),
            fit_down: "ctrl+down".into(),
            fit_up: "ctrl+up".into(),
            fit_right: "ctrl+right".into(),

            focus_next: "tab".into(),
            toggle_mode: "f2".into(),
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
