#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionItem {
    ShowAlias,
    ShowTime,
}

pub struct OptionsState {
    pub items: Vec<OptionItem>,
    pub selected: usize,

    pub show_alias: bool,
    pub show_time: bool,
}

impl OptionsState {
    pub fn new() -> Self {
        Self {
            items: vec![OptionItem::ShowAlias, OptionItem::ShowTime],

            selected: 0,

            show_alias: true,
            show_time: true,
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

    pub fn toggle(&mut self) {
        match self.items[self.selected] {
            OptionItem::ShowAlias => {
                self.show_alias = !self.show_alias;
            }

            OptionItem::ShowTime => {
                self.show_time = !self.show_time;
            }
        }
    }
}
