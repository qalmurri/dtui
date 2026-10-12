#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Focus {
    pub window_id: Option<u64>,
}

impl Focus {
    pub fn set(&mut self, window_id: Option<u64>) {
        self.window_id = window_id;
    }

    pub fn is_focused(&self, window_id: u64) -> bool {
        self.window_id == Some(window_id)
    }
}
