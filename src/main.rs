use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::io::{self, stdout};
use std::time::Duration;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(area);

            let header = Paragraph::new("⏱️  POMOTUI - LazyDocker layout")
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(Color::Rgb(255, 90, 95))
                        .add_modifier(Modifier::BOLD),
                )
                .block(
                    Block::default()
                        .borders(Borders::BOTTOM)
                        .border_style(Style::default().fg(Color::DarkGray)),
                );
            frame.render_widget(header, chunks[0]);

            let main_columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(chunks[1]);

            let col1_blocks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Ratio(1, 2),
                    Constraint::Ratio(1, 2),
                    Constraint::Length(10),
                    //Constraint::Ratio(1, 4),
                ])
                .split(main_columns[0]);

            let titulos_col1 = ["📝 TODO list", "📊 Report", "💾 History"];
            for (i, titulo) in titulos_col1.iter().enumerate() {
                let bloco = Block::default()
                    .title(format!(" {} ", titulo))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan));

                frame.render_widget(bloco, col1_blocks[i]);
            }

            let main_view_block = Block::default()
                .title(" 📄 Pomodoro Timer ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green));
            frame.render_widget(main_view_block, main_columns[1]);

            let footer = Paragraph::new(Line::from(vec![
                "[q]".bold().red(),
                " Quit ".into(),
                "| [Tab]".bold().yellow(),
                " Navigate ".into(),
            ]))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
            frame.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
