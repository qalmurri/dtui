use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{app::AppState, config::Config};

pub fn render(frame: &mut Frame<'_>, state: &AppState, config: &Config) {
    let area = frame.area();

    // Help is a full-screen view. Do not render the desktop,
    // workspace header, or mode footer behind it.
    if state.show_help {
        render_help(frame, area, config);
        return;
    }

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, sections[0], state);

    if let Some(input) = &state.workspace_input {
        let title = match input.mode {
            crate::app::WorkspaceInputMode::Rename => "Rename Workspace",
        };

        let dialog = Paragraph::new(format!(
            "{title}\n\nName: {}_\n\nEnter: save | Esc: cancel",
            input.value
        ))
        .block(Block::default().borders(Borders::ALL).title(title));

        let dialog_area = centered_rect(60, 30, sections[1]);
        frame.render_widget(dialog, dialog_area);
    } else {
        render_desktop(frame, sections[1], state);
    }

    render_footer(frame, sections[2], state);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
    let mut spans = Vec::new();

    for (index, workspace) in state.desktop.workspaces.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw("  "));
        }

        let label = format!("[{}] {}", index + 1, workspace.name);

        if index == state.desktop.active_workspace {
            spans.push(Span::styled(
                label,
                Style::default().add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::raw(label));
        }
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_desktop(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
    frame.render_widget(Block::default().borders(Borders::NONE), area);

    let Some(workspace) = state.desktop.active_workspace() else {
        return;
    };

    for id in &state.window_manager.z_order.windows {
        let Some(window) = state.window_manager.get(*id) else {
            continue;
        };

        if window.workspace_id != workspace.id {
            continue;
        }

        let geometry = window.geometry.fit(area.width, area.height);

        let window_area = Rect::new(
            area.x.saturating_add(geometry.x),
            area.y.saturating_add(geometry.y),
            geometry.width,
            geometry.height,
        );

        if window_area.is_empty() {
            continue;
        }

        crate::ui::window::render(
            frame,
            window_area,
            window,
            state.window_manager.focus.is_focused(window.id),
            state.terminals.screen(window.id),
            state.terminals.is_exited(window.id),
            state.terminal_input_mode && state.window_manager.focus.is_focused(window.id),
        );
    }
}

fn render_help(frame: &mut Frame<'_>, area: Rect, config: &Config) {
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
        shortcut("F2", "Toggle terminal input / desktop shortcuts"),
        Line::raw(""),
        Line::styled(
            "Press any key to return",
            Style::default().add_modifier(Modifier::DIM),
        ),
    ];

    let popup_width = 76.min(area.width.saturating_sub(2));
    let popup_height = 24.min(area.height.saturating_sub(2));

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

    // Clear the screen so no desktop content remains visible.
    frame.render_widget(ratatui::widgets::Clear, area);

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

fn render_footer(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
    let mode = if state.terminal_input_mode {
        "Terminal mode"
    } else {
        "Desktop mode"
    };

    frame.render_widget(Paragraph::new(mode), area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
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
