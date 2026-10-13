use crossterm::event::KeyCode;

use super::{AppState, WorkspaceInput, WorkspaceInputMode};

pub fn select_index(state: &mut AppState, index: usize) {
    if let Some(workspace) = state.desktop.workspaces.get(index) {
        let id = workspace.id;
        state.desktop.select_workspace_id(id);
        state.window_manager.activate_workspace(id);
    }
}

pub fn select_previous(state: &mut AppState) {
    state.desktop.select_previous_workspace();
    activate_current(state);
}

pub fn select_next(state: &mut AppState) {
    state.desktop.select_next_workspace();
    activate_current(state);
}

pub fn begin_create(state: &mut AppState) {
    if let Some(id) = state.desktop.create_workspace_auto() {
        state.window_manager.activate_workspace(id);
    }
}

pub fn begin_rename(state: &mut AppState) {
    if let Some(workspace) = state.desktop.active_workspace() {
        state.workspace_input = Some(WorkspaceInput {
            mode: WorkspaceInputMode::Rename,
            value: workspace.name.clone(),
        });
    }
}

pub fn delete_active(state: &mut AppState) {
    if let Some((removed_id, target_id)) = state.desktop.remove_active_workspace() {
        for window in &mut state.window_manager.windows {
            if window.workspace_id == removed_id {
                window.workspace_id = target_id;
            }
        }

        state.window_manager.activate_workspace(target_id);
    }
}

pub fn handle_input(state: &mut AppState, key: KeyCode) {
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

fn activate_current(state: &mut AppState) {
    if let Some(workspace) = state.desktop.active_workspace() {
        state.window_manager.activate_workspace(workspace.id);
    }
}
