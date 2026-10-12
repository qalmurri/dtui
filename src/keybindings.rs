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
    let value = value.trim().to_lowercase();

    match value.as_str() {
        "esc" | "escape" => Ok(KeyCode::Esc),
        "enter" | "return" => Ok(KeyCode::Enter),
        "up" => Ok(KeyCode::Up),
        "down" => Ok(KeyCode::Down),
        "left" => Ok(KeyCode::Left),
        "right" => Ok(KeyCode::Right),
        "tab" => Ok(KeyCode::Tab),
        "space" => Ok(KeyCode::Char(' ')),
        "f1" => Ok(KeyCode::F(1)),
        "f2" => Ok(KeyCode::F(2)),
        _ => {
            let mut chars = value.chars();
            match (chars.next(), chars.next()) {
                (Some(ch), None) => Ok(KeyCode::Char(ch)),
                _ => Err(format!("Nama tombol tidak didukung: '{value}'")),
            }
        }
    }
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
        let key = parse_key(value)
            .map_err(|error| format!("Keybinding '{action}' tidak valid: {error}"))?;

        if let Some((_, previous)) = seen.iter().find(|(existing, _)| existing == &key) {
            return Err(
                format!("Tombol '{value}' dipakai oleh '{previous}' dan '{action}'").into(),
            );
        }
        seen.push((key, action));
    }

    Ok(())
}

pub fn resolve(key: KeyCode, bindings: &Keybindings) -> Option<Action> {
    let entries = [
        (&bindings.quit, Action::Quit),
        (&bindings.back, Action::Back),
        (&bindings.navigate_up, Action::NavigateUp),
        (&bindings.navigate_down, Action::NavigateDown),
        (&bindings.select, Action::Select),
    ];

    entries
        .into_iter()
        .find_map(|(binding, action)| (parse_key(binding).ok() == Some(key)).then_some(action))
}
