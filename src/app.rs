use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Timer,
    Stats,
    Settings,
}

impl Tab {
    pub const ALL: [Tab; 3] = [Tab::Timer, Tab::Stats, Tab::Settings];

    pub fn to_index(self) -> usize {
        match self {
            Tab::Timer => 0,
            Tab::Stats => 1,
            Tab::Settings => 2,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Tab::Timer),
            1 => Some(Tab::Stats),
            2 => Some(Tab::Settings),
            _ => None,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Tab::Timer => "[1] 🍅 Timer",
            Tab::Stats => "[2] 📊 Stats",
            Tab::Settings => "[3] ⚙️ Settings",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerStatus {
    Paused,
    Running,
    Finished,
}

pub struct App {
    pub active_tab: Tab,
    pub timer_status: TimerStatus,
    pub time_remaining: Duration,
    pub total_duration: Duration,
    pub last_tick: Instant,
}

impl App {
    pub fn new() -> Self {
        let default_duration = Duration::from_secs(25 * 60);
        Self {
            active_tab: Tab::Timer,
            timer_status: TimerStatus::Paused,
            time_remaining: default_duration,
            total_duration: default_duration,
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        if self.timer_status != TimerStatus::Running {
            return;
        }

        let elapsed = self.last_tick.elapsed();

        if self.time_remaining > elapsed {
            self.time_remaining -= elapsed;
            self.last_tick = Instant::now();
            return;
        }

        self.time_remaining = Duration::ZERO;
        self.timer_status = TimerStatus::Finished;
    }

    pub fn toggle_timer(&mut self) {
        match self.timer_status {
            TimerStatus::Paused | TimerStatus::Finished => {
                if self.timer_status == TimerStatus::Finished {
                    self.reset_timer();
                }
                self.timer_status = TimerStatus::Running;
                self.last_tick = Instant::now();
            }
            TimerStatus::Running => {
                self.timer_status = TimerStatus::Paused;
            }
        }
    }

    pub fn reset_timer(&mut self) {
        self.timer_status = TimerStatus::Paused;
        self.time_remaining = self.total_duration;
    }

    pub fn next_tab(&mut self) {
        let current_index = self.active_tab.to_index();
        let next_index = (current_index + 1) % Tab::ALL.len();
        if let Some(next_tab) = Tab::from_index(next_index) {
            self.active_tab = next_tab;
        }
    }

    pub fn previous_tab(&mut self) {
        let current_index = self.active_tab.to_index();
        let prev_index = (current_index + Tab::ALL.len() - 1) % Tab::ALL.len();
        if let Some(prev_tab) = Tab::from_index(prev_index) {
            self.active_tab = prev_tab;
        }
    }

    pub fn progress_percent(&self) -> u16 {
        let total_secs = self.total_duration.as_secs();
        if total_secs == 0 {
            return 0;
        }

        let elapsed = total_secs.saturating_sub(self.time_remaining.as_secs());
        ((elapsed as f64 / total_secs as f64) * 100.0) as u16
    }
}
