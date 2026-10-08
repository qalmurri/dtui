use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, dashboard::DashboardItem};

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

    render_title(frame, "Anonymous Chat", layout[0]);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(layout[1]);

    render_menu(frame, app, columns[0]);

    render_identity_panel(frame, app, columns[1]);

    render_footer(frame, "↑ ↓ Navigate   Enter Select   q Quit", layout[2]);
}

fn render_menu(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().title(" Menu ").borders(Borders::ALL);

    let items = app
        .dashboard
        .items
        .iter()
        .map(|item| {
            let text = match item {
                DashboardItem::JoinPublic => "Join chat publik",

                DashboardItem::JoinPrivate => "Join chat code private",

                DashboardItem::CreateLobby => "Create lobi",

                DashboardItem::Options => "Options",

                DashboardItem::Config => "Config",
            };

            ListItem::new(text)
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut state = ListState::default();

    state.select(Some(app.dashboard.selected));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_identity_panel(frame: &mut Frame, app: &App, area: Rect) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(area);

    render_identity(frame, app, sections[0]);

    render_client_info(frame, app, sections[1]);
}

fn render_identity(frame: &mut Frame, app: &App, area: Rect) {
    let text = format!("\nAlias\n\n{}", app.client.alias);

    let paragraph = Paragraph::new(text)
        .block(Block::default().title(" Identity ").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_client_info(frame: &mut Frame, app: &App, area: Rect) {
    let text = format!(
        "\
Client       : {}
Version      : {}
Last online  : {}
IP           : {}
",
        app.client.name, app.client.version, app.client.last_online, app.client.ip_address,
    );

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .title(" Client Information ")
            .borders(Borders::ALL),
    );

    frame.render_widget(paragraph, area);
}
