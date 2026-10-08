use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

use super::common::{render_footer, render_title};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_title(frame, &app.chat_breadcrumb(), layout[0]);

    render_messages(frame, app, layout[1]);

    render_footer(frame, "Esc Back   q Quit", layout[2]);
}

fn render_messages(frame: &mut Frame, app: &App, area: Rect) {
    let lines = app
        .chat
        .messages
        .iter()
        .map(|message| {
            let mut line = String::new();

            if app.options.show_time {
                line.push_str(&format!("[{}] ", message.timestamp));
            }

            if app.options.show_alias {
                line.push_str(&format!("{}: ", message.sender));
            }

            line.push_str(&message.content);

            line
        })
        .collect::<Vec<_>>()
        .join("\n");

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title(" Obrolan ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
