use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{config::Keybindings, identity::Identity};

use super::components::{
    identity_panel::render_identity_panel, panel::render_panel, shortcut_bar::render_shortcut_bar,
};

pub fn render_dashboard(frame: &mut Frame<'_>, bindings: &Keybindings, identity: &Identity) {
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .split(area);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(vertical[0]);

    render_panel(frame, top[0], "TOP LEFT");
    render_panel(frame, top[1], "TOP CENTER");

    render_identity_panel(frame, top[2], identity);

    render_panel(frame, vertical[1], "MIDDLE");

    render_shortcut_bar(frame, vertical[2], bindings);
}
