use crate::{
    application::ports::{TerminalColor, TerminalScreen},
    domain::window_manager::Window,
};
use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render(
    frame: &mut Frame<'_>,
    area: Rect,
    window: &Window,
    focused: bool,
    screen: Option<&dyn TerminalScreen>,
    process_exited: bool,
    terminal_input_mode: bool,
) {
    if area.is_empty() {
        return;
    }

    // Clear the entire window area before drawing it.
    // This prevents cells left by a previously rendered window
    // from remaining visible through empty or unwritten cells.
    frame.render_widget(Clear, area);

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
        frame.render_widget(Paragraph::new("Terminal session unavailable"), inner);
        return;
    };

    let mut lines = Vec::with_capacity(inner.height as usize + 1);

    for row in 0..inner.height {
        let mut spans = Vec::with_capacity(inner.width as usize);

        for col in 0..inner.width {
            let Some(cell) = screen.cell(row, col) else {
                // Always write a blank cell when the source screen
                // does not contain a cell at this position.
                spans.push(Span::raw(" "));
                continue;
            };

            let mut style = Style::default()
                .fg(convert_color(cell.foreground))
                .bg(convert_color(cell.background));

            if cell.bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if cell.dim {
                style = style.add_modifier(Modifier::DIM);
            }
            if cell.italic {
                style = style.add_modifier(Modifier::ITALIC);
            }
            if cell.underline {
                style = style.add_modifier(Modifier::UNDERLINED);
            }
            if cell.inverse {
                style = style.add_modifier(Modifier::REVERSED);
            }

            let contents = if cell.wide_continuation {
                // A continuation cell has no independent character.
                // Clear it explicitly instead of leaving the previous
                // window's buffer contents in that position.
                " "
            } else if cell.has_contents {
                cell.contents
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

fn convert_color(color: TerminalColor) -> Color {
    match color {
        TerminalColor::Default => Color::Reset,
        TerminalColor::Indexed(index) => Color::Indexed(index),
        TerminalColor::Rgb(red, green, blue) => Color::Rgb(red, green, blue),
    }
}
