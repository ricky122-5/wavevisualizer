use image::GenericImageView;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;

pub fn load_album_art(path: &str, width: u32, height: u32) -> Vec<Line<'static>> {
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to load image: {}", e);
            return vec![];
        }
    };
    let img = img.resize_exact(width, height * 2, image::imageops::FilterType::Lanczos3);
    let mut lines = Vec::new();
    for y in (0..img.height()).step_by(2) {
        let mut line = Vec::new();
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let color = Color::Rgb(r, g, b);
            line.push(Span::styled("█", Style::default().fg(color)));
        }
        lines.push(Line::from(line));
    }
    lines
}
