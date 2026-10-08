use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_title(frame: &mut Frame, title: &str, area: Rect) {
    let paragraph = Paragraph::new(title).block(Block::default().borders(Borders::ALL));

    frame.render_widget(paragraph, area);
}

pub fn render_footer(frame: &mut Frame, text: &str, area: Rect) {
    frame.render_widget(Paragraph::new(text), area);
}
