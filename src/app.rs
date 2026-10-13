use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::{
    Tui,
    config::Config,
    core::{renderer, runtime},
    desktop::{DesktopState, workspace::WorkspaceId},
    keybindings::{self, Action},
    window_manager::{Geometry, WindowManager},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceInputMode {
    Create,
    Rename,
}

pub struct WorkspaceInput {
    pub mode: WorkspaceInputMode,
    pub value: String,
}

pub struct AppState {
    pub running: bool,
    pub show_help: bool,
    pub desktop: DesktopState,
    pub window_manager: WindowManager,
    pub workspace_input: Option<WorkspaceInput>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            running: true,
            show_help: false,
            desktop: DesktopState::default(),
            window_manager: WindowManager::default(),
            workspace_input: None,
        }
    }
}

pub fn run(
    terminal: &mut Tui,
    config: &Config,
    saved_session: Option<crate::session::SessionState>,
) -> Result<(), Box<dyn Error>> {
    let mut state = AppState::default();
    let terminal_size = terminal.size()?;
    let desktop_height = terminal_size.height.saturating_sub(2);

    if let Some(session) = saved_session {
        use crate::desktop::workspace::WorkspaceId;
        use crate::window_manager::Window;

        let active_workspace_id = WorkspaceId(session.active_workspace_id);
        let windows = session
            .windows
            .iter()
            .map(|window| {
                Window::new(
                    window.id,
                    window.title.clone(),
                    window.geometry,
                    WorkspaceId(window.workspace_id),
                )
            })
            .collect();

        let restored = state.desktop.restore_workspaces(
            session.workspaces,
            active_workspace_id,
            session.next_workspace_id,
        );

        if restored {
            state.window_manager.restore_state(
                windows,
                session.z_order,
                session.focused_window_id,
                session.next_window_id,
            );

            // Jangan mengganti fokus tersimpan jika masih sesuai dengan
            // workspace aktif. Validasi load juga memastikan kondisi ini.
        } else {
            state.window_manager.create(
                "Terminal",
                Geometry::cascade(0, terminal_size.width, desktop_height),
                active_workspace_id,
            );
        }
    } else {
        // Startup pertama: siapkan satu window awal pada workspace MAIN.
        state.window_manager.create(
            "Terminal",
            Geometry::cascade(0, terminal_size.width, desktop_height),
            WorkspaceId(0),
        );
    }

    let poll_interval = Duration::from_millis(config.runtime.poll_interval_ms);

    while state.running {
        terminal.draw(|frame| {
            renderer::render(frame, &state, config);
        })?;

        let Some(Event::Key(key)) = runtime::poll(poll_interval)? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        if state.workspace_input.is_some() {
            handle_workspace_input(&mut state, key.code);
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
                        state.window_manager.close(id);
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

                let title = format!("Window {}", count + 1);

                state.window_manager.create(
                    title,
                    Geometry::cascade(count, size.width, desktop_height),
                    workspace_id,
                );
            }

            Action::Workspace1 => {
                if let Some(workspace) = state.desktop.workspaces.first() {
                    let id = workspace.id;
                    state.desktop.select_workspace_id(id);
                    state.window_manager.activate_workspace(id);
                }
            }

            Action::Workspace2 => {
                if let Some(workspace) = state.desktop.workspaces.get(1) {
                    let id = workspace.id;
                    state.desktop.select_workspace_id(id);
                    state.window_manager.activate_workspace(id);
                }
            }

            Action::WorkspacePrevious => {
                state.desktop.select_previous_workspace();

                if let Some(workspace) = state.desktop.active_workspace() {
                    state.window_manager.activate_workspace(workspace.id);
                }
            }

            Action::WorkspaceNext => {
                state.desktop.select_next_workspace();

                if let Some(workspace) = state.desktop.active_workspace() {
                    state.window_manager.activate_workspace(workspace.id);
                }
            }

            Action::WorkspaceCreate => {
                state.workspace_input = Some(WorkspaceInput {
                    mode: WorkspaceInputMode::Create,
                    value: String::new(),
                });
            }

            Action::WorkspaceRename => {
                if let Some(workspace) = state.desktop.active_workspace() {
                    state.workspace_input = Some(WorkspaceInput {
                        mode: WorkspaceInputMode::Rename,
                        value: workspace.name.clone(),
                    });
                }
            }

            Action::WorkspaceDelete => {
                if let Some((removed_id, target_id)) = state.desktop.remove_active_workspace() {
                    for window in &mut state.window_manager.windows {
                        if window.workspace_id == removed_id {
                            window.workspace_id = target_id;
                        }
                    }

                    state.window_manager.activate_workspace(target_id);
                }
            }
        }
    }

    let session = crate::session::SessionState {
        workspaces: state.desktop.workspaces.clone(),
        active_workspace_id: state
            .desktop
            .active_workspace()
            .map(|w| w.id.0)
            .unwrap_or(0),
        next_workspace_id: state.desktop.next_workspace_id(),
        windows: state
            .window_manager
            .windows
            .iter()
            .map(crate::session::SessionState::window_from)
            .collect(),
        z_order: state.window_manager.z_order.windows.clone(),
        focused_window_id: state.window_manager.focus.window_id,
        next_window_id: state.window_manager.next_id(),
    };

    session.save()?;

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

fn handle_workspace_input(state: &mut AppState, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            state.workspace_input = None;
            state.desktop.status.message = "Input dibatalkan".into();
        }

        KeyCode::Enter => {
            let Some(input) = state.workspace_input.take() else {
                return;
            };

            match input.mode {
                WorkspaceInputMode::Create => {
                    if let Some(id) = state.desktop.create_workspace(&input.value) {
                        state.window_manager.activate_workspace(id);
                    }
                }

                WorkspaceInputMode::Rename => {
                    state.desktop.rename_active_workspace(&input.value);
                }
            }
        }

        KeyCode::Backspace => {
            if let Some(input) = state.workspace_input.as_mut() {
                input.value.pop();
            }
        }

        KeyCode::Char(ch) if !ch.is_control() => {
            if let Some(input) = state.workspace_input.as_mut() {
                input.value.push(ch);
            }
        }

        _ => {}
    }
}
