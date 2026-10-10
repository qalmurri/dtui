use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::identity::Identity;

pub fn render_identity_panel(frame: &mut Frame<'_>, area: Rect, identity: &Identity) {
    let block = Block::default()
        .title(" IDENTITY ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let lines = vec![
        Line::styled(identity.alias(), Style::default().fg(Color::White)),
        Line::styled(
            identity.mode_label(),
            Style::default().fg(match identity.mode_label() {
                "Anonymous" => Color::Yellow,
                _ => Color::Green,
            }),
        ),
        Line::styled(identity.display_id(), Style::default().fg(Color::Gray)),
    ];

    let paragraph = Paragraph::new(lines).block(block);

    frame.render_widget(paragraph, area);
}
