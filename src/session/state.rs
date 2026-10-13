use serde::{Deserialize, Serialize};

use crate::{
    desktop::workspace::Workspace,
    window_manager::{Geometry, Window},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionWindow {
    pub id: u64,
    pub title: String,
    pub geometry: Geometry,
    pub workspace_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionState {
    pub workspaces: Vec<Workspace>,
    pub active_workspace_id: usize,
    pub next_workspace_id: usize,
    pub windows: Vec<SessionWindow>,
    pub z_order: Vec<u64>,
    pub focused_window_id: Option<u64>,
    pub next_window_id: u64,
}

impl SessionState {
    pub fn window_from(window: &Window) -> SessionWindow {
        SessionWindow {
            id: window.id,
            title: window.title.clone(),
            geometry: window.geometry,
            workspace_id: window.workspace_id.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop::workspace::WorkspaceId;

    #[test]
    fn window_from_preserves_window_fields() {
        let window = Window::new(
            5,
            "Terminal",
            Geometry {
                x: 1,
                y: 2,
                width: 30,
                height: 10,
            },
            WorkspaceId(3),
        );

        let saved = SessionState::window_from(&window);

        assert_eq!(saved.id, 5);
        assert_eq!(saved.title, "Terminal");
        assert_eq!(saved.geometry, window.geometry);
        assert_eq!(saved.workspace_id, 3);
    }
}
