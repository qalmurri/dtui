use std::error::Error;

use crate::{Tui, desktop::workspace::WorkspaceId, keybindings::Action, window_manager::Geometry};

use super::{AppState, workspace};

pub fn handle(state: &mut AppState, action: Action, terminal: &Tui) -> Result<(), Box<dyn Error>> {
    let workspace_id = state
        .desktop
        .active_workspace()
        .map(|workspace| workspace.id)
        .unwrap_or(WorkspaceId(0));

    match action {
        Action::Quit => state.running = false,
        Action::Help => state.show_help = true,

        Action::MoveLeft => move_focused_window(state, terminal, -2, 0, workspace_id)?,
        Action::MoveDown => move_focused_window(state, terminal, 0, 1, workspace_id)?,
        Action::MoveUp => move_focused_window(state, terminal, 0, -1, workspace_id)?,
        Action::MoveRight => move_focused_window(state, terminal, 2, 0, workspace_id)?,

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
                size.height.saturating_sub(1),
                workspace_id,
            );
        }

        Action::FocusNext => state.window_manager.focus_next(workspace_id),

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
            let size = terminal.size()?;

            let count = state
                .window_manager
                .windows
                .iter()
                .filter(|window| window.workspace_id == workspace_id)
                .count();

            let geometry = Geometry::cascade(count, size.width, size.height.saturating_sub(1));

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

    let size = terminal.size()?;
    let windows = state.window_manager.windows.clone();

    if let Err(error) =
        state
            .terminals
            .resize_all(&windows, size.width, size.height.saturating_sub(1))
    {
        state.desktop.status.message = format!("Gagal resize terminal: {error}");
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
    let size = terminal.size()?;

    state.window_manager.move_focused(
        dx,
        dy,
        size.width,
        size.height.saturating_sub(1),
        workspace_id,
    );

    Ok(())
}
