#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanelState {
    pub visible: bool,
}

impl Default for PanelState {
    fn default() -> Self {
        Self { visible: true }
    }
}
