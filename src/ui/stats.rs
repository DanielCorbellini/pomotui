use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

const DAYS_IN_MONTH: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn render(frame: &mut Frame, area: Rect) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Summary KPIs
            Constraint::Min(20),   // Contribution Heatmap
        ])
        .split(area);

    render_summary_cards(frame, main_chunks[0]);
    render_contribution_graph(frame, main_chunks[1]);
}

fn render_summary_cards(frame: &mut Frame, area: Rect) {
    let kpi_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let kpis = [
        ("🍅 Total Sessions", "128", Color::Rgb(255, 130, 0)),
        ("⏱️ Total Focus Time", "53h 20m", Color::Cyan),
        ("🔥 Current Streak", "7 days", Color::Yellow),
        ("🏆 Longest Streak", "14 days", Color::Green),
    ];

    for (i, (title, value, color)) in kpis.iter().enumerate() {
        let card_text = vec![Line::from(""), Line::from(value.bold().fg(*color))];

        let card = Paragraph::new(card_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(format!(" {} ", title))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            );

        frame.render_widget(card, kpi_chunks[i]);
    }
}

fn render_contribution_graph(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" 📊 Yearly Activity Heatmap (Monthly Calendar Grid) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(255, 130, 0)));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if inner_area.width < 40 || inner_area.height < 10 {
        return;
    }

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let day_labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    let mut lines = Vec::new();

    // Render 12 months in 2 rows of 6 months (Jan-Jun, Jul-Dec)
    let month_groups = [&months[0..6], &months[6..12]];

    for (group_idx, group) in month_groups.iter().enumerate() {
        let month_offset = group_idx * 6;

        // 1. Month Header Row
        let mut header_spans = vec![Span::raw("   ")]; // Space for day label column
        for (m_rel, month) in group.iter().enumerate() {
            let m_idx = month_offset + m_rel;
            let total_days = DAYS_IN_MONTH[m_idx];
            let num_cols = (total_days + 6) / 7;
            let width_chars = num_cols * 3;

            let header_str = format!("{:^width$} ", month, width = width_chars);
            header_spans.push(Span::styled(
                header_str,
                Style::default().fg(Color::White).bold(),
            ));
        }
        lines.push(Line::from(header_spans));

        // 2. 7 Lines for Days of the Week (Mon..Sun)
        for day_idx in 0..7 {
            let mut row_spans = Vec::new();

            // Day label column (in bold white)
            let label = format!("{:<4}", day_labels[day_idx]);
            row_spans.push(Span::styled(
                label,
                Style::default().fg(Color::White).bold(),
            ));

            // 6 Months in this row
            for m_rel in 0..6 {
                let m_idx = month_offset + m_rel;
                let total_days = DAYS_IN_MONTH[m_idx];
                let num_cols = (total_days + 6) / 7;

                for week_idx in 0..num_cols {
                    let day_num = week_idx * 7 + day_idx + 1;

                    // Early exit guard clause for non-existent days (eliminating else)
                    if day_num > total_days {
                        row_spans.push(Span::raw("   "));
                        continue;
                    }

                    let level = get_mock_level(m_idx, week_idx, day_idx);
                    let color = get_level_color(level);
                    row_spans.push(Span::styled("██ ", Style::default().fg(color)));
                }

                // Compact gap between month blocks (1 space)
                row_spans.push(Span::raw(" "));
            }

            lines.push(Line::from(row_spans));
        }

        lines.push(Line::from("")); // Blank line between month rows
    }

    // 3. Legend Row (in bold white with double-block icons)
    let legend_spans = vec![
        Span::styled("  Less ", Style::default().fg(Color::White).bold()),
        Span::styled("██ ", Style::default().fg(get_level_color(0))),
        Span::styled("██ ", Style::default().fg(get_level_color(1))),
        Span::styled("██ ", Style::default().fg(get_level_color(2))),
        Span::styled("██ ", Style::default().fg(get_level_color(3))),
        Span::styled("██ ", Style::default().fg(get_level_color(4))),
        Span::styled(
            "More (Darker Orange = More Sessions) ",
            Style::default().fg(Color::White).bold(),
        ),
    ];
    lines.push(Line::from(legend_spans));

    let heatmap_paragraph = Paragraph::new(lines);
    frame.render_widget(heatmap_paragraph, inner_area);
}

// Generate realistic pseudo-random mock contribution levels (0 to 4)
fn get_mock_level(month: usize, week: usize, day: usize) -> u8 {
    // Weekends (Sat=5, Sun=6) have lower activity
    if day >= 5 {
        return ((month * 4 + week + day) % 2) as u8;
    }

    let seed = (month * 17 + week * 7 + day * 5) % 100;
    match seed {
        0..=25 => 0,  // no sessions
        26..=50 => 1, // 1-2 sessions (light orange)
        51..=75 => 2, // 3-4 sessions (medium orange)
        76..=90 => 3, // 5-6 sessions (deep orange)
        _ => 4,       // 7+ sessions (darkest orange)
    }
}

// Orange contribution color palette (from Level 0 to Level 4)
fn get_level_color(level: u8) -> Color {
    match level {
        0 => Color::Rgb(40, 44, 52),    // Inactive (Dark Gray background)
        1 => Color::Rgb(255, 179, 102), // Light Orange
        2 => Color::Rgb(255, 130, 0),   // Medium Orange
        3 => Color::Rgb(210, 80, 0),    // Deep Orange
        _ => Color::Rgb(150, 35, 0),    // Dark Burnt Orange (Most sessions)
    }
}
