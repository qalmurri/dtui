use ratatui::Frame;

use crate::{app::AppState, ui};

pub fn render(frame: &mut Frame<'_>, state: &AppState) {
    ui::layout::render(frame, state);
}
