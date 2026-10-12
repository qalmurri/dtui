#!/usr/bin/env bash
set -euo pipefail

# Run from the DTUI project root, where Cargo.toml exists.
if [[ ! -f Cargo.toml ]]; then
  echo "Error: Cargo.toml tidak ditemukan. Jalankan script dari root proyek DTUI." >&2
  exit 1
fi

mkdir -p src/ui/components src/ui/screens

# Preserve any existing Rust sources before replacing them.
backup="backup_before_dtui_reset_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$backup"
for file in src/main.rs src/app.rs src/config.rs src/event.rs src/keybindings.rs src/ui/layout.rs src/ui/mod.rs src/ui/components/mod.rs src/ui/screens/mod.rs; do
  if [[ -f "$file" ]]; then
    mkdir -p "$backup/$(dirname "$file")"
    cp "$file" "$backup/$file"
  fi
done

cat > Cargo.toml <<'EOF'
[package]
name = "dtui"
version = "0.1.0"
edition = "2024"

[dependencies]
ratatui = "0.30"
crossterm = "0.29"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
directories = "6"
EOF

cat > src/main.rs <<'EOF'
mod app;
mod config;
mod event;
mod keybindings;
mod ui;

use std::{
    error::Error,
    io::{self, Stdout},
};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load_or_create()?;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let _guard = TerminalGuard;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let result = app::run(&mut terminal, &config);
    drop(terminal);
    result
}
EOF

cat > src/config.rs <<'EOF'
use std::{error::Error, fs, io, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub desktop: DesktopConfig,
    pub appearance: AppearanceConfig,
    pub keybindings: Keybindings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DesktopConfig {
    pub title: String,
    pub subtitle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub accent: String,
    pub muted: String,
    pub show_borders: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Keybindings {
    pub quit: String,
    pub back: String,
    pub navigate_up: String,
    pub navigate_down: String,
    pub select: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            desktop: DesktopConfig::default(),
            appearance: AppearanceConfig::default(),
            keybindings: Keybindings::default(),
        }
    }
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            title: "DTUI".into(),
            subtitle: "A simple desktop for your terminal".into(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            accent: "cyan".into(),
            muted: "gray".into(),
            show_borders: true,
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: "q".into(),
            back: "esc".into(),
            navigate_up: "up".into(),
            navigate_down: "down".into(),
            select: "enter".into(),
        }
    }
}

pub fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    let dirs = ProjectDirs::from("com", "qalmurri", "dtui")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tidak dapat menentukan direktori konfigurasi"))?;
    Ok(dirs.config_dir().join("config.toml"))
}

pub fn load_or_create() -> Result<Config, Box<dyn Error>> {
    let path = config_path()?;

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, toml::to_string_pretty(&Config::default())?)?;
    }

    let contents = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&contents)?;
    crate::keybindings::validate(&config.keybindings)?;
    Ok(config)
}
EOF

cat > src/keybindings.rs <<'EOF'
use std::error::Error;

use crossterm::event::KeyCode;

use crate::config::Keybindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Back,
    NavigateUp,
    NavigateDown,
    Select,
}

pub fn parse_key(value: &str) -> Result<KeyCode, String> {
    let value = value.trim().to_lowercase();

    match value.as_str() {
        "esc" | "escape" => Ok(KeyCode::Esc),
        "enter" | "return" => Ok(KeyCode::Enter),
        "up" => Ok(KeyCode::Up),
        "down" => Ok(KeyCode::Down),
        "left" => Ok(KeyCode::Left),
        "right" => Ok(KeyCode::Right),
        "tab" => Ok(KeyCode::Tab),
        "space" => Ok(KeyCode::Char(' ')),
        "f1" => Ok(KeyCode::F(1)),
        "f2" => Ok(KeyCode::F(2)),
        _ => {
            let mut chars = value.chars();
            match (chars.next(), chars.next()) {
                (Some(ch), None) => Ok(KeyCode::Char(ch)),
                _ => Err(format!("Nama tombol tidak didukung: '{value}'")),
            }
        }
    }
}

