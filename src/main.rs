use std::{
    error::Error,
    io::{self, IsTerminal, Write},
};

use crossterm::{
    cursor::Show,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
        supports_keyboard_enhancement,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tuicli::{
    application::ports::SessionRepository,
    infrastructure::{
        config,
        pty::{MANAGED_SHELL_ENV, TerminalManager},
        session,
    },
    presentation,
};

struct TerminalGuard {
    keyboard_enhancement: bool,
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        if self.keyboard_enhancement {
            let _ = execute!(stdout, PopKeyboardEnhancementFlags);
        }

        let _ = disable_raw_mode();
        let _ = execute!(stdout, Show, LeaveAlternateScreen);
        let _ = stdout.flush();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var_os(MANAGED_SHELL_ENV).as_deref() == Some(std::ffi::OsStr::new("1")) {
        return Err("tuicli sudah berjalan; keluar dari shell/window tuicli sebelum menjalankan instance baru".into());
    }

    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("tuicli harus dijalankan dari terminal interaktif (TTY), bukan melalui pipe atau redirect".into());
    }

    let config = config::load_or_create()?;
    let keymap =
        tuicli::presentation::controller::keybindings::Keymap::compile(&config.keybindings)?;

    let session_repository = session::FileSessionRepository;
    let (saved_session, persist_session) = match session_repository.load() {
        Ok(session) => (session, true),
        Err(error) => {
            eprintln!("TUICLI: sesi tidak dapat dipulihkan: {error}");
            (None, false)
        }
    };

    enable_raw_mode()?;

    let mut guard = TerminalGuard {
        keyboard_enhancement: false,
    };

    execute!(io::stdout(), EnterAlternateScreen)?;

    if supports_keyboard_enhancement().unwrap_or(false) {
        execute!(
            io::stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
            )
        )?;

        guard.keyboard_enhancement = true;
    }

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    tuicli::infrastructure::signals::install()?;

    presentation::runtime::run(
        &mut terminal,
        &config,
        &keymap,
        saved_session,
        persist_session,
        Box::new(TerminalManager::default()),
        Box::new(session_repository),
    )?;

    Ok(())
}
