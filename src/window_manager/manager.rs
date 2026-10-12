use crate::desktop::workspace::WorkspaceId;

use super::{focus::Focus, geometry::Geometry, window::Window, z_order::ZOrder};

#[derive(Debug, Default)]
pub struct WindowManager {
    pub windows: Vec<Window>,
    pub focus: Focus,
    pub z_order: ZOrder,
    next_id: u64,
}

impl WindowManager {
    pub fn create(
        &mut self,
        title: impl Into<String>,
        geometry: Geometry,
        workspace_id: WorkspaceId,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.windows
            .push(Window::new(id, title, geometry, workspace_id));

        self.z_order.insert(id);
        self.focus.set(Some(id));

        id
    }

    pub fn close(&mut self, id: u64, workspace_id: WorkspaceId) {
        self.windows.retain(|window| window.id != id);
        self.z_order.remove(id);

        if self.focus.is_focused(id) {
            let next = self
                .z_order
                .windows
                .iter()
                .rev()
                .find(|window_id| {
                    self.get(**window_id)
                        .is_some_and(|window| window.workspace_id == workspace_id)
                })
                .copied();

            self.focus.set(next);
        }
    }

    pub fn focus_next(&mut self, workspace_id: WorkspaceId) {
        let visible_ids: Vec<u64> = self
            .z_order
            .windows
            .iter()
            .filter(|id| {
                self.get(**id)
                    .is_some_and(|window| window.workspace_id == workspace_id)
            })
            .copied()
            .collect();

        if visible_ids.is_empty() {
            self.focus.set(None);
            return;
        }

        let current_index = visible_ids
            .iter()
            .position(|id| Some(*id) == self.focus.window_id);

        let next_index = match current_index {
            Some(index) => (index + 1) % visible_ids.len(),
            None => 0,
        };

        let next_id = visible_ids[next_index];

        self.focus.set(Some(next_id));
        self.z_order.raise(next_id);
    }

    pub fn activate_workspace(&mut self, workspace_id: WorkspaceId) {
        let next = self
            .z_order
            .windows
            .iter()
            .rev()
            .find(|id| {
                self.get(**id)
                    .is_some_and(|window| window.workspace_id == workspace_id)
            })
            .copied();

        self.focus.set(next);
    }

    pub fn get(&self, id: u64) -> Option<&Window> {
        self.windows.iter().find(|window| window.id == id)
    }

    pub fn focused(&self) -> Option<&Window> {
        self.focus.window_id.and_then(|id| self.get(id))
    }
}
