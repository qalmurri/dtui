use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::{
    application::settings::Config,
    application::{
        self, AppState,
        ports::{SessionRepository, TerminalBackend},
    },
    domain::session::SessionState,
    presentation::{
        Tui, controller,
        terminal::{renderer, runtime as event_runtime},
    },
};

pub fn run(
    terminal: &mut Tui,
    config: &Config,
    saved_session: Option<SessionState>,
    persist_session: bool,
    terminal_backend: Box<dyn TerminalBackend>,
    session_repository: Box<dyn SessionRepository>,
) -> Result<(), Box<dyn Error>> {
    let mut state = AppState::new(terminal_backend);

    let terminal_size = terminal.size()?;
    let desktop_height = terminal_size.height.saturating_sub(1);

    application::restore_or_initialize(
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

    let poll_interval = Duration::from_millis(config.runtime.poll_interval_ms.max(16));
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
                let input = match key.code {
                    KeyCode::Esc => application::WorkspaceInputKey::Escape,
                    KeyCode::Enter => application::WorkspaceInputKey::Enter,
                    KeyCode::Backspace => application::WorkspaceInputKey::Backspace,
                    KeyCode::Char(ch) => application::WorkspaceInputKey::Character(ch),
                    _ => application::WorkspaceInputKey::Other,
                };
                application::handle_workspace_input(&mut state, input);
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
            if key.kind != KeyEventKind::Release
                && let Some(window_id) = state.window_manager.focus.window_id
            {
                let input = controller::terminal_input::key_to_bytes(key);
                state.terminals.write_input(window_id, &input)?;
            }

            continue;
        }

        let Some(action) = controller::input::resolve(key, &config.keybindings) else {
            continue;
        };

        let size = terminal.size()?;
        application::handle(&mut state, action, size.width, size.height)?;
    }

    state.terminals.close_processes();
    if persist_session {
        let session = application::to_session_state(&state);
        session_repository.save(&session)?;
    }

    Ok(())
}
