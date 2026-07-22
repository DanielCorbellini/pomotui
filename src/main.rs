mod app;
mod ui;

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, stdout};
use std::time::Duration;

use app::{App, Tab};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(&app, frame))?;

        app.tick();

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Char(' ') => app.toggle_timer(),
                    KeyCode::Char('r') => app.reset_timer(),

                    KeyCode::Right => app.next_tab(),
                    KeyCode::Left => app.previous_tab(),

                    KeyCode::Char('1') => app.active_tab = Tab::Timer,
                    KeyCode::Char('2') => app.active_tab = Tab::Stats,
                    KeyCode::Char('3') => app.active_tab = Tab::Settings,

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
