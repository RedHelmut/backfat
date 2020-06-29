use crate::font::font_sizes::Font;

#[derive(Clone)]
pub struct FontInfo {
    pub font: Font,
    pub size: f64,
    pub font_color: (f64, f64, f64),
}

impl FontInfo {
    pub fn new(size: f64, font: Font) -> Self {
        Self {
            font,
            size,
            font_color: (0.0, 0.0, 0.0),
        }
    }

    pub fn new_colored(size: f64, font: Font, font_color: (f64, f64, f64)) -> Self {
        Self {
            font,
            size,
            font_color,
        }
    }
}

pub fn get_font_breakdown(font_size_p: f64) -> (f64, f64, f64) {
    let font_size = if font_size_p % 2.0 == 0.0 {
        font_size_p + 0.001
    } else {
        font_size_p
    };
    let fs_inch = (font_size / 72.0) as f64;
    (font_size, fs_inch / (font_size * 0.5), fs_inch)
}
