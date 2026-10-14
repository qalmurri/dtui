mod app;
mod config;
mod core;
mod desktop;
mod keybindings;
mod scripting;
mod session;
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
    let config = config::load_or_create()?;

    if config.lua.enabled
        && let Err(error) = scripting::lua_runtime::run_init(&config)
    {
        if config.lua.stop_on_error {
            return Err(error);
        }

        eprintln!("TUICLI Lua warning: {error}");
    }

    let saved_session = match session::SessionState::load() {
        Ok(session) => session,
        Err(error) => {
            eprintln!("TUICLI: sesi tidak dapat dipulihkan: {error}");
            None
        }
    };

    enable_raw_mode()?;
    let _guard = TerminalGuard;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    app::run(&mut terminal, &config, saved_session)?;
    Ok(())
}
