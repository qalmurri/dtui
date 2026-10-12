#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartMenuState {
    pub open: bool,
    pub selected: usize,
}
impl Default for StartMenuState {
    fn default() -> Self {
        Self {
            open: false,
            selected: 0,
        }
    }
}
impl StartMenuState {
    pub fn open(&mut self) {
        self.open = true;
        self.selected = 0;
    }
    pub fn close(&mut self) {
        self.open = false;
    }
    pub fn select(&mut self, index: usize, item_count: usize) {
        self.selected = if item_count == 0 {
            0
        } else {
            index.min(item_count - 1)
        };
    }
}
