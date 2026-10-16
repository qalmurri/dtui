use ratatui::Frame;

use crate::{
    application::{AppState, settings::Config},
    presentation::ui,
};

pub fn render(frame: &mut Frame<'_>, state: &AppState, config: &Config) {
    ui::layout::render(frame, state, config);
}
