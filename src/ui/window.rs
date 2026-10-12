use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::window_manager::Window;

pub fn render(frame: &mut Frame<'_>, area: Rect, window: &Window, focused: bool) {
    let title = if focused {
        format!(" {} [Focused] ", window.title)
    } else {
        format!(" {} ", window.title)
    };

    let border_style = if focused {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let content = Paragraph::new(Line::raw("Internal DTUI window")).block(block);

    frame.render_widget(content, area);
}
