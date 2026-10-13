use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEventKind};

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
    )?;
    let windows = state.window_manager.windows.clone();
    let _ = state
        .terminals
        .resize_all(&windows, terminal_size.width, desktop_height);

    let poll_interval = Duration::from_millis(config.runtime.poll_interval_ms);

    while state.running {
        state.terminals.poll();

        // Close windows whose shell processes have exited.
        let exited_windows = state.terminals.exited_window_ids();

        for window_id in exited_windows {
            state.terminals.remove(window_id);
            state.window_manager.close(window_id);
        }

        terminal.draw(|frame| renderer::render(frame, &state, config))?;

        let Some(event) = event_runtime::poll(poll_interval)? else {
            continue;
        };

        let key = match event {
            Event::Resize(width, height) => {
                let windows = state.window_manager.windows.clone();
                let _ = state
                    .terminals
                    .resize_all(&windows, width, height.saturating_sub(2));
                continue;
            }
            Event::Key(key) => key,
            _ => continue,
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

        // F2 switches between shell input and desktop shortcuts.
        if key.code == KeyCode::F(2) {
            state.terminal_input_mode = !state.terminal_input_mode;
            state.desktop.status.message = if state.terminal_input_mode {
                "Terminal input mode (F2: desktop mode)".into()
            } else {
                "Desktop mode (F2: terminal input)".into()
            };
            continue;
        }

        if state.terminal_input_mode {
            if let Some(window_id) = state.window_manager.focus.window_id {
                state.terminals.write_key(window_id, key)?;
            }
            continue;
        }

        let Some(action) = keybindings::resolve(key, &config.keybindings) else {
            continue;
        };

        actions::handle(&mut state, action, terminal)?;
    }

    state.terminals.close_processes();
    session::save(&state)?;
    Ok(())
}
