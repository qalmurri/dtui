use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::Stdout;

pub mod app;
pub mod applications;
pub mod config;
pub mod controller;
pub mod core;
pub mod desktop;
pub mod keybindings;
pub mod scripting;
pub mod session;
pub mod ui;
pub mod window_manager;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;
