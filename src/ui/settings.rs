use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let settings_text = vec![
        Line::from("⚙️  Pomotui Settings").bold().magenta(),
        Line::from(""),
        Line::from("🍅 Focus Time: 25 minutes"),
        Line::from("☕ Short Break: 5 minutes"),
        Line::from("🌴 Long Break: 15 minutes"),
        Line::from("🔊 Notification Sounds: Enabled"),
        Line::from(""),
        Line::from("💡 Note: In the future you will be able to edit these times interactively!")
            .italic()
            .gray(),
    ];

    let settings_panel = Paragraph::new(settings_text).block(
        Block::default()
            .title(" Settings Panel ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta)),
    );
    frame.render_widget(settings_panel, area);
}
