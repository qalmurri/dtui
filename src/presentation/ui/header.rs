use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::application::{AppState, settings::Config};

pub(super) fn render(
    frame: &mut Frame<'_>,
    mode_status_area: Rect,
    workspace_area: Rect,
    state: &AppState,
    config: &Config,
) {
    let mode = if state.terminal_input_mode {
        " TERM "
    } else {
        " DESK "
    };

    let mode_style = Style::default()
        .fg(Color::Black)
        .bg(Color::White)
        .add_modifier(Modifier::BOLD);
    let mut status_spans = vec![Span::styled(mode, mode_style)];
    if !state.desktop.status.message.is_empty() {
        status_spans.push(Span::raw(" "));
        status_spans.push(Span::styled(
            state.desktop.status.message.as_str(),
            Style::default().fg(Color::Gray),
        ));
    }
    let time = system_time_label(&config.desktop.time_format);
    let time_width = time
        .chars()
        .count()
        .min(usize::from(mode_status_area.width)) as u16;
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(time_width)])
        .split(mode_status_area);

    frame.render_widget(Paragraph::new(Line::from(status_spans)), columns[0]);
    frame.render_widget(
        Paragraph::new(time)
            .alignment(Alignment::Right)
            .style(Style::default().fg(Color::Gray)),
        columns[1],
    );

    let mut spans = Vec::new();

    for (index, workspace) in state.desktop.workspaces.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw("  "));
        }

        let label = format!("[{}] {}", index + 1, workspace.name);

        if index == state.desktop.active_workspace {
            spans.push(Span::styled(
                label,
                Style::default().add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::raw(label));
        }
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), workspace_area);
}

fn system_time_label(format: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();

    if format.is_empty() {
        return timestamp.to_string();
    }

    let Ok(format) = std::ffi::CString::new(format) else {
        return timestamp.to_string();
    };
    let timestamp = timestamp as libc::time_t;
    let mut local_time = std::mem::MaybeUninit::<libc::tm>::uninit();
    // `localtime_r` initializes the value when it returns a non-null pointer.
    let local_time = unsafe {
        if libc::localtime_r(&timestamp, local_time.as_mut_ptr()).is_null() {
            return timestamp.to_string();
        }
        local_time.assume_init()
    };

    let mut output = [0 as libc::c_char; 128];
    let written = unsafe {
        libc::strftime(
            output.as_mut_ptr(),
            output.len(),
            format.as_ptr(),
            &local_time,
        )
    };
    if written == 0 {
        return timestamp.to_string();
    }

    unsafe { std::ffi::CStr::from_ptr(output.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}
