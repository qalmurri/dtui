use std::error::Error;

use crate::domain::{
    desktop::workspace::WorkspaceId,
    session::SessionState,
    window_manager::{Geometry, Window},
};

use super::AppState;

pub fn restore_or_initialize(
    state: &mut AppState,
    saved_session: Option<SessionState>,
    desktop_width: u16,
    desktop_height: u16,
) -> Result<(), Box<dyn Error>> {
    let restored = saved_session.is_some_and(|session| restore(state, session));

    if restored {
        let failed_windows = state
            .terminals
            .start_for_windows(&state.window_manager.windows);

        for (window_id, _) in &failed_windows {
            state.terminals.remove(*window_id);
            state.window_manager.close(*window_id);
        }

        if let Some((_, reason)) = failed_windows.first() {
            state.desktop.status.message = format!(
                "{} terminal sesi gagal dipulihkan; jendela gagal dilewati: {reason}",
                failed_windows.len()
            );
        }
    }

    if state.window_manager.windows.is_empty() {
        let workspace_id = state
            .desktop
            .active_workspace()
            .map(|workspace| workspace.id)
            .unwrap_or(WorkspaceId(0));

        let geometry = Geometry::cascade(0, desktop_width, desktop_height);
        let window_id = state
            .window_manager
            .create("Terminal", geometry, workspace_id);
        state.terminals.spawn_for_window(window_id, geometry)?;
    }

    Ok(())
}

fn restore(state: &mut AppState, session: SessionState) -> bool {
    let SessionState {
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

pub fn to_session_state(state: &AppState) -> SessionState {
    SessionState {
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
            .map(SessionState::window_from)
            .collect(),
        z_order: state.window_manager.z_order.windows.clone(),
        focused_window_id: state.window_manager.focus.window_id,
        next_window_id: state.window_manager.next_id(),
    }
}
