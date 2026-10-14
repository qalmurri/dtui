use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::app::AppState;

pub(super) fn render(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
    frame.render_widget(Block::default().borders(Borders::NONE), area);

    let Some(workspace) = state.desktop.active_workspace() else {
        return;
    };

    for id in &state.window_manager.z_order.windows {
        let Some(window) = state.window_manager.get(*id) else {
            continue;
        };

        if window.workspace_id != workspace.id {
            continue;
        }

        let geometry = window.geometry.fit(area.width, area.height);

        let window_area = Rect::new(
            area.x.saturating_add(geometry.x),
            area.y.saturating_add(geometry.y),
            geometry.width,
            geometry.height,
        );

        if window_area.is_empty() {
            continue;
        }

        crate::ui::window::render(
            frame,
            window_area,
            window,
            state.window_manager.focus.is_focused(window.id),
            state.terminals.screen(window.id),
            state.terminals.is_exited(window.id),
            state.terminal_input_mode && state.window_manager.focus.is_focused(window.id),
        );
    }
}
