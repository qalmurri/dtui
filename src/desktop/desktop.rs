use super::{panel::PanelState, status::DesktopStatus, workspace::Workspace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopState {
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub panel: PanelState,
    pub status: DesktopStatus,
}

impl Default for DesktopState {
    fn default() -> Self {
        Self {
            workspaces: vec![Workspace::new(0, "DEV"), Workspace::new(1, "TOOLS")],
            active_workspace: 0,
            panel: PanelState::default(),
            status: DesktopStatus::default(),
        }
    }
}

impl DesktopState {
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
}
