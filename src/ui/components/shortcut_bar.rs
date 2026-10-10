use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{config::Keybindings, keybindings::format_key};

pub fn render_shortcut_bar(frame: &mut Frame<'_>, area: Rect, bindings: &Keybindings) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let navigation = format!(
        "{}{}",
        format_key(&bindings.navigate_up),
        format_key(&bindings.navigate_down),
    );

    let left = Paragraph::new(Line::from(vec![
        shortcut(format_key(&bindings.quit)),
        Span::raw(" Quit  "),
        shortcut(format_key(&bindings.back)),
        Span::raw(" Back"),
    ]));

    let right = Paragraph::new(Line::from(vec![
        shortcut(navigation),
        Span::raw(" Navigate  "),
        shortcut(format_key(&bindings.select)),
        Span::raw(" Select"),
    ]))
    .alignment(Alignment::Right);

    frame.render_widget(left, columns[0]);
    frame.render_widget(right, columns[1]);
}

fn shortcut(key: String) -> Span<'static> {
    Span::styled(
        format!(" {key} "),
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
}
