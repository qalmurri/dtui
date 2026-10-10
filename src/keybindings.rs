use std::error::Error;

use crossterm::event::KeyCode;

use crate::config::Keybindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Back,
    NavigateUp,
    NavigateDown,
    Select,
}

pub fn parse_key(value: &str) -> Result<KeyCode, String> {
    let normalized = value.trim().to_lowercase();

    let key = match normalized.as_str() {
        "esc" | "escape" => KeyCode::Esc,
        "enter" | "return" => KeyCode::Enter,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "tab" => KeyCode::Tab,
        "backspace" => KeyCode::Backspace,
        "delete" => KeyCode::Delete,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "space" => KeyCode::Char(' '),
        _ => {
            let mut chars = value.chars();

            match (chars.next(), chars.next()) {
                (Some(ch), None) => KeyCode::Char(ch),
                _ => {
                    return Err(format!("Unsupported key name: '{value}'"));
                }
            }
        }
    };

    Ok(key)
}

pub fn validate(bindings: &Keybindings) -> Result<(), Box<dyn Error>> {
    let entries = [
        ("quit", bindings.quit.as_str()),
        ("back", bindings.back.as_str()),
        ("navigate_up", bindings.navigate_up.as_str()),
        ("navigate_down", bindings.navigate_down.as_str()),
        ("select", bindings.select.as_str()),
    ];

    let mut seen: Vec<(KeyCode, &str)> = Vec::new();

    for (action, value) in entries {
        let key =
            parse_key(value).map_err(|error| format!("Invalid keybinding '{action}': {error}"))?;

        if let Some((_, previous)) = seen.iter().find(|(existing, _)| existing == &key) {
            return Err(
                format!("Key '{value}' is assigned to both '{previous}' and '{action}'").into(),
            );
        }

        seen.push((key, action));
    }

    Ok(())
}

pub fn resolve(key: KeyCode, bindings: &Keybindings) -> Option<Action> {
    if parse_key(&bindings.quit).ok() == Some(key) {
        Some(Action::Quit)
    } else if parse_key(&bindings.back).ok() == Some(key) {
        Some(Action::Back)
    } else if parse_key(&bindings.navigate_up).ok() == Some(key) {
        Some(Action::NavigateUp)
    } else if parse_key(&bindings.navigate_down).ok() == Some(key) {
        Some(Action::NavigateDown)
    } else if parse_key(&bindings.select).ok() == Some(key) {
        Some(Action::Select)
    } else {
        None
    }
}

pub fn format_key(value: &str) -> String {
    match value.to_lowercase().as_str() {
        "esc" | "escape" => "Esc".into(),
        "enter" | "return" => "Enter".into(),
        "up" => "↑".into(),
        "down" => "↓".into(),
        "left" => "←".into(),
        "right" => "→".into(),
        "tab" => "Tab".into(),
        "backspace" => "Backspace".into(),
        "delete" => "Delete".into(),
        "home" => "Home".into(),
        "end" => "End".into(),
        "pageup" => "PageUp".into(),
        "pagedown" => "PageDown".into(),
        "space" => "Space".into(),
        _ => value.to_uppercase(),
    }
}
