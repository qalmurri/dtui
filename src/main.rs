mod app;
mod config;
mod event;
mod identity;
mod keybindings;
mod ui;

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
    let args: Vec<String> = std::env::args().collect();

    // Membuat identitas persisten:
    // cargo run -p client -- --create-identity Qalmurri
    if args.len() == 3 && args[1] == "--create-identity" {
        let identity = identity::Identity::create_persistent(&args[2])?;

        println!("Identitas Srawung berhasil dibuat.");
        println!("Alias : {}", identity.alias());
        println!("Mode  : {}", identity.mode_label());
        println!("ID    : {}", identity.display_id());
        println!("Lokasi: {}", identity::identity_path()?.display());

        return Ok(());
    }

    if args.len() > 1 {
        eprintln!("Penggunaan:");
        eprintln!("  srawung");
        eprintln!("  srawung --create-identity ALIAS");
        std::process::exit(2);
    }

    let config = config::load_or_create()?;
    let identity = identity::Identity::load_or_anonymous()?;

    let _guard = TerminalGuard;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    terminal.clear()?;

    let result = app::run(&mut terminal, &config, &identity);

    drop(terminal);

    result
}
