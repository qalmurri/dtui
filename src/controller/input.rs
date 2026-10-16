use crossterm::event::{KeyEvent, KeyEventKind};

use crate::{config::Keybindings, keybindings, keybindings::Action};

/// Resolve one keyboard event into one configured action.
///
/// Desktop shortcuts are press based. Repeats and releases do not produce
/// application actions, which keeps one key event independent from another.
pub fn resolve(key: KeyEvent, bindings: &Keybindings) -> Option<Action> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    keybindings::resolve(key, bindings)
}
