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
    area_width: u16,
    area_height: u16,
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
    state
        .window_manager
        .move_focused(dx, dy, area_width, area_height, workspace_id);
}

pub(super) fn resize_focused(
    state: &mut AppState,
    area_width: u16,
    area_height: u16,
    action: Action,
) {
    let delta = match action {
        Action::ResizeLeft => ResizeDelta::left(),
        Action::ResizeDown => ResizeDelta::down(),
        Action::ResizeUp => ResizeDelta::up(),
        Action::ResizeRight => ResizeDelta::right(),
        _ => return,
    };

    let workspace_id = active_workspace_id(state);

    crate::domain::window_manager::resize::apply(
        &mut state.window_manager,
        delta,
        area_width,
        area_height,
        workspace_id,
    );

    if let Err(error) = resize_focused_terminal(state, area_width, area_height) {
        state.desktop.status.message =
            format!("Geometri berubah, tetapi resize terminal gagal: {error}");
    } else {
        state.desktop.status.message = "Ukuran window diperbarui".into();
    }
}

pub(super) fn fit_focused(state: &mut AppState, area_width: u16, area_height: u16, action: Action) {
    let direction = match action {
        Action::FitLeft => SnapDirection::Left,
        Action::FitDown => SnapDirection::Down,
        Action::FitUp => SnapDirection::Up,
        Action::FitRight => SnapDirection::Right,
        _ => return,
    };

    let workspace_id = active_workspace_id(state);

    state
        .window_manager
        .snap_focused(direction, area_width, area_height, workspace_id);

    if let Err(error) = resize_focused_terminal(state, area_width, area_height) {
        state.desktop.status.message =
            format!("Window berubah, tetapi resize terminal gagal: {error}");
    } else {
        state.desktop.status.message = "Window di-snap sesuai arah".into();
    }
}

fn resize_focused_terminal(
    state: &mut AppState,
    area_width: u16,
    area_height: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(window_id) = state.window_manager.focus.window_id else {
        return Ok(());
    };
    let Some(window) = state.window_manager.get(window_id) else {
        return Ok(());
    };

    state
        .terminals
        .resize_window(window_id, window.geometry.fit(area_width, area_height))
}
