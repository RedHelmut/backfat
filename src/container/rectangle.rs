use crate::container::page_size_info::PageSizeInfo;
use crate::container::placement_info::PlacementInfo;
use lopdf::Object;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn get_pdf_version(&self, page_info: PageSizeInfo) -> Rectangle {
        Rectangle::new(
            self.x,
            page_info.page_height_pixels - self.y - self.height,
            self.width,
            self.height,
        )
    }
}
impl From<Rectangle> for Vec<Object> {
    fn from(rec: Rectangle) -> Self {
        vec![
            rec.x.into(),
            rec.y.into(),
            rec.width.into(),
            rec.height.into(),
        ]
    }
}
impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl From<PlacementInfo> for Rectangle {
    fn from(placement_info: PlacementInfo) -> Self {
        Rectangle::new(
            placement_info.rec.x,
            placement_info.rec.y,
            placement_info.rec.width,
            placement_info.rec.height,
        )
    }
}
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x = {}\r\ny = {}\r\nwidth = {}\r\nheight = {}",
            self.x, self.y, self.width, self.height
        )
    }
}

#[derive(Clone)]
pub struct Border {
    pub rec: PlacementInfo,
    pub color: (f64, f64, f64),
    pub pixel_size: f64,
}
impl Border {
    pub fn new(rec: PlacementInfo, color: (f64, f64, f64), pixel_size: f64) -> Self {
        Self {
            rec,
            color,
            pixel_size,
        }
    }
}
