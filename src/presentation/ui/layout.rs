use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::application::{AppState, settings::Config};

use super::{desktop, header, help};

const MODE_STATUS_ROW_HEIGHT: u16 = 1;
const WORKSPACE_ROW_HEIGHT: u16 = 1;
pub const HEADER_ROWS: u16 = MODE_STATUS_ROW_HEIGHT + WORKSPACE_ROW_HEIGHT;

pub fn desktop_size(width: u16, terminal_height: u16) -> (u16, u16) {
    (width, terminal_height.saturating_sub(HEADER_ROWS))
}

pub fn render(frame: &mut Frame<'_>, state: &AppState, config: &Config) {
    let area = frame.area();

    if state.show_help {
        help::render(frame, area, config);
        return;
    }

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(MODE_STATUS_ROW_HEIGHT),
            Constraint::Length(WORKSPACE_ROW_HEIGHT),
            Constraint::Min(1),
        ])
        .split(area);

    header::render(frame, sections[0], sections[1], state, config);

    if let Some(input) = &state.workspace_input {
        let title = match input.mode {
            crate::application::WorkspaceInputMode::Rename => "Rename Workspace",
        };

        let dialog = Paragraph::new(format!(
            "{title}\n\nName: {}_\n\nEnter: save | Esc: cancel",
            input.value
        ))
        .block(Block::default().borders(Borders::ALL).title(title));

        let dialog_area = help::centered_rect(60, 30, sections[2]);
        frame.render_widget(dialog, dialog_area);
    } else {
        desktop::render(frame, sections[2], state);
    }
}
