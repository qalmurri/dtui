#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ZOrder {
    pub windows: Vec<u64>,
}

impl ZOrder {
    pub fn insert(&mut self, id: u64) {
        self.remove(id);
        self.windows.push(id);
    }

    pub fn remove(&mut self, id: u64) {
        self.windows.retain(|window_id| *window_id != id);
    }

    pub fn raise(&mut self, id: u64) {
        if self.windows.contains(&id) {
            self.windows.retain(|window_id| *window_id != id);
            self.windows.push(id);
        }
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/window_manager/z_order.rs"]
mod tests;
