use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Back,
    ToggleHelp,
    None,
}

pub fn dispatch_key(key: KeyCode) -> Action {
    match key {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Esc => Action::Back,
        KeyCode::Char('?') => Action::ToggleHelp,
        _ => Action::None,
    }
}
