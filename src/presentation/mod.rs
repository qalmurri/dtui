pub mod controller;
pub mod runtime;
pub mod terminal;
pub mod ui;

use std::io::Stdout;

use ratatui::{Terminal, backend::CrosstermBackend};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;
