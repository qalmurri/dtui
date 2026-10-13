use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyEventKind};

use crate::{
    Tui,
    config::Config,
    core::{renderer, runtime as event_runtime},
    keybindings,
};

use super::{AppState, actions, session, workspace};

pub fn run(
    terminal: &mut Tui,
    config: &Config,
    saved_session: Option<crate::session::SessionState>,
) -> Result<(), Box<dyn Error>> {
    let mut state = AppState::default();
    let terminal_size = terminal.size()?;
    let desktop_height = terminal_size.height.saturating_sub(2);

    session::restore_or_initialize(
        &mut state,
        saved_session,
        terminal_size.width,
        desktop_height,
    );

    let poll_interval = Duration::from_millis(config.runtime.poll_interval_ms);

    while state.running {
        terminal.draw(|frame| renderer::render(frame, &state, config))?;

        let Some(Event::Key(key)) = event_runtime::poll(poll_interval)? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        if state.workspace_input.is_some() {
            workspace::handle_input(&mut state, key.code);
            continue;
        }

        if state.show_help {
            state.show_help = false;
            continue;
        }

        let Some(action) = keybindings::resolve(key, &config.keybindings) else {
            continue;
        };

        actions::handle(&mut state, action, terminal)?;
    }

    session::save(&state)?;
    Ok(())
}
