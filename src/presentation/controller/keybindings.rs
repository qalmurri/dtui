use crate::application::{action::Action, settings::Keybindings};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::error::Error;

type ActionBinding = (&'static str, fn(&Keybindings) -> &str, Action);

const ACTIONS: &[ActionBinding] = &[
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
    ("fit_left", |b| &b.fit_left, Action::FitLeft),
    ("fit_down", |b| &b.fit_down, Action::FitDown),
    ("fit_up", |b| &b.fit_up, Action::FitUp),
    ("fit_right", |b| &b.fit_right, Action::FitRight),
    ("focus_next", |b| &b.focus_next, Action::FocusNext),
    ("toggle_mode", |b| &b.toggle_mode, Action::ToggleMode),
    ("close_window", |b| &b.close_window, Action::CloseWindow),
    ("new_window", |b| &b.new_window, Action::NewWindow),
    ("workspace_1", |b| &b.workspace_1, Action::Workspace1),
    ("workspace_2", |b| &b.workspace_2, Action::Workspace2),
    ("workspace_3", |b| &b.workspace_3, Action::Workspace3),
    ("workspace_4", |b| &b.workspace_4, Action::Workspace4),
    ("workspace_5", |b| &b.workspace_5, Action::Workspace5),
    ("workspace_6", |b| &b.workspace_6, Action::Workspace6),
    ("workspace_7", |b| &b.workspace_7, Action::Workspace7),
    ("workspace_8", |b| &b.workspace_8, Action::Workspace8),
    ("workspace_9", |b| &b.workspace_9, Action::Workspace9),
    (
        "workspace_previous",
        |b| &b.workspace_previous,
        Action::WorkspacePrevious,
    ),
    (
        "workspace_next",
        |b| &b.workspace_next,
        Action::WorkspaceNext,
    ),
    (
        "workspace_create",
        |b| &b.workspace_create,
        Action::WorkspaceCreate,
    ),
    (
        "workspace_rename",
        |b| &b.workspace_rename,
        Action::WorkspaceRename,
    ),
    (
        "workspace_delete",
        |b| &b.workspace_delete,
        Action::WorkspaceDelete,
    ),
];

/// Parsed once during startup so resolving each input event does not parse or
/// allocate every configured shortcut again.
#[derive(Debug, Clone)]
pub struct Keymap {
    entries: Vec<(KeyCode, KeyModifiers, Action)>,
}

impl Keymap {
    pub fn compile(bindings: &Keybindings) -> Result<Self, Box<dyn Error>> {
        let mut entries: Vec<(KeyCode, KeyModifiers, Action)> = Vec::with_capacity(ACTIONS.len());

        for (name, get_binding, action) in ACTIONS {
            let value = get_binding(bindings);
            let parsed = parse_key(value)
                .map_err(|error| format!("Keybinding '{name}' tidak valid: {error}"))?;

            if let Some((_, _, previous_action)) = entries
                .iter()
                .find(|(code, modifiers, _)| (*code, *modifiers) == parsed)
            {
                let previous = ACTIONS
                    .iter()
                    .find(|(_, _, candidate)| candidate == previous_action)
                    .map(|(previous, _, _)| *previous)
                    .unwrap_or("unknown");
                return Err(
                    format!("Binding '{value}' bentrok antara '{previous}' dan '{name}'").into(),
                );
            }

            entries.push((parsed.0, parsed.1, *action));
        }

        Ok(Self { entries })
    }

    pub fn resolve(&self, key: KeyEvent) -> Option<Action> {
        let code = match key.code {
            KeyCode::Char(ch) if ch.is_ascii_alphabetic() => KeyCode::Char(ch.to_ascii_lowercase()),
            code => code,
        };
        let modifiers = relevant_modifiers(key.modifiers);
        self.entries
            .iter()
            .find_map(|(candidate, required, action)| {
                (*candidate == code && *required == modifiers).then_some(*action)
            })
    }
}

pub fn parse_key(value: &str) -> Result<(KeyCode, KeyModifiers), String> {
    let value = value.trim().to_lowercase();
    let mut modifiers = KeyModifiers::NONE;
    let mut key = None;
    for part in value.split('+') {
        let modifier = match part {
            "alt" => Some(KeyModifiers::ALT),
            "ctrl" | "control" => Some(KeyModifiers::CONTROL),
            "shift" => Some(KeyModifiers::SHIFT),
            "win" | "super" => Some(KeyModifiers::SUPER),
            "hyper" => Some(KeyModifiers::HYPER),
            "meta" => Some(KeyModifiers::META),
            _ => None,
        };

        if let Some(modifier) = modifier {
            if modifiers.contains(modifier) {
                return Err(format!("Modifier '{part}' ditulis lebih dari sekali"));
            }
            modifiers.insert(modifier);
        } else if key.replace(part).is_some() {
            return Err(format!("Terlalu banyak tombol dalam binding '{value}'"));
        }
    }
    let key = key.ok_or_else(|| format!("Binding '{value}' tidak memiliki tombol"))?;

    let code = match key {
        "esc" | "escape" => KeyCode::Esc,
        "enter" | "return" => KeyCode::Enter,
        "backspace" => KeyCode::Backspace,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" | "page_up" => KeyCode::PageUp,
        "pagedown" | "page_down" => KeyCode::PageDown,
        "delete" | "del" => KeyCode::Delete,
        "insert" | "ins" => KeyCode::Insert,
        "tab" => KeyCode::Tab,
        "backtab" => KeyCode::BackTab,
        "space" => KeyCode::Char(' '),
        _ if key.starts_with('f')
            && key[1..]
                .parse::<u8>()
                .is_ok_and(|number| (1..=12).contains(&number)) =>
        {
            KeyCode::F(key[1..].parse().expect("validated function key"))
        }
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
    Keymap::compile(bindings).map(|_| ())
}

pub fn resolve(key: KeyEvent, keymap: &Keymap) -> Option<Action> {
    keymap.resolve(key)
}

/// Ignore lock-state flags that terminals may attach to otherwise identical keys.
fn relevant_modifiers(modifiers: KeyModifiers) -> KeyModifiers {
    modifiers
        & (KeyModifiers::SHIFT
            | KeyModifiers::ALT
            | KeyModifiers::CONTROL
            | KeyModifiers::SUPER
            | KeyModifiers::HYPER
            | KeyModifiers::META)
}
