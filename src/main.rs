mod app;
mod core;
mod desktop;
mod event;
mod ui;
mod window_manager;

use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let _guard = TerminalGuard;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    app::run(&mut terminal)?;

    Ok(())
}
