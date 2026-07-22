mod settings;
mod stats;
mod timer;

use crate::app::{App, Tab};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Tabs},
};
use tui_big_text::{BigText, PixelSize};

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let header = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .style(Style::default().fg(Color::Rgb(255, 90, 95)).bold())
        .lines(vec![Line::from("POMOTUI - Your pomodoro in the terminal")])
        .alignment(Alignment::Center)
        .build();
    frame.render_widget(header, chunks[0]);

    let tab_titles = Tab::ALL.iter().map(|t| t.to_string()).collect::<Vec<_>>();
    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .select(app.active_tab.to_index())
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(255, 90, 95))
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(tabs, chunks[1]);

    match app.active_tab {
        Tab::Timer => timer::render(app, frame, chunks[2]),
        Tab::Stats => stats::render(frame, chunks[2]),
        Tab::Settings => settings::render(frame, chunks[2]),
    }

    let footer_text = Line::from(vec![
        "[q]".bold().red(),
        " Quit ".into(),
        "| [Space]".bold().yellow(),
        " Start/Pause ".into(),
        "| [r]".bold().cyan(),
        " Reset ".into(),
        "| [←/→]".bold().magenta(),
        " Switch Tab ".into(),
        "| [1,2,3]".bold().magenta(),
        " Go to Tab".into(),
    ]);

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(footer, chunks[3]);
}
