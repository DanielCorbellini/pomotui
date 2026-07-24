use crate::app::{App, TimerStatus};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let info_text = vec![
        Line::from(vec!["🍅 Current Mode: ".bold().cyan(), "Focus".into()]),
        Line::from(vec![
            "⏱️  Timer Target: ".bold().cyan(),
            "25 Minutes".into(),
        ]),
        Line::from(""),
        Line::from("💡 Best Practices:".underlined().bold()),
        Line::from("- Avoid distractions"),
        Line::from("- Focus on the current task"),
        Line::from("- Respect breaks"),
    ];

    let info_panel = Paragraph::new(info_text).block(
        Block::default()
            .title(" Focus Panel ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(info_panel, columns[0]);

    let timer_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(0),
            Constraint::Min(0),
            Constraint::Percentage(0),
        ])
        .split(columns[1]);

    let theme_color = match app.timer_status {
        TimerStatus::Running => Color::Rgb(255, 90, 95),
        TimerStatus::Paused => Color::Yellow,
        TimerStatus::Finished => Color::Green,
    };

    let status_label = match app.timer_status {
        TimerStatus::Running => " 🍅 ACTIVE FOCUS ",
        TimerStatus::Paused => " ⏸️ TIMER PAUSED ",
        TimerStatus::Finished => " 🎉 SESSION COMPLETED! ",
    };

    let timer_block = Block::default()
        .title(status_label)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(theme_color));

    let inner_area = timer_block.inner(timer_vertical[1]);

    let scale_w = (inner_area.width / 29) as usize;
    let scale_h = ((inner_area.height.saturating_sub(4)) / 5) as usize;
    let scale = scale_w.min(scale_h).clamp(1, 7);
    let clock_height = (5 * scale) as u16;

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(clock_height),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner_area);

    frame.render_widget(timer_block, timer_vertical[1]);

    let minutes = app.time_remaining.as_secs() / 60;
    let seconds = app.time_remaining.as_secs() % 60;
    let time_str = format!("{:02}:{:02}", minutes, seconds);

    let big_time_lines = get_big_time(&time_str, scale);
    let clock = Paragraph::new(big_time_lines)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(clock, inner_chunks[1]);

    let percent = app.progress_percent();

    let gauge_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(4),
            Constraint::Percentage(92),
            Constraint::Percentage(4),
        ])
        .split(inner_chunks[3]);

    let progress_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .gauge_style(Style::default().fg(theme_color).bg(Color::Rgb(40, 40, 40)))
        .percent(percent)
        .use_unicode(true)
        .label(Span::from(format!("{}%", percent)).bold().white());
    frame.render_widget(progress_gauge, gauge_horizontal[1]);
}

fn get_big_time(time_str: &str, scale: usize) -> Vec<Line<'static>> {
    let mut lines = vec![
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];

    for ch in time_str.chars() {
        let ch_lines = match ch {
            '0' => &["█████", "█   █", "█   █", "█   █", "█████"],
            '1' => &["  ██ ", "   █ ", "   █ ", "   █ ", " ████"],
            '2' => &["█████", "    █", "█████", "█    ", "█████"],
            '3' => &["█████", "    █", "█████", "    █", "█████"],
            '4' => &["█   █", "█   █", "█████", "    █", "    █"],
            '5' => &["█████", "█    ", "█████", "    █", "█████"],
            '6' => &["█████", "█    ", "█████", "█   █", "█████"],
            '7' => &["█████", "    █", "   █ ", "  █  ", " █   "],
            '8' => &["█████", "█   █", "█████", "█   █", "█████"],
            '9' => &["█████", "█   █", "█████", "    █", "█████"],
            ':' => &["   ", " █ ", "   ", " █ ", "   "],
            _ => &["     ", "     ", "     ", "     ", "     "],
        };

        for i in 0..5 {
            if !lines[i].is_empty() {
                for _ in 0..scale {
                    lines[i].push(' ');
                }
            }
            for char_in_digit in ch_lines[i].chars() {
                for _ in 0..scale {
                    lines[i].push(char_in_digit);
                }
            }
        }
    }

    let mut scaled_lines = Vec::new();
    for line in lines {
        for _ in 0..scale {
            scaled_lines.push(Line::from(line.clone()));
        }
    }
    scaled_lines
}
