use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::application::settings::Config;

pub(super) fn render(frame: &mut Frame<'_>, area: Rect, config: &Config) {
    let bindings = &config.keybindings;

    let lines = vec![
        Line::raw(""),
        Line::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        shortcut(
            &bindings.quit,
            "Quit tuicli (desktop mode; press F2 first in terminal mode)",
        ),
        shortcut(&bindings.help, "Toggle shortcut help"),
        shortcut(&bindings.focus_next, "Focus next window"),
        shortcut(
            &format!(
                "{}/{}/{}/{}",
                bindings.move_left, bindings.move_down, bindings.move_up, bindings.move_right
            ),
            "Move focused window (left/down/up/right)",
        ),
        shortcut(&bindings.resize_left, "Decrease window width"),
        shortcut(&bindings.resize_right, "Increase window width"),
        shortcut(&bindings.resize_up, "Decrease window height"),
        shortcut(&bindings.resize_down, "Increase window height"),
        shortcut(&bindings.fit_left, "Fit window to left half"),
        shortcut(&bindings.fit_right, "Fit window to right half"),
        shortcut(&bindings.fit_up, "Fit window to top half"),
        shortcut(&bindings.fit_down, "Fit window to bottom half"),
        shortcut(&bindings.new_window, "Create new window"),
        shortcut(&bindings.close_window, "Close focused window"),
        shortcut(&bindings.workspace_1, "Switch to first workspace"),
        shortcut(&bindings.workspace_2, "Switch to second workspace"),
        shortcut(&bindings.workspace_previous, "Previous workspace"),
        shortcut(&bindings.workspace_next, "Next workspace"),
        shortcut(&bindings.workspace_create, "Create workspace"),
        shortcut(&bindings.workspace_rename, "Rename workspace"),
        shortcut(&bindings.workspace_delete, "Delete workspace"),
        Line::raw(""),
        shortcut(
            &bindings.toggle_mode,
            "Toggle terminal input / desktop shortcuts",
        ),
        Line::raw(""),
        Line::styled(
            "Press any key to return",
            Style::default().add_modifier(Modifier::DIM),
        ),
    ];

    let popup_width = 76.min(area.width.saturating_sub(2));
    let popup_height = 28.min(area.height.saturating_sub(2));

    let popup_area = centered_rect(
        if area.width > 2 {
            popup_width.saturating_mul(100) / area.width
        } else {
            100
        },
        if area.height > 2 {
            popup_height.saturating_mul(100) / area.height
        } else {
            100
        },
        area,
    );

    frame.render_widget(Clear, area);

    let block = Block::default().title(" Help ").borders(Borders::ALL);
    let inner = block.inner(popup_area);

    frame.render_widget(block, popup_area);
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn shortcut(key: &str, description: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{key:<20}"),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(description.to_owned()),
    ])
}

pub(super) fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
