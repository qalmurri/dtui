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
mod tests {
    use super::ZOrder;

    #[test]
    fn insert_adds_window_to_top() {
        let mut z_order = ZOrder::default();

        z_order.insert(1);
        z_order.insert(2);

        assert_eq!(z_order.windows, vec![1, 2]);
    }

    #[test]
    fn insert_existing_window_does_not_duplicate_it() {
        let mut z_order = ZOrder::default();

        z_order.insert(1);
        z_order.insert(2);
        z_order.insert(1);

        assert_eq!(z_order.windows, vec![2, 1]);
    }

    #[test]
    fn raise_moves_window_to_top() {
        let mut z_order = ZOrder {
            windows: vec![1, 2, 3],
        };

        z_order.raise(1);

        assert_eq!(z_order.windows, vec![2, 3, 1]);
    }

    #[test]
    fn raise_unknown_window_does_nothing() {
        let mut z_order = ZOrder {
            windows: vec![1, 2],
        };

        z_order.raise(99);

        assert_eq!(z_order.windows, vec![1, 2]);
    }

    #[test]
    fn remove_deletes_window() {
        let mut z_order = ZOrder {
            windows: vec![1, 2, 3],
        };

        z_order.remove(2);

        assert_eq!(z_order.windows, vec![1, 3]);
    }
}
