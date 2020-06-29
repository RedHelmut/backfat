use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct PageSizeInfo {
    pub page_height_pixels: f64,
    pub page_width_pixels: f64,
    pub dpi: f64,
    pub top_margin: f64,
    pub bottom_margin: f64,
}
impl PageSizeInfo {
    pub fn new(
        page_width_pixels: f64,
        page_height_pixels: f64,
        dpi: f64,
        top_margin: f64,
        bottom_margin: f64,
    ) -> Self {
        Self {
            page_height_pixels,
            page_width_pixels,
            dpi,
            top_margin,
            bottom_margin,
        }
    }
}
impl Display for PageSizeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pixel Width: {}, Pixel Height: {}, DPI: {}, Top Margin: {}, Bottom Margin: {}",
            self.page_width_pixels,
            self.page_height_pixels,
            self.dpi,
            self.top_margin,
            self.bottom_margin
        )
    }
}
