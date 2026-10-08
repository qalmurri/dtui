pub mod chat;
pub mod common;
pub mod create_lobby;
pub mod dashboard;
pub mod explorer;
pub mod options;
pub mod private;

use ratatui::Frame;

use crate::{app::App, screen::Screen};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Dashboard => {
            dashboard::render(frame, app);
        }

        Screen::Explorer => {
            explorer::render(frame, app);
        }

        Screen::Chat => {
            chat::render(frame, app);
        }

        Screen::Options => {
            options::render(frame, app);
        }

        Screen::Config => {
            render_config(frame);
        }

        Screen::JoinPrivate => {
            private::render(frame);
        }

        Screen::CreateLobby => {
            create_lobby::render(frame);
        }
    }
}

fn render_config(frame: &mut Frame) {
    use ratatui::{
        layout::Alignment,
        widgets::{Block, Borders, Paragraph},
    };

    let area = frame.area();

    let text = "\n\n\
        Configuration\n\n\
        Belum ada konfigurasi.\n\n\
        Pengaturan client lainnya\n\
        akan ditempatkan di sini.\n\n\
        Tekan Esc untuk kembali.
    ";

    let paragraph = Paragraph::new(text)
        .block(Block::default().title(" Config ").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
