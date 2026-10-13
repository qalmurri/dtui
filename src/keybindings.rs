use crate::config::Keybindings;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Help,
    MoveLeft,
    MoveDown,
    MoveUp,
    MoveRight,
    ResizeLeft,
    ResizeDown,
    ResizeUp,
    ResizeRight,
    FocusNext,
    CloseWindow,
    NewWindow,
    Workspace1,
    Workspace2,
}

const ACTIONS: &[(&str, fn(&Keybindings) -> &str, Action)] = &[
    ("quit", |b| &b.quit, Action::Quit),
    ("help", |b| &b.help, Action::Help),
    ("move_left", |b| &b.move_left, Action::MoveLeft),
    ("move_down", |b| &b.move_down, Action::MoveDown),
    ("move_up", |b| &b.move_up, Action::MoveUp),
    ("move_right", |b| &b.move_right, Action::MoveRight),
    ("resize_left", |b| &b.resize_left, Action::ResizeLeft),
    ("resize_down", |b| &b.resize_down, Action::ResizeDown),
    ("resize_up", |b| &b.resize_up, Action::ResizeUp),
    ("resize_right", |b| &b.resize_right, Action::ResizeRight),
    ("focus_next", |b| &b.focus_next, Action::FocusNext),
    ("close_window", |b| &b.close_window, Action::CloseWindow),
    ("new_window", |b| &b.new_window, Action::NewWindow),
    ("workspace_1", |b| &b.workspace_1, Action::Workspace1),
    ("workspace_2", |b| &b.workspace_2, Action::Workspace2),
];

pub fn parse_key(value: &str) -> Result<(KeyCode, KeyModifiers), String> {
    let value = value.trim().to_lowercase();
    let (modifiers, key) = if let Some(key) = value.strip_prefix("alt+") {
        (KeyModifiers::ALT, key)
    } else {
        (KeyModifiers::NONE, value.as_str())
    };

    let code = match key {
        "esc" | "escape" => KeyCode::Esc,
        "enter" | "return" => KeyCode::Enter,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "tab" => KeyCode::Tab,
        "space" => KeyCode::Char(' '),
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        _ => {
            let mut chars = key.chars();
            match (chars.next(), chars.next()) {
                (Some(ch), None) => KeyCode::Char(ch),
                _ => return Err(format!("Nama tombol tidak didukung: '{value}'")),
            }
        }
    };

    Ok((code, modifiers))
}

pub fn validate(bindings: &Keybindings) -> Result<(), Box<dyn Error>> {
    let mut seen: Vec<((KeyCode, KeyModifiers), &str)> = Vec::new();

    for (name, get_binding, _) in ACTIONS {
        let value = get_binding(bindings);
        let parsed = parse_key(value)
            .map_err(|error| format!("Keybinding '{name}' tidak valid: {error}"))?;

        if let Some((_, previous)) = seen.iter().find(|(key, _)| key == &parsed) {
            return Err(
                format!("Binding '{value}' bentrok antara '{previous}' dan '{name}'").into(),
            );
        }

        seen.push((parsed, name));
    }

    Ok(())
}

pub fn resolve(key: KeyEvent, bindings: &Keybindings) -> Option<Action> {
    ACTIONS.iter().find_map(|(_, get_binding, action)| {
        let (code, modifiers) = parse_key(get_binding(bindings)).ok()?;

        (key.code == code && key.modifiers == modifiers).then_some(*action)
    })
}
