use crate::domain::desktop::workspace::WorkspaceId;

use super::{
    focus::Focus,
    geometry::{Geometry, SnapDirection},
    window::Window,
    z_order::ZOrder,
};

#[derive(Debug, Default)]
pub struct WindowManager {
    pub windows: Vec<Window>,
    pub focus: Focus,
    pub z_order: ZOrder,
    next_id: u64,
}

impl WindowManager {
    pub fn next_id(&self) -> u64 {
        self.next_id
    }

    pub fn restore_state(
        &mut self,
        windows: Vec<Window>,
        z_order: Vec<u64>,
        focused_window_id: Option<u64>,
        next_id: u64,
    ) {
        use std::collections::HashSet;

        let valid_ids: HashSet<u64> = windows.iter().map(|w| w.id).collect();

        self.windows = windows;
        self.z_order.windows.clear();
        for id in z_order {
            if valid_ids.contains(&id) && !self.z_order.windows.contains(&id) {
                self.z_order.windows.push(id);
            }
        }

        // Tambahkan jendela yang belum tercantum di z-order.
        for window in &self.windows {
            if !self.z_order.windows.contains(&window.id) {
                self.z_order.windows.push(window.id);
            }
        }

        self.focus
            .set(focused_window_id.filter(|id| valid_ids.contains(id)));

        self.next_id = next_id.max(
            self.windows
                .iter()
                .map(|w| w.id.saturating_add(1))
                .max()
                .unwrap_or(0),
        );
    }

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

    pub fn close(&mut self, id: u64) {
        let Some(workspace_id) = self.get(id).map(|window| window.workspace_id) else {
            return;
        };

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

    pub fn move_focused(
        &mut self,
        dx: i16,
        dy: i16,
        area_width: u16,
        area_height: u16,
        workspace_id: WorkspaceId,
    ) {
        let Some(id) = self.focus.window_id else {
            return;
        };

        let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.id == id && window.workspace_id == workspace_id)
        else {
            return;
        };

        let geometry = window.geometry.fit(area_width, area_height);

        let max_x = area_width.saturating_sub(geometry.width);
        let max_y = area_height.saturating_sub(geometry.height);

        let x = (i32::from(geometry.x) + i32::from(dx)).clamp(0, i32::from(max_x)) as u16;

        let y = (i32::from(geometry.y) + i32::from(dy)).clamp(0, i32::from(max_y)) as u16;

        window.geometry = Geometry {
            x,
            y,
            width: geometry.width,
            height: geometry.height,
        };
    }

    pub fn resize_focused(
        &mut self,
        dw: i16,
        dh: i16,
        area_width: u16,
        area_height: u16,
        workspace_id: WorkspaceId,
    ) {
        let Some(id) = self.focus.window_id else {
            return;
        };

        let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.id == id && window.workspace_id == workspace_id)
        else {
            return;
        };

        window.geometry = window.geometry.resize(dw, dh, area_width, area_height);
    }

    pub fn snap_focused(
        &mut self,
        direction: SnapDirection,
        area_width: u16,
        area_height: u16,
        workspace_id: WorkspaceId,
    ) {
        let Some(id) = self.focus.window_id else {
            return;
        };

        let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.id == id && window.workspace_id == workspace_id)
        else {
            return;
        };

        window.geometry = Geometry::snap(direction, area_width, area_height);
    }

    pub fn get(&self, id: u64) -> Option<&Window> {
        self.windows.iter().find(|window| window.id == id)
    }

    pub fn focused(&self) -> Option<&Window> {
        self.focus.window_id.and_then(|id| self.get(id))
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/window_manager/manager.rs"]
mod tests;
