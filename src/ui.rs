use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Color, Style},
    symbols::Marker,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
};
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
        {
            let mut data: Vec<f32> = Vec::new();
            let lock = audio_data.lock().unwrap();
            if lock.len() >= 4096 {
                data = lock.clone();
                drop(lock);
            }
            if !data.is_empty() {
                let width = 200;
                let skip = data.len() / width;
                let mut points = Vec::new();

                for (i, val) in data.iter().step_by(skip).take(width).enumerate() {
                    points.push((i as f64, *val as f64));
                }

                terminal.draw(|f| {
                    let size = f.area();

                    let datasets = vec![
                        Dataset::default()
                            .name("Wave")
                            .marker(Marker::Dot)
                            .graph_type(GraphType::Line)
                            .style(Style::default().fg(Color::White))
                            .data(&points),
                    ];

                    let chart = Chart::new(datasets)
                        .block(Block::default().title("Now Playing").borders(Borders::ALL))
                        .x_axis(Axis::default().bounds([0.0, width as f64]))
                        .y_axis(Axis::default().bounds([-1.0, 1.0]));

                    f.render_widget(chart, size);
                })?;
            }
        }

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
