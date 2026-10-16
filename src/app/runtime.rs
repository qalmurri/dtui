use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::{
    Tui,
    config::Config,
    controller,
    core::{renderer, runtime as event_runtime},
};

use super::{AppState, actions, session, workspace};

pub fn run(
    terminal: &mut Tui,
    config: &Config,
    saved_session: Option<crate::session::SessionState>,
    persist_session: bool,
) -> Result<(), Box<dyn Error>> {
    let mut state = AppState::default();

    let terminal_size = terminal.size()?;
    let desktop_height = terminal_size.height.saturating_sub(1);

    session::restore_or_initialize(
        &mut state,
        saved_session,
        terminal_size.width,
        desktop_height,
    )?;

    let windows = state.window_manager.windows.clone();

    if let Err(error) = state
        .terminals
        .resize_all(&windows, terminal_size.width, desktop_height)
    {
        state.desktop.status.message = format!("Gagal resize terminal saat startup: {error}");
    }

    let poll_interval = Duration::from_millis(config.runtime.poll_interval_ms);
    while state.running {
        state.terminals.poll();

        for window_id in state.terminals.exited_window_ids() {
            state.terminals.remove(window_id);
            state.window_manager.close(window_id);
        }

        terminal.draw(|frame| {
            renderer::render(frame, &state, config);
        })?;

        let Some(event) = event_runtime::poll(poll_interval)? else {
            continue;
        };

        let key = match event {
            Event::Resize(width, height) => {
                let windows = state.window_manager.windows.clone();

                if let Err(error) =
                    state
                        .terminals
                        .resize_all(&windows, width, height.saturating_sub(1))
                {
                    state.desktop.status.message = format!("Gagal resize terminal: {error}");
                }

                continue;
            }

            Event::Key(key) => key,
            _ => continue,
        };

        if key.kind == KeyEventKind::Release {
            continue;
        }

        if state.workspace_input.is_some() {
            if key.kind == KeyEventKind::Press {
                workspace::handle_input(&mut state, key.code);
            }

            continue;
        }

        if state.show_help {
            if key.kind == KeyEventKind::Press {
                state.show_help = false;
            }

            continue;
        }

        if key.kind == KeyEventKind::Press && key.code == KeyCode::F(2) {
            state.terminal_input_mode = !state.terminal_input_mode;

            state.desktop.status.message = if state.terminal_input_mode {
                "Terminal input mode (F2: desktop mode)".into()
            } else {
                "Desktop mode (F2: terminal input)".into()
            };

            continue;
        }

        // Terminal input tidak melewati controller desktop.
        // Press dan Repeat diteruskan ke PTY.
        if state.terminal_input_mode {
            if let Some(window_id) = state.window_manager.focus.window_id {
                state.terminals.write_key(window_id, key)?;
            }

            continue;
        }

        let Some(action) = controller::input::resolve(key, &config.keybindings) else {
            continue;
        };

        actions::handle(&mut state, action, terminal)?;
    }

    state.terminals.close_processes();
    if persist_session {
        session::save(&state)?;
    }

    Ok(())
}
