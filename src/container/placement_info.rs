use crate::container::page_size_info::PageSizeInfo;
use crate::container::rectangle::Rectangle;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Range;

#[derive(Clone)]
pub struct PlacementInfo {
    pub rec: Rectangle,
    pub is_new_page: bool,
    pub draw_height_left_on_page: f64,
    pub page_number: usize,
    pub page_size_info: PageSizeInfo,
    pub percent_range: Range<usize>,
    pub restricted_area_option: Option<Rectangle>,
}
impl Default for PlacementInfo {
    fn default() -> Self {
        Self {
            rec: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            is_new_page: false,
            draw_height_left_on_page: 0.0,
            page_number: 0,
            page_size_info: PageSizeInfo::new(0.0, 0.0, 0.0, 0.0, 0.0),
            percent_range: 0..1,
            restricted_area_option: None,
        }
    }
}
impl PlacementInfo {
    pub fn get_outer_rec(&self) -> Rectangle {
        self.rec.clone()
    }
    pub fn get_inner_rec(&self) -> Option<Rectangle> {
        self.restricted_area_option.clone()
    }
}
impl Display for PlacementInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r##"Top Left: {}, {}
Bottom Right: {}, {}
"##,
            self.rec.x,
            self.rec.y,
            self.rec.x + self.rec.width,
            self.rec.y + self.rec.height
        )
    }
}

#[derive(Clone)]
pub struct PlacementOptions {
    pub move_to_next_page: bool,
    pub ignore: bool,
}