pub fn validate(bindings: &Keybindings) -> Result<(), Box<dyn Error>> {
    let entries = [
        ("quit", bindings.quit.as_str()),
        ("back", bindings.back.as_str()),
        ("navigate_up", bindings.navigate_up.as_str()),
        ("navigate_down", bindings.navigate_down.as_str()),
        ("select", bindings.select.as_str()),
    ];

    let mut seen: Vec<(KeyCode, &str)> = Vec::new();
    for (action, value) in entries {
        let key = parse_key(value)
            .map_err(|error| format!("Keybinding '{action}' tidak valid: {error}"))?;

        if let Some((_, previous)) = seen.iter().find(|(existing, _)| existing == &key) {
            return Err(format!("Tombol '{value}' dipakai oleh '{previous}' dan '{action}'").into());
        }
        seen.push((key, action));
    }

    Ok(())
}

pub fn resolve(key: KeyCode, bindings: &Keybindings) -> Option<Action> {
    let entries = [
        (&bindings.quit, Action::Quit),
        (&bindings.back, Action::Back),
        (&bindings.navigate_up, Action::NavigateUp),
        (&bindings.navigate_down, Action::NavigateDown),
        (&bindings.select, Action::Select),
    ];

    entries.into_iter().find_map(|(binding, action)| {
        (parse_key(binding).ok() == Some(key)).then_some(action)
    })
}

pub fn format_key(value: &str) -> String {
    match value.to_lowercase().as_str() {
        "esc" | "escape" => "Esc".into(),
        "enter" | "return" => "Enter".into(),
        "up" => "↑".into(),
        "down" => "↓".into(),
        "left" => "←".into(),
        "right" => "→".into(),
        "space" => "Space".into(),
        _ => value.to_uppercase(),
    }
}
EOF

cat > src/event.rs <<'EOF'
use std::{io, time::Duration};

use crossterm::event::{self, Event};

pub fn poll_event(timeout: Duration) -> io::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
EOF

cat > src/app.rs <<'EOF'
use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyEventKind};

use crate::{
    Tui,
    config::Config,
    event,
    keybindings::{self, Action},
    ui,
};

const MENU_ITEMS: [&str; 4] = ["Terminal", "File Browser", "System Monitor", "Settings"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Desktop,
    Menu,
}

pub struct AppState {
    screen: Screen,
    selected: usize,
    status: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: Screen::Desktop,
            selected: 0,
            status: "Welcome to DTUI".into(),
        }
    }
}

impl AppState {
    pub fn is_menu_open(&self) -> bool {
        self.screen == Screen::Menu
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn menu_items(&self) -> &'static [&'static str] {
        &MENU_ITEMS
    }
}

pub fn run(terminal: &mut Tui, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut state = AppState::default();

    loop {
        terminal.draw(|frame| ui::layout::render(frame, config, &state))?;

        if let Some(Event::Key(key)) = event::poll_event(Duration::from_millis(100))? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match keybindings::resolve(key.code, &config.keybindings) {
                Some(Action::Quit) => {
                    if state.screen == Screen::Menu {
                        state.screen = Screen::Desktop;
                    } else {
                        break;
                    }
                }
                Some(Action::Back) => {
                    if state.screen == Screen::Menu {
                        state.screen = Screen::Desktop;
                        state.status = "Menu closed".into();
                    }
                }
                Some(Action::NavigateUp) if state.screen == Screen::Menu => {
                    state.selected = state.selected.saturating_sub(1);
                }
                Some(Action::NavigateDown) if state.screen == Screen::Menu => {
                    state.selected = (state.selected + 1).min(MENU_ITEMS.len() - 1);
                }
                Some(Action::Select) => match state.screen {
                    Screen::Desktop => {
                        state.screen = Screen::Menu;
                        state.selected = 0;
                    }
                    Screen::Menu => {
                        state.status = format!("{} dipilih (belum diimplementasikan)", MENU_ITEMS[state.selected]);
                        state.screen = Screen::Desktop;
                    }
                },
                _ => {}
            }
        }
    }

    Ok(())
}
EOF

cat > src/ui/mod.rs <<'EOF'
pub mod components;
pub mod layout;
pub mod screens;
EOF

cat > src/ui/components/mod.rs <<'EOF'
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_panel(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    body: Vec<Line<'static>>,
    accent: Color,
    show_borders: bool,
    focused: bool,
) {
    let borders = if show_borders { Borders::ALL } else { Borders::NONE };
    let border_color = if focused { accent } else { Color::DarkGray };
    let block = Block::default()
        .title(format!(" {title} "))
        .borders(borders)
        .border_style(Style::default().fg(border_color));

    frame.render_widget(Paragraph::new(body).block(block), area);
}

