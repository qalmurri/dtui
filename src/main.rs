use std::{
    error::Error,
    io::{self, Write},
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
    infrastructure::{config, pty::TerminalManager, session},
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
    let config = config::load_or_create()?;
    tuicli::presentation::controller::keybindings::validate(&config.keybindings)?;

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
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
            )
        )?;

        guard.keyboard_enhancement = true;
    }

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    presentation::runtime::run(
        &mut terminal,
        &config,
        saved_session,
        persist_session,
        Box::new(TerminalManager::default()),
        Box::new(session_repository),
    )?;

    Ok(())
}
