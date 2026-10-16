use std::error::Error;

use crate::{
    application::action::Action,
    domain::window_manager::{Geometry, MAX_WINDOWS},
};

use super::{AppState, window_actions, workspace};

pub fn handle(
    state: &mut AppState,
    action: Action,
    desktop_width: u16,
    desktop_height: u16,
) -> Result<(), Box<dyn Error>> {
    let workspace_id = window_actions::active_workspace_id(state);

    match action {
        Action::Quit => state.running = false,

        Action::Help => state.show_help = true,

        Action::MoveLeft | Action::MoveDown | Action::MoveUp | Action::MoveRight => {
            window_actions::move_focused(state, desktop_width, desktop_height, action);
        }

        Action::ResizeLeft | Action::ResizeDown | Action::ResizeUp | Action::ResizeRight => {
            window_actions::resize_focused(state, desktop_width, desktop_height, action);
            return Ok(());
        }

        Action::FitLeft | Action::FitDown | Action::FitUp | Action::FitRight => {
            window_actions::fit_focused(state, desktop_width, desktop_height, action);
        }

        Action::FocusNext => {
            state.window_manager.focus_next(workspace_id);
        }

        Action::ToggleMode => {
            state.terminal_input_mode = !state.terminal_input_mode;
            state.desktop.status.message = if state.terminal_input_mode {
                "Terminal input mode".into()
            } else {
                "Desktop mode".into()
            };
        }

        Action::CloseWindow => {
            let focused_id = state
                .window_manager
                .focused()
                .filter(|window| window.workspace_id == workspace_id)
                .map(|window| window.id);

            if let Some(id) = focused_id {
                state.terminals.remove(id);
                state.window_manager.close(id);
            }
        }

        Action::NewWindow => {
            if state.window_manager.windows.len() >= MAX_WINDOWS {
                state.desktop.status.message = format!("Maksimal {MAX_WINDOWS} jendela terminal");
                return Ok(());
            }

            let count = state
                .window_manager
                .windows
                .iter()
                .filter(|window| window.workspace_id == workspace_id)
                .count();

            let geometry = Geometry::cascade(count, desktop_width, desktop_height);

            let id = state.window_manager.create(
                format!("Terminal {}", count + 1),
                geometry,
                workspace_id,
            );

            if let Err(error) = state.terminals.spawn_for_window(id, geometry) {
                state.window_manager.close(id);
                state.desktop.status.message = format!("Gagal membuat terminal: {error}");
                return Ok(());
            }

            state.terminal_input_mode = true;
            state.desktop.status.message = "Terminal baru dibuat".into();
        }

        Action::Workspace1 => workspace::select_index(state, 0),
        Action::Workspace2 => workspace::select_index(state, 1),
        Action::Workspace3 => workspace::select_index(state, 2),
        Action::Workspace4 => workspace::select_index(state, 3),
        Action::Workspace5 => workspace::select_index(state, 4),
        Action::Workspace6 => workspace::select_index(state, 5),
        Action::Workspace7 => workspace::select_index(state, 6),
        Action::Workspace8 => workspace::select_index(state, 7),
        Action::Workspace9 => workspace::select_index(state, 8),

        Action::WorkspacePrevious => workspace::select_previous(state),
        Action::WorkspaceNext => workspace::select_next(state),
        Action::WorkspaceCreate => workspace::begin_create(state),
        Action::WorkspaceRename => workspace::begin_rename(state),
        Action::WorkspaceDelete => workspace::delete_active(state),
    }

    Ok(())
}
