use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let text = "\n\n\
        Create Lobi\n\n\
        Nama lobi: belum tersedia\n\n\
        Visibility:\n\
        [x] Public\n\
        [ ] Private\n\n\
        Form belum diimplementasikan.\n\n\
        Tekan Esc untuk kembali.
    ";

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" Create Lobi ")
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
