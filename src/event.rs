use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if !event::poll(Duration::from_millis(100))? {
        return Ok(());
    }

    let event = event::read()?;

    if let Event::Key(key) = event {
        handle_key(app, key);
    }

    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.quit();
        return;
    }

    match key.code {
        KeyCode::Char('q') => {
            app.quit();
        }

        KeyCode::Up => {
            app.previous();
        }

        KeyCode::Down => {
            app.next();
        }

        KeyCode::Enter => {
            app.enter();
        }

        KeyCode::Esc => {
            app.back();
        }

        _ => {}
    }
}
