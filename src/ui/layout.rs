use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::AppState, config::Config};

pub fn render(frame: &mut Frame<'_>, state: &AppState, config: &Config) {
    let area = frame.area();

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(frame, sections[0], state, config);

    if state.show_help {
        render_help(frame, sections[1], config);
    } else {
        render_desktop(frame, sections[1], state);
    }

    render_footer(frame, sections[2], state, config);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, state: &AppState, config: &Config) {
    let workspace = state
        .desktop
        .active_workspace()
        .map(|workspace| workspace.name.as_str())
        .unwrap_or("NONE");

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {} ", config.desktop.title),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("| "),
        Span::raw(workspace),
        Span::raw(" | "),
        Span::raw(format!(
            "{}:DEV  {}:TOOLS",
            config.keybindings.workspace_1, config.keybindings.workspace_2
        )),
    ]));

    frame.render_widget(header, area);
}

fn render_desktop(frame: &mut Frame<'_>, area: Rect, state: &AppState) {
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
        );
    }
}

fn render_help(frame: &mut Frame<'_>, area: Rect, config: &Config) {
    let bindings = &config.keybindings;

    let lines = vec![
        Line::raw(""),
        Line::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        shortcut(&bindings.quit, "Quit DTUI"),
        shortcut(&bindings.help, "Toggle shortcut help"),
        shortcut(&bindings.focus_next, "Focus next window"),
        shortcut(
            &format!(
                "{}/{}/{}/{}",
                bindings.move_left, bindings.move_down, bindings.move_up, bindings.move_right
            ),
            "Move focused window (left/down/up/right)",
        ),
        shortcut(&bindings.resize_left, "Decrease window width"),
        shortcut(&bindings.resize_right, "Increase window width"),
        shortcut(&bindings.resize_up, "Decrease window height"),
        shortcut(&bindings.resize_down, "Increase window height"),
        shortcut(&bindings.new_window, "Create new window"),
        shortcut(&bindings.close_window, "Close focused window"),
        shortcut(&bindings.workspace_1, "Switch to workspace DEV"),
        shortcut(&bindings.workspace_2, "Switch to workspace TOOLS"),
        Line::raw(""),
        Line::raw("Press any key to return."),
    ];

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::NONE)),
        area,
    );
}

fn shortcut(key: &str, description: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw(format!("{key:<20}")),
        Span::raw(description.to_owned()),
    ])
}

fn render_footer(frame: &mut Frame<'_>, area: Rect, state: &AppState, config: &Config) {
    let bindings = &config.keybindings;

    let footer = Paragraph::new(Line::from(vec![
        Span::raw(" "),
        Span::raw(state.desktop.status.message.as_str()),
        Span::raw(" | Move: "),
        Span::raw(format!(
            "{}{}{}{}",
            bindings.move_left, bindings.move_down, bindings.move_up, bindings.move_right
        )),
        Span::raw(" | Focus: "),
        Span::raw(bindings.focus_next.as_str()),
        Span::raw(" | Help: "),
        Span::raw(bindings.help.as_str()),
        Span::raw(" | Quit: "),
        Span::raw(bindings.quit.as_str()),
        Span::raw(" "),
    ]));

    frame.render_widget(footer, area);
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend};

    use super::render_desktop;
    use crate::{
        app::AppState,
        desktop::workspace::WorkspaceId,
        window_manager::{Geometry, WindowManager},
    };

    fn geometry(x: u16, y: u16) -> Geometry {
        Geometry {
            x,
            y,
            width: 30,
            height: 8,
        }
    }

    #[test]
    fn renders_only_windows_from_active_workspace() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut state = AppState::default();
        state.window_manager = WindowManager::default();

        state
            .window_manager
            .create("VISIBLE", geometry(0, 0), WorkspaceId(0));

        state
            .window_manager
            .create("HIDDEN", geometry(0, 0), WorkspaceId(1));

        terminal
            .draw(|frame| {
                render_desktop(frame, frame.area(), &state);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();

        let content: String = (0..buffer.area.height)
            .flat_map(|y| (0..buffer.area.width).map(move |x| buffer[(x, y)].symbol().to_owned()))
            .collect();

        assert!(content.contains("VISIBLE"));
        assert!(!content.contains("HIDDEN"));
    }

    #[test]
    fn renders_top_window_over_window_behind_it() {
        let backend = TestBackend::new(60, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut state = AppState::default();
        state.window_manager = WindowManager::default();

        state
            .window_manager
            .create("BACK", geometry(0, 0), WorkspaceId(0));

        state
            .window_manager
            .create("FRONT", geometry(0, 0), WorkspaceId(0));

        terminal
            .draw(|frame| {
                render_desktop(frame, frame.area(), &state);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();

        let title: String = (0..buffer.area.width)
            .map(|x| buffer[(x, 0)].symbol())
            .collect();

        assert!(title.contains("FRONT"));
        assert!(!title.contains("BACK"));
    }
}