pub fn render_header(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    subtitle: &str,
    accent: Color,
) {
    let line = Line::from(vec![
        Span::styled(format!(" {title} "), Style::default().fg(accent).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" / {subtitle}"), Style::default().fg(Color::Gray)),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

pub fn render_footer(frame: &mut Frame<'_>, area: Rect, status: &str, accent: Color) {
    let line = Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(Color::Black).bg(accent).add_modifier(Modifier::BOLD)),
        Span::raw(" Navigate  "),
        Span::styled(" Enter ", Style::default().fg(Color::Black).bg(accent).add_modifier(Modifier::BOLD)),
        Span::raw(" Menu/Select  "),
        Span::styled(" q ", Style::default().fg(Color::Black).bg(accent).add_modifier(Modifier::BOLD)),
        Span::raw(" Quit  "),
        Span::styled(format!("| {status}"), Style::default().fg(Color::Gray)),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
EOF

cat > src/ui/layout.rs <<'EOF'
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::{
    app::AppState,
    config::Config,
    ui::components::{render_footer, render_header, render_panel},
};

fn parse_color(value: &str) -> Color {
    match value.to_lowercase().as_str() {
        "black" => Color::Black,
        "blue" => Color::Blue,
        "cyan" => Color::Cyan,
        "green" => Color::Green,
        "magenta" => Color::Magenta,
        "red" => Color::Red,
        "white" => Color::White,
        "yellow" => Color::Yellow,
        "gray" | "grey" => Color::Gray,
        _ => Color::Cyan,
    }
}

pub fn render(frame: &mut Frame<'_>, config: &Config, state: &AppState) {
    let area = frame.area();
    let accent = parse_color(&config.appearance.accent);
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(5), Constraint::Length(1)])
        .split(area);

    render_header(
        frame,
        sections[0],
        &config.desktop.title,
        &config.desktop.subtitle,
        accent,
    );

    let body = vec![
        Line::raw(""),
        Line::styled("Welcome to your terminal desktop.", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Line::raw(""),
        Line::raw("Press Enter to open the application menu."),
        Line::raw(""),
        Line::styled("This is the minimal DTUI foundation.", Style::default().fg(Color::Gray)),
        Line::styled("Customization and application runtime will be added incrementally.", Style::default().fg(Color::Gray)),
    ];
    render_panel(
        frame,
        sections[1],
        " DESKTOP ",
        body,
        accent,
        config.appearance.show_borders,
        !state.is_menu_open(),
    );
    render_footer(frame, sections[2], state.status(), accent);

    if state.is_menu_open() {
        render_menu(frame, area, state, accent);
    }
}

fn render_menu(frame: &mut Frame<'_>, area: Rect, state: &AppState, accent: Color) {
    let width = area.width.min(42);
    let height = area.height.min(12);
    let popup = Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    };

    frame.render_widget(Clear, popup);
    let mut lines = vec![Line::styled(" Select an application ", Style::default().fg(Color::Gray))];
    lines.push(Line::raw(""));
    for (index, item) in state.menu_items().iter().enumerate() {
        let selected = index == state.selected();
        lines.push(Line::styled(
            format!(" {} {}", if selected { "›" } else { " " }, item),
            Style::default()
                .fg(if selected { accent } else { Color::White })
                .add_modifier(if selected { Modifier::BOLD } else { Modifier::empty() }),
        ));
    }
    lines.push(Line::raw(""));
    lines.push(Line::styled("Esc: close menu", Style::default().fg(Color::Gray)));

    let block = Block::default().title(" APPLICATIONS ").borders(Borders::ALL).border_style(Style::default().fg(accent));
    frame.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: true }), popup);
}
EOF

cat > src/ui/screens/mod.rs <<'EOF'
// Screen-specific rendering will be extracted here when DTUI grows.
// The first milestone keeps one desktop screen and one reusable menu overlay.
EOF

echo
echo "DTUI minimal baseline has been written."
echo "Existing source backups are in: $backup"
echo
echo "Now run:"
echo "  cargo check"
echo "  cargo run"
