use std::error::Error;

use crate::{
    desktop::workspace::WorkspaceId,
    window_manager::{Geometry, Window},
};

use super::AppState;

pub fn restore_or_initialize(
    state: &mut AppState,
    saved_session: Option<crate::session::SessionState>,
    terminal_width: u16,
    desktop_height: u16,
) {
    let restored = saved_session.is_some_and(|session| restore(state, session));

    if !restored {
        let workspace_id = state
            .desktop
            .active_workspace()
            .map(|workspace| workspace.id)
            .unwrap_or(WorkspaceId(0));

        state.window_manager.create(
            "Terminal",
            Geometry::cascade(0, terminal_width, desktop_height),
            workspace_id,
        );
    }
}

fn restore(state: &mut AppState, session: crate::session::SessionState) -> bool {
    let crate::session::SessionState {
        workspaces,
        active_workspace_id,
        next_workspace_id,
        windows,
        z_order,
        focused_window_id,
        next_window_id,
    } = session;

    let active_workspace = WorkspaceId(active_workspace_id);
    if !state
        .desktop
        .restore_workspaces(workspaces, active_workspace, next_workspace_id)
    {
        return false;
    }

    let windows = windows
        .into_iter()
        .map(|window| {
            Window::new(
                window.id,
                window.title,
                window.geometry,
                WorkspaceId(window.workspace_id),
            )
        })
        .collect();

    state
        .window_manager
        .restore_state(windows, z_order, focused_window_id, next_window_id);

    true
}

pub fn save(state: &AppState) -> Result<(), Box<dyn Error>> {
    let session = crate::session::SessionState {
        workspaces: state.desktop.workspaces.clone(),
        active_workspace_id: state
            .desktop
            .active_workspace()
            .map(|workspace| workspace.id.0)
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

    session.save()
}
