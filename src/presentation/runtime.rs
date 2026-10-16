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
        ui::layout,
    },
};

pub fn run(
    terminal: &mut Tui,
    config: &Config,
    keymap: &controller::keybindings::Keymap,
    saved_session: Option<SessionState>,
    persist_session: bool,
    terminal_backend: Box<dyn TerminalBackend>,
    session_repository: Box<dyn SessionRepository>,
) -> Result<(), Box<dyn Error>> {
    let mut state = AppState::new(terminal_backend);

    let terminal_size = terminal.size()?;
    let (desktop_width, desktop_height) =
        layout::desktop_size(terminal_size.width, terminal_size.height);

    application::restore_or_initialize(&mut state, saved_session, desktop_width, desktop_height)?;

    let windows = state.window_manager.windows.clone();

    if let Err(error) = state
        .terminals
        .resize_all(&windows, desktop_width, desktop_height)
    {
        state.desktop.status.message = format!("Gagal resize terminal saat startup: {error}");
    }

    let poll_interval = Duration::from_millis(config.runtime.poll_interval_ms);
    let mut redraw = true;
    let mut last_clock_second = unix_second();
    state.cpu_frequency_ghz = crate::infrastructure::cpu_frequency::average_ghz();
    while state.running && !crate::infrastructure::signals::shutdown_requested() {
        redraw |= state.terminals.poll();

        let exited = state.terminals.exited_window_ids();
        for window_id in exited.iter().copied() {
            state.terminals.remove(window_id);
            state.window_manager.close(window_id);
        }
        redraw |= !exited.is_empty();

        let clock_second = unix_second();
        if clock_second != last_clock_second {
            last_clock_second = clock_second;
            state.cpu_frequency_ghz = crate::infrastructure::cpu_frequency::average_ghz();
            redraw = true;
        }

        if redraw {
            terminal.draw(|frame| renderer::render(frame, &state, config))?;
            redraw = false;
        }

        let event = match event_runtime::poll(poll_interval) {
            Ok(event) => event,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error.into()),
        };
        let Some(event) = event else {
            continue;
        };

        let key = match event {
            Event::Resize(width, height) => {
                let windows = state.window_manager.windows.clone();
                let (desktop_width, desktop_height) = layout::desktop_size(width, height);

                if let Err(error) =
                    state
                        .terminals
                        .resize_all(&windows, desktop_width, desktop_height)
                {
                    state.desktop.status.message = format!("Gagal resize terminal: {error}");
                }

                redraw = true;
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
                redraw = true;
            }

            continue;
        }

        if state.show_help {
            if key.kind == KeyEventKind::Press {
                state.show_help = false;
                redraw = true;
            }

            continue;
        }

        if key.kind == KeyEventKind::Press
            && controller::keybindings::resolve(key, keymap)
                == Some(application::action::Action::ToggleMode)
        {
            let size = terminal.size()?;
            let (desktop_width, desktop_height) = layout::desktop_size(size.width, size.height);
            application::handle(
                &mut state,
                application::action::Action::ToggleMode,
                desktop_width,
                desktop_height,
            )?;
            redraw = true;
            continue;
        }

        // Terminal input tidak melewati controller desktop.
        // Press dan Repeat diteruskan ke PTY.
        if state.terminal_input_mode {
            if key.kind != KeyEventKind::Release
                && let Some(window_id) = state.window_manager.focus.window_id
            {
                let input = controller::terminal_input::key_to_bytes(key);
                if let Err(error) = state.terminals.write_input(window_id, &input) {
                    state.terminals.remove(window_id);
                    state.window_manager.close(window_id);
                    state.desktop.status.message = format!("Sesi terminal berhenti: {error}");
                    redraw = true;
                }
            }

            continue;
        }

        let Some(action) = controller::input::resolve(key, keymap) else {
            continue;
        };

        let size = terminal.size()?;
        let (desktop_width, desktop_height) = layout::desktop_size(size.width, size.height);
        application::handle(&mut state, action, desktop_width, desktop_height)?;
        redraw = true;
    }

    state.terminals.close_processes();
    if persist_session {
        let session = application::to_session_state(&state);
        session_repository.save(&session)?;
    }

    Ok(())
}

fn unix_second() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}
