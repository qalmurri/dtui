use super::{
    panel::PanelState,
    status::DesktopStatus,
    workspace::{Workspace, WorkspaceId},
};

pub const MAX_WORKSPACES: usize = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopState {
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub panel: PanelState,
    pub status: DesktopStatus,
    next_workspace_id: usize,
}

impl Default for DesktopState {
    fn default() -> Self {
        Self {
            workspaces: vec![Workspace::new(0, "MAIN")],
            active_workspace: 0,
            panel: PanelState::default(),
            status: DesktopStatus::default(),
            next_workspace_id: 1,
        }
    }
}

impl DesktopState {
    pub fn next_workspace_id(&self) -> usize {
        self.next_workspace_id
    }

    pub fn restore_workspaces(
        &mut self,
        workspaces: Vec<Workspace>,
        active_id: WorkspaceId,
        next_id: usize,
    ) -> bool {
        if workspaces.is_empty() || workspaces.len() > MAX_WORKSPACES {
            return false;
        }

        let Some(index) = workspaces.iter().position(|w| w.id == active_id) else {
            return false;
        };

        let max_next_id = workspaces
            .iter()
            .map(|w| w.id.0.saturating_add(1))
            .max()
            .unwrap_or(0);

        self.workspaces = workspaces;
        self.active_workspace = index;
        self.next_workspace_id = next_id.max(max_next_id);

        true
    }

    pub fn active_workspace(&self) -> Option<&Workspace> {
        self.workspaces.get(self.active_workspace)
    }

    pub fn select_workspace(&mut self, index: usize) {
        let Some(name) = self.workspaces.get(index).map(|w| w.name.clone()) else {
            return;
        };

        self.active_workspace = index;
        self.status.message = format!("Workspace: {name}");
    }

    pub fn select_workspace_id(&mut self, id: WorkspaceId) -> bool {
        let Some(index) = self.workspaces.iter().position(|w| w.id == id) else {
            return false;
        };

        self.select_workspace(index);
        true
    }

    pub fn create_workspace(&mut self, name: &str) -> Option<WorkspaceId> {
        if self.workspaces.len() >= MAX_WORKSPACES {
            self.status.message = format!("Maksimal {MAX_WORKSPACES} workspace");
            return None;
        }

        let name = name.trim();

        if name.is_empty() {
            self.status.message = "Nama workspace tidak boleh kosong".into();
            return None;
        }

        if self.workspaces.iter().any(|w| w.name == name) {
            self.status.message = "Nama workspace sudah digunakan".into();
            return None;
        }

        let id = WorkspaceId(self.next_workspace_id);
        self.next_workspace_id = self.next_workspace_id.saturating_add(1);

        self.workspaces.push(Workspace {
            id,
            name: name.to_owned(),
        });

        self.active_workspace = self.workspaces.len() - 1;
        self.status.message = format!("Workspace dibuat: {name}");

        Some(id)
    }

    pub fn create_workspace_auto(&mut self) -> Option<WorkspaceId> {
        if self.workspaces.len() >= MAX_WORKSPACES {
            self.status.message = format!("Maksimal {MAX_WORKSPACES} workspace");
            return None;
        }

        let mut number = 2;

        let name = loop {
            let candidate = format!("WORKSPACE {number}");

            if !self.workspaces.iter().any(|w| w.name == candidate) {
                break candidate;
            }

            number += 1;
        };

        self.create_workspace(&name)
    }

    pub fn rename_active_workspace(&mut self, name: &str) -> bool {
        let name = name.trim();

        if name.is_empty() {
            self.status.message = "Nama workspace tidak boleh kosong".into();
            return false;
        }

        let Some(current) = self.active_workspace() else {
            return false;
        };

        if self
            .workspaces
            .iter()
            .any(|w| w.name == name && w.id != current.id)
        {
            self.status.message = "Nama workspace sudah digunakan".into();
            return false;
        }

        let Some(workspace) = self.workspaces.get_mut(self.active_workspace) else {
            return false;
        };

        workspace.name = name.to_owned();
        self.status.message = format!("Workspace diubah menjadi: {name}");

        true
    }

    /// Menghapus workspace aktif dan mengembalikan
    /// (ID workspace yang dihapus, ID workspace tujuan).
    ///
    /// Workspace terakhir tidak boleh dihapus.
    pub fn remove_active_workspace(&mut self) -> Option<(WorkspaceId, WorkspaceId)> {
        if self.workspaces.len() <= 1 {
            self.status.message = "Minimal satu workspace harus tersedia".into();
            return None;
        }

        let removed = self.workspaces.remove(self.active_workspace);

        self.active_workspace = self.active_workspace.min(self.workspaces.len() - 1);

        let target = self.workspaces.get(self.active_workspace)?.id;

        self.status.message = format!("Workspace {} dihapus", removed.name);

        Some((removed.id, target))
    }

    pub fn select_next_workspace(&mut self) {
        if self.workspaces.is_empty() {
            return;
        }

        let next = (self.active_workspace + 1) % self.workspaces.len();
        self.select_workspace(next);
    }

