use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::{App, options::OptionItem};

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

    render_title(frame, "Options", layout[0]);

    let items = app
        .options
        .items
        .iter()
        .map(|item| {
            let text = match item {
                OptionItem::ShowAlias => {
                    if app.options.show_alias {
                        "[x] Tampilkan nama alias"
                    } else {
                        "[ ] Tampilkan nama alias"
                    }
                }

                OptionItem::ShowTime => {
                    if app.options.show_time {
                        "[x] Tampilkan waktu"
                    } else {
                        "[ ] Tampilkan waktu"
                    }
                }
            };

            ListItem::new(text)
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Chat Options ")
                .borders(Borders::ALL),
        )
        .highlight_symbol("> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut state = ListState::default();

    state.select(Some(app.options.selected));

    frame.render_stateful_widget(list, layout[1], &mut state);

    render_footer(frame, "↑ ↓ Navigate   Enter Toggle   Esc Back", layout[2]);
}
