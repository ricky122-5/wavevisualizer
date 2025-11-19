use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{BarChart, Block, Borders},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use spectrum_analyzer::FrequencySpectrum;
use std::io::{self, Stdout};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::Local;

pub fn start_ui(audio_data: Arc<Mutex<Vec<f32>>>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let now = Local::now();
            let time_str = now.format("%H:%M:%S").to_string();
            let block = Block::default().title(time_str).borders(Borders::ALL);
            f.render_widget(block, size);
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
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