    pub fn select_previous_workspace(&mut self) {
        if self.workspaces.is_empty() {
            return;
        }

        let previous = if self.active_workspace == 0 {
            self.workspaces.len() - 1
        } else {
            self.active_workspace - 1
        };

        self.select_workspace(previous);
    }
}

#[cfg(test)]
mod tests {
    use super::{DesktopState, MAX_WORKSPACES};

    #[test]
    fn starts_with_one_main_workspace() {
        let desktop = DesktopState::default();

        assert_eq!(desktop.workspaces.len(), 1);
        assert_eq!(desktop.active_workspace().unwrap().name, "MAIN");
        assert_eq!(desktop.next_workspace_id(), 1);
    }

    #[test]
    fn creates_workspace_and_activates_it() {
        let mut desktop = DesktopState::default();

        let id = desktop.create_workspace("GAMES").unwrap();

        assert_eq!(desktop.active_workspace().unwrap().name, "GAMES");
        assert_eq!(desktop.active_workspace().unwrap().id, id);
        assert_eq!(desktop.workspaces.len(), 2);
    }

    #[test]
    fn creates_workspace_with_automatic_name() {
        let mut desktop = DesktopState::default();

        let id = desktop.create_workspace_auto().unwrap();

        assert_eq!(desktop.active_workspace().unwrap().name, "WORKSPACE 2");
        assert_eq!(desktop.active_workspace().unwrap().id, id);
        assert_eq!(desktop.workspaces.len(), 2);
    }

    #[test]
    fn automatic_name_skips_existing_names() {
        let mut desktop = DesktopState::default();

        desktop.create_workspace("WORKSPACE 2").unwrap();
        desktop.select_workspace(0);

        let id = desktop.create_workspace_auto().unwrap();

        assert_eq!(desktop.active_workspace().unwrap().id, id);
        assert_eq!(desktop.active_workspace().unwrap().name, "WORKSPACE 3");
    }

    #[test]
    fn rejects_duplicate_workspace_names() {
        let mut desktop = DesktopState::default();

        assert!(desktop.create_workspace("MAIN").is_none());
        assert_eq!(desktop.workspaces.len(), 1);
    }

    #[test]
    fn renames_active_workspace() {
        let mut desktop = DesktopState::default();

        assert!(desktop.rename_active_workspace("CODE"));
        assert_eq!(desktop.active_workspace().unwrap().name, "CODE");
    }

    #[test]
    fn cannot_remove_last_workspace() {
        let mut desktop = DesktopState::default();

        assert!(desktop.remove_active_workspace().is_none());
        assert_eq!(desktop.workspaces.len(), 1);
    }

    #[test]
    fn workspace_ids_are_not_reused() {
        let mut desktop = DesktopState::default();

        let first = desktop.create_workspace("GAMES").unwrap();
        desktop.remove_active_workspace();
        let second = desktop.create_workspace("MUSIC").unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn navigation_wraps_around() {
        let mut desktop = DesktopState::default();

        desktop.create_workspace("DEV").unwrap();

        desktop.select_previous_workspace();
        assert_eq!(desktop.active_workspace().unwrap().name, "MAIN");

        desktop.select_next_workspace();
        assert_eq!(desktop.active_workspace().unwrap().name, "DEV");

        desktop.select_next_workspace();
        assert_eq!(desktop.active_workspace().unwrap().name, "MAIN");

        desktop.select_previous_workspace();
        assert_eq!(desktop.active_workspace().unwrap().name, "DEV");
    }

    #[test]
    fn rejects_workspace_creation_above_limit() {
        let mut desktop = DesktopState::default();

        for number in 2..=MAX_WORKSPACES {
            desktop
                .create_workspace(&format!("WORKSPACE {number}"))
                .unwrap();
        }

        assert_eq!(desktop.workspaces.len(), MAX_WORKSPACES);
        assert!(desktop.create_workspace_auto().is_none());
        assert!(desktop.create_workspace("EXTRA").is_none());
        assert_eq!(desktop.workspaces.len(), MAX_WORKSPACES);
    }

    #[test]
    fn rejects_restoring_more_than_nine_workspaces() {
        use crate::desktop::workspace::{Workspace, WorkspaceId};

        let mut desktop = DesktopState::default();

        let workspaces = (0..10)
            .map(|id| Workspace::new(id, format!("WORKSPACE {id}")))
            .collect();

        assert!(!desktop.restore_workspaces(workspaces, WorkspaceId(0), 10));
        assert_eq!(desktop.workspaces.len(), 1);
    }

    #[test]
    fn restores_workspaces_and_active_workspace() {
        use crate::desktop::workspace::{Workspace, WorkspaceId};

        let mut desktop = DesktopState::default();
        let restored = vec![Workspace::new(0, "MAIN"), Workspace::new(4, "TOOLS")];

        assert!(desktop.restore_workspaces(restored, WorkspaceId(4), 5));
        assert_eq!(desktop.active_workspace().unwrap().name, "TOOLS");
        assert_eq!(desktop.next_workspace_id(), 5);
    }
}
