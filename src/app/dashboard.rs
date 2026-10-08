#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardItem {
    JoinPublic,
    JoinPrivate,
    CreateLobby,
    Options,
    Config,
}

pub struct DashboardState {
    pub items: Vec<DashboardItem>,
    pub selected: usize,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            items: vec![
                DashboardItem::JoinPublic,
                DashboardItem::JoinPrivate,
                DashboardItem::CreateLobby,
                DashboardItem::Options,
                DashboardItem::Config,
            ],
            selected: 0,
        }
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.items.len() {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn selected_item(&self) -> DashboardItem {
        self.items[self.selected]
    }
}
