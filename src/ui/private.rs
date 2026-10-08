use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let text = "\n\n\
        Join Chat Private\n\n\
        Masukkan kode private chat.\n\n\
        Input belum diimplementasikan.\n\n\
        Tekan Esc untuk kembali.
    ";

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Private Chat ")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
