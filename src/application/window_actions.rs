use crate::{
    application::action::Action,
    domain::{
        desktop::workspace::WorkspaceId,
        window_manager::{geometry::SnapDirection, resize::ResizeDelta},
    },
};

use super::AppState;

pub(super) fn active_workspace_id(state: &AppState) -> WorkspaceId {
    state
        .desktop
        .active_workspace()
        .map(|workspace| workspace.id)
        .unwrap_or(WorkspaceId(0))
}

pub(super) fn move_focused(
    state: &mut AppState,
    terminal_width: u16,
    terminal_height: u16,
    action: Action,
) {
    let (dx, dy) = match action {
        Action::MoveLeft => (-2, 0),
        Action::MoveDown => (0, 1),
        Action::MoveUp => (0, -1),
        Action::MoveRight => (2, 0),
        _ => return,
    };

    let workspace_id = active_workspace_id(state);
    state.window_manager.move_focused(
        dx,
        dy,
        terminal_width,
        terminal_height.saturating_sub(1),
        workspace_id,
    );
}

pub(super) fn resize_focused(
    state: &mut AppState,
    terminal_width: u16,
    terminal_height: u16,
    action: Action,
) {
    let delta = match action {
        Action::ResizeLeft => ResizeDelta::left(),
        Action::ResizeDown => ResizeDelta::down(),
        Action::ResizeUp => ResizeDelta::up(),
        Action::ResizeRight => ResizeDelta::right(),
        _ => return,
    };

    let desktop_height = terminal_height.saturating_sub(1);
    let workspace_id = active_workspace_id(state);

    crate::domain::window_manager::resize::apply(
        &mut state.window_manager,
        delta,
        terminal_width,
        desktop_height,
        workspace_id,
    );

    let windows = state.window_manager.windows.clone();
    if let Err(error) = state
        .terminals
        .resize_all(&windows, terminal_width, desktop_height)
    {
        state.desktop.status.message =
            format!("Geometri berubah, tetapi resize terminal gagal: {error}");
    } else {
        state.desktop.status.message = "Ukuran window diperbarui".into();
    }
}

pub(super) fn fit_focused(
    state: &mut AppState,
    terminal_width: u16,
    terminal_height: u16,
    action: Action,
) {
    let direction = match action {
        Action::FitLeft => SnapDirection::Left,
        Action::FitDown => SnapDirection::Down,
        Action::FitUp => SnapDirection::Up,
        Action::FitRight => SnapDirection::Right,
        _ => return,
    };

    let desktop_height = terminal_height.saturating_sub(1);
    let workspace_id = active_workspace_id(state);

    state
        .window_manager
        .snap_focused(direction, terminal_width, desktop_height, workspace_id);

    let windows = state.window_manager.windows.clone();
    if let Err(error) = state
        .terminals
        .resize_all(&windows, terminal_width, desktop_height)
    {
        state.desktop.status.message =
            format!("Window berubah, tetapi resize terminal gagal: {error}");
    } else {
        state.desktop.status.message = "Window di-snap sesuai arah".into();
    }
}
