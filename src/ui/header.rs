use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::AppState;

pub(super) fn render(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
    let mode = if state.terminal_input_mode {
        " TERM "
    } else {
        " DESK "
    };

    let mode_style = Style::default()
        .fg(Color::Black)
        .bg(Color::White)
        .add_modifier(Modifier::BOLD);

    let mut spans = vec![Span::styled(mode, mode_style), Span::raw(" ")];

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
