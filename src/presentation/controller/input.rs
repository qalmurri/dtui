use crossterm::event::{KeyEvent, KeyEventKind};

use crate::application::{action::Action, settings::Keybindings};

use super::keybindings;

/// Resolve one keyboard event into one configured action.
///
/// Movement and resize shortcuts repeat while held. Other shortcuts only act
/// on the initial press to avoid repeating destructive or mode-changing actions.
pub fn resolve(key: KeyEvent, bindings: &Keybindings) -> Option<Action> {
    let action = keybindings::resolve(key, bindings)?;
    match key.kind {
        KeyEventKind::Press => Some(action),
        KeyEventKind::Repeat
            if matches!(
                action,
                Action::MoveLeft
                    | Action::MoveDown
                    | Action::MoveUp
                    | Action::MoveRight
                    | Action::ResizeLeft
                    | Action::ResizeDown
                    | Action::ResizeUp
                    | Action::ResizeRight
            ) =>
        {
            Some(action)
        }
        _ => None,
    }
}
