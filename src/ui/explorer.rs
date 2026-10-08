use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::{App, explorer::ExplorerNode};

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

    render_title(frame, &app.explorer.breadcrumb(), layout[0]);

    render_tree(frame, app, layout[1]);

    render_footer(
        frame,
        "↑ ↓ Navigate   Enter Open   Esc Back   q Quit",
        layout[2],
    );
}

fn render_tree(frame: &mut Frame, app: &App, area: Rect) {
    let items = app
        .explorer
        .current_nodes()
        .iter()
        .map(|node| match node {
            ExplorerNode::Folder { name, .. } => ListItem::new(format!("📁 {}", name)),

            ExplorerNode::Chat { name } => ListItem::new(format!("💬 {}", name)),
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(Block::default().title(" Explorer ").borders(Borders::ALL))
        .highlight_symbol("> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut state = ListState::default();

    if !app.explorer.current_nodes().is_empty() {
        state.select(Some(app.explorer.selected));
    }

    frame.render_stateful_widget(list, area, &mut state);
}
