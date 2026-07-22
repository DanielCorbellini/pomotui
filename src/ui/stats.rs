use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, area: Rect) {
    let stats_text = vec![
        Line::from("📊 Productivity History").bold().cyan(),
        Line::from(""),
        Line::from("• Focus sessions completed today: 0"),
        Line::from("• Total focus time accumulated: 0 minutes"),
        Line::from("• Average daily performance: 0%"),
        Line::from(""),
        Line::from("💡 Note: Soon we will save this data locally on your computer!")
            .italic()
            .gray(),
    ];

    let stats_panel = Paragraph::new(stats_text).block(
        Block::default()
            .title(" Statistics Panel ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(stats_panel, area);
}
