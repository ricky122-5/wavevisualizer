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

pub fn get_avg_color(path: &str) -> Color {
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to load image");
            return Color::White;
        }
    };
    let mut avgR: u64 = 0;
    let mut avgG: u64 = 0;
    let mut avgB: u64 = 0;
    let mut count: u64 = 0;

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
            let saturation = {
                let max_val = r.max(g).max(b) as f32;
                let min_val = r.min(g).min(b) as f32;
                if max_val == 0.0 {
                    0.0
                } else {
                    (max_val - min_val) / max_val
                }
            };

            if luminance > 0.15 && luminance < 0.85 && saturation > 0.1 {
                avgR += r as u64;
                avgG += g as u64;
                avgB += b as u64;
                count += 1;
            }
        }
    }

    if count == 0 {
        return Color::White;
    }

    let mut avg_r = (avgR / count) as u8;
    let mut avg_g = (avgG / count) as u8;
    let mut avg_b = (avgB / count) as u8;

    let final_luminance =
        (0.299 * avg_r as f32 + 0.587 * avg_g as f32 + 0.114 * avg_b as f32) / 255.0;

    let dark_threshold = 0.25;
    let target_luminance = 0.50;

    if final_luminance < dark_threshold {
        if final_luminance < 0.05 {
            return Color::White;
        }

        let scale_factor = target_luminance / final_luminance.max(0.01);

        avg_r = ((avg_r as f32 * scale_factor).min(255.0)) as u8;
        avg_g = ((avg_g as f32 * scale_factor).min(255.0)) as u8;
        avg_b = ((avg_b as f32 * scale_factor).min(255.0)) as u8;
    }

    Color::Rgb(avg_r, avg_g, avg_b)
}
