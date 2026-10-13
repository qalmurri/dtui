use ratatui::Frame;

use crate::{app::AppState, config::Config, ui};

pub fn render(frame: &mut Frame<'_>, state: &AppState, config: &Config) {
    ui::layout::render(frame, state, config);
}
