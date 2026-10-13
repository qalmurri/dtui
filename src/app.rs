use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyEventKind};

use crate::{
    Tui,
    config::Config,
    core::{renderer, runtime},
    desktop::{DesktopState, workspace::WorkspaceId},
    keybindings::{self, Action},
    window_manager::{Geometry, WindowManager},
};

pub struct AppState {
    pub running: bool,
    pub show_help: bool,
    pub desktop: DesktopState,
    pub window_manager: WindowManager,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            running: true,
            show_help: false,
            desktop: DesktopState::default(),
            window_manager: WindowManager::default(),
        }
    }
}

pub fn run(terminal: &mut Tui, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut state = AppState::default();

    let terminal_size = terminal.size()?;
    let desktop_height = terminal_size.height.saturating_sub(2);

    state.window_manager.create(
        "Terminal",
        Geometry::cascade(0, terminal_size.width, desktop_height),
        WorkspaceId(0),
    );

    state.window_manager.create(
        "System Monitor",
        Geometry::cascade(0, terminal_size.width, desktop_height),
        WorkspaceId(1),
    );

    state.window_manager.activate_workspace(WorkspaceId(0));

    let poll_interval = Duration::from_millis(config.runtime.poll_interval_ms);

    while state.running {
        terminal.draw(|frame| {
            renderer::render(frame, &state);
        })?;

        let Some(Event::Key(key)) = runtime::poll(poll_interval)? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        if state.show_help {
            state.show_help = false;
            continue;
        }

        let Some(action) = keybindings::resolve(key, &config.keybindings) else {
            continue;
        };

        let workspace_id = state
            .desktop
            .active_workspace()
            .map(|workspace| workspace.id)
            .unwrap_or(WorkspaceId(0));

        match action {
            Action::Quit => {
                state.running = false;
            }

            Action::Help => {
                state.show_help = true;
            }

            Action::MoveLeft => {
                move_focused_window(&mut state, terminal, -2, 0, workspace_id)?;
            }

            Action::MoveDown => {
                move_focused_window(&mut state, terminal, 0, 1, workspace_id)?;
            }

            Action::MoveUp => {
                move_focused_window(&mut state, terminal, 0, -1, workspace_id)?;
            }

            Action::MoveRight => {
                move_focused_window(&mut state, terminal, 2, 0, workspace_id)?;
            }

            Action::ResizeLeft | Action::ResizeDown | Action::ResizeUp | Action::ResizeRight => {
                let (dw, dh) = match action {
                    Action::ResizeLeft => (-1, 0),
                    Action::ResizeDown => (0, 1),
                    Action::ResizeUp => (0, -1),
                    Action::ResizeRight => (1, 0),
                    _ => unreachable!(),
                };

                let size = terminal.size()?;

                state.window_manager.resize_focused(
                    dw,
                    dh,
                    size.width,
                    size.height.saturating_sub(2),
                    workspace_id,
                );
            }

            Action::FocusNext => {
                state.window_manager.focus_next(workspace_id);
            }

            Action::CloseWindow => {
                if let Some(window) = state.window_manager.focused() {
                    if window.workspace_id == workspace_id {
                        let id = window.id;
                        state.window_manager.close(id, workspace_id);
                    }
                }
            }

            Action::NewWindow => {
                let size = terminal.size()?;
                let desktop_height = size.height.saturating_sub(2);

                let count = state
                    .window_manager
                    .windows
                    .iter()
                    .filter(|window| window.workspace_id == workspace_id)
                    .count();

                let title = format!("Window {}", state.window_manager.windows.len() + 1);

                state.window_manager.create(
                    title,
                    Geometry::cascade(count, size.width, desktop_height),
                    workspace_id,
                );
            }

            Action::Workspace1 => {
                state.desktop.select_workspace(0);
                state.window_manager.activate_workspace(WorkspaceId(0));
            }

            Action::Workspace2 => {
                state.desktop.select_workspace(1);
                state.window_manager.activate_workspace(WorkspaceId(1));
            }
        }
    }

    Ok(())
}

fn move_focused_window(
    state: &mut AppState,
    terminal: &Tui,
    dx: i16,
    dy: i16,
    workspace_id: WorkspaceId,
) -> Result<(), Box<dyn Error>> {
    let terminal_size = terminal.size()?;

    state.window_manager.move_focused(
        dx,
        dy,
        terminal_size.width,
        terminal_size.height.saturating_sub(2),
        workspace_id,
    );

    Ok(())
}
