use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::AppState;

pub fn render(frame: &mut Frame<'_>, state: &AppState) {
    let area = frame.area();

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, sections[0], state);

    if state.show_help {
        render_help(frame, sections[1]);
    } else {
        render_desktop(frame, sections[1], state);
    }

    render_footer(frame, sections[2], state);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
    let workspace = state
        .desktop
        .active_workspace()
        .map(|workspace| workspace.name.as_str())
        .unwrap_or("NONE");

    let header = Paragraph::new(Line::from(vec![
        Span::styled(" DTUI ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("| "),
        Span::raw(workspace),
        Span::raw(" | 1:DEV  2:TOOLS"),
    ]));

    frame.render_widget(header, area);
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
            area.x + geometry.x,
            area.y + geometry.y,
            geometry.width,
            geometry.height,
        );

        crate::ui::window::render(
            frame,
            window_area,
            window,
            state.window_manager.focus.is_focused(window.id),
        );
    }
}

fn render_help(frame: &mut Frame<'_>, area: Rect) {
    let lines = vec![
        Line::raw(""),
        Line::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::raw("q           Quit DTUI"),
        Line::raw("?           Toggle shortcut help"),
        Line::raw("Tab         Next window"),
        Line::raw("H/J/K/L     Move focused window"),
        Line::raw("Alt + H     Decrease window width"),
        Line::raw("Alt + L     Increase window width"),
        Line::raw("Alt + K     Decrease window height"),
        Line::raw("Alt + J     Increase window height"),
        Line::raw("n           New window"),
        Line::raw("x           Close focused window"),
        Line::raw("1           Workspace DEV"),
        Line::raw("2           Workspace TOOLS"),
        Line::raw(""),
        Line::raw("Press any key to return."),
    ];

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::NONE)),
        area,
    );
}

fn render_footer(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
    let footer = Paragraph::new(Line::from(vec![
        Span::raw(" "),
        Span::raw(state.desktop.status.message.as_str()),
        Span::raw(" | HJKL: Move | Alt+HJKL: Resize | Tab: Focus | ?: Help | q: Quit "),
    ]));

    frame.render_widget(footer, area);
}
