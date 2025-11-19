use crate::album_art;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    text::Line,
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, GraphType, Paragraph},
};
use std::io::{self, Stdout};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::Local;

use crate::metadata::TrackInfo;

pub fn start_ui(
    audio_data: Arc<Mutex<Vec<f32>>>,
    info: Arc<Mutex<Option<TrackInfo>>>,
) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut cached_cover_path = String::new();
    let mut cached_song_name = String::new();
    let mut cached_art_lines: Vec<Line<'static>> = Vec::new();
    let mut smoothed_max = 0.1;
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

                let current_max = data
                    .iter()
                    .map(|v| v.abs())
                    .fold(0.0f32, |a, b| a.max(b))
                    .max(0.05);

                smoothed_max = smoothed_max * 0.85 + current_max * 0.15;

                let y_bound = (smoothed_max * 1.2).min(1.0).max(0.1);

                let mut ratio = 0.0;
                let mut current_song = String::from("No music playing");
                if let Ok(lock) = info.lock() {
                    if let Some(inf) = &*lock {
                        let pos = inf.position.parse::<f64>().unwrap_or(0.0);
                        let dur = inf.duration.parse::<f64>().unwrap_or(1.0);
                        ratio = (pos / dur).clamp(0.0, 1.0);
                        current_song = format!("{} by {}", inf.name, inf.artist);
                        if inf.cover != cached_cover_path || inf.name != cached_song_name {
                            cached_cover_path = inf.cover.clone();
                            cached_song_name = inf.name.clone();
                            cached_art_lines = album_art::load_album_art(&inf.cover, 60, 22);
                        }
                    }
                }

                terminal.draw(|f| {
                    let size = f.area();
                    let datasets = vec![
                        Dataset::default()
                            .marker(Marker::Dot)
                            .graph_type(GraphType::Line)
                            .style(Style::default().fg(Color::Indexed(208)))
                            .data(&points),
                    ];

                    let chart = Chart::new(datasets)
                        .block(Block::default().title("Now Playing").borders(Borders::ALL))
                        .x_axis(Axis::default().bounds([0.0, width as f64]))
                        .y_axis(Axis::default().bounds([-y_bound as f64, y_bound as f64]));

                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                        .split(size);
                    let info_block = Block::default().title("Song Info").borders(Borders::ALL);
                    let p = Paragraph::new(current_song)
                        .alignment(Alignment::Center)
                        .block(info_block);
                    let art = Paragraph::new(cached_art_lines.clone())
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL));

                    let right_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Min(10),
                            Constraint::Length(3),
                        ])
                        .split(chunks[1]);
                    let gauge = Gauge::default()
                        .block(Block::default().title("Progress").borders(Borders::ALL))
                        .gauge_style(Style::default().fg(Color::Indexed(208)))
                        .ratio(ratio);
                    f.render_widget(p, right_chunks[0]);
                    f.render_widget(chart, chunks[0]);
                    f.render_widget(gauge, right_chunks[2]);
                    f.render_widget(art, right_chunks[1]);
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
