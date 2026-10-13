use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use vt100::{Color as VtColor, Screen};

use crate::window_manager::Window;

pub fn render(
    frame: &mut Frame<'_>,
    area: Rect,
    window: &Window,
    focused: bool,
    screen: Option<&Screen>,
    process_exited: bool,
    terminal_input_mode: bool,
) {
    let title = if focused {
        format!(
            " {}{} ",
            window.title,
            if terminal_input_mode {
                " [Terminal]"
            } else {
                " [Focused]"
            }
        )
    } else {
        format!(" {} ", window.title)
    };

    let border_style = if focused {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.is_empty() {
        return;
    }

    let Some(screen) = screen else {
        let content = Paragraph::new("Terminal session unavailable");
        frame.render_widget(content, inner);
        return;
    };

    let mut lines = Vec::with_capacity(inner.height as usize + 1);
    for row in 0..inner.height {
        let mut spans = Vec::with_capacity(inner.width as usize);
        for col in 0..inner.width {
            let Some(cell) = screen.cell(row, col) else {
                continue;
            };

            let mut style = Style::default()
                .fg(convert_color(cell.fgcolor()))
                .bg(convert_color(cell.bgcolor()));

            if cell.bold() {
                style = style.add_modifier(Modifier::BOLD);
            }
            if cell.dim() {
                style = style.add_modifier(Modifier::DIM);
            }
            if cell.italic() {
                style = style.add_modifier(Modifier::ITALIC);
            }
            if cell.underline() {
                style = style.add_modifier(Modifier::UNDERLINED);
            }
            if cell.inverse() {
                style = style.add_modifier(Modifier::REVERSED);
            }
            let contents = if cell.is_wide_continuation() {
                ""
            } else if cell.has_contents() {
                cell.contents()
            } else {
                " "
            };
            spans.push(Span::styled(contents.to_owned(), style));
        }
        lines.push(Line::from(spans));
    }

    if process_exited {
        lines.push(Line::styled(
            "[process exited]",
            Style::default().add_modifier(Modifier::DIM),
        ));
    }

    frame.render_widget(Paragraph::new(lines), inner);

    if focused && terminal_input_mode && !screen.hide_cursor() {
        let (row, col) = screen.cursor_position();
        if row < inner.height && col < inner.width {
            frame.set_cursor_position(Position::new(
                inner.x.saturating_add(col),
                inner.y.saturating_add(row),
            ));
        }
    }
}

fn convert_color(color: VtColor) -> Color {
    match color {
        VtColor::Default => Color::Reset,
        VtColor::Idx(index) => Color::Indexed(index),
        VtColor::Rgb(red, green, blue) => Color::Rgb(red, green, blue),
    }
}
