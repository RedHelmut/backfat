use crate::container::page_size_info::PageSizeInfo;
use crate::container::placement_info::{PlacementInfo, PlacementOptions};
use crate::container::rectangle::Rectangle;
use std::collections::BTreeMap;
use std::ops::Range;

pub struct PageMargins {
    pub top_margin_pixels: f64,
    pub bottom_margin_pixels: f64,
}
pub struct PageMaster {
    columns: Vec<(usize, f64)>,
    page_width_pixels: f64,
    page_height_pixels: f64,
    dpi: f64,
    top_margin_inch: f64,
    bottom_margin_inch: f64,
    pub groups: BTreeMap<usize, Vec<Vec<Rectangle>>>,
}
impl Default for PageMaster {
    fn default() -> Self {
        Self {
            columns: vec![],
            page_width_pixels: 0.0,
            page_height_pixels: 0.0,
            dpi: 0.0,
            top_margin_inch: 0.0,
            bottom_margin_inch: 0.0,
            groups: Default::default(),
        }
    }
}
impl PageMaster {
    pub fn get_page_cnt(&self) -> usize {
        let (largest_page, _) = self.get_largest_for_range(0..100);
        largest_page
    }
    pub fn get_column_len(&self) -> usize {
        self.columns.len()
    }
    pub fn get_page_info(&self) -> (f64, f64, f64) {
        (self.page_width_pixels, self.page_height_pixels, self.dpi)
    }
    pub fn new(
        page_width_pixels: f64,
        page_height_pixels: f64,
        dpi: f64,
        top_margin_inch: f64,
        bottom_margin_inch: f64,
    ) -> Self {
        Self {
            columns: vec![(0, top_margin_inch * dpi); 100],
            page_height_pixels,
            page_width_pixels,
            dpi,
            top_margin_inch,
            bottom_margin_inch,
            groups: Default::default(),
        }
    }
    pub fn get_margins_pixels(&self) -> PageMargins {
        PageMargins {
            top_margin_pixels: self.top_margin_inch * self.dpi,
            bottom_margin_pixels: self.bottom_margin_inch * self.dpi,
        }
    }
    pub fn set_group(&mut self, group_id: Option<usize>, rec: &PlacementInfo) {
        if let Some(id) = group_id {
            self.groups
                .entry(id)
                .and_modify(|x| {
                    if rec.page_number >= x.len() {
                        x.resize(rec.page_number + 1, Vec::default());
                    }
                    x[rec.page_number].push(rec.rec)
                })
                .or_insert(Vec::new());
        }
    }
    /*
        pub fn get_group_vertical_lines(&self) {
            let lines: Vec<Vec<(f64, f64, f64, f64)>> = Vec::new();
            for group in &self.groups {
                if !group.1.is_empty() {
                    for page_index in 0..group.1.len() {
                        let page = &group.1[page_index];
                        for rec in page {

                        }
                    }
                }
            }
        }
    */
    pub fn get_group_borders(&self) -> BTreeMap<usize, Vec<Rectangle>> {
        let mut ret: BTreeMap<usize, Vec<Rectangle>> = BTreeMap::new();
        for group in &self.groups {
            if !group.1.is_empty() {
                for page_index in 0..group.1.len() {
                    let page = &group.1[page_index];

                    let smallest_x = page
                        .iter()
                        .map(|x| x.x)
                        .fold(f64::INFINITY, |x, y| f64::min(x, y));
                    let largest_x = page
                        .iter()
                        .map(|x| x.x + x.width)
                        .fold(0.0, |x, y| f64::max(x, y));
                    let smallest_y = page
                        .iter()
                        .map(|x| x.y)
                        .fold(f64::INFINITY, |x, y| f64::min(x, y));
                    let largest_y = page
                        .iter()
                        .map(|x| x.y + x.height)
                        .fold(0.0, |x, y| f64::max(x, y));

                    let page_rec = Rectangle::new(
                        smallest_x,
                        smallest_y,
                        largest_x - smallest_x,
                        largest_y - smallest_y,
                    );
                    ret.entry(*group.0)
                        .and_modify(|x| {
                            if page_index >= x.len() {
                                x.resize(page_index + 1, Rectangle::default())
                            }
                            x[page_index] = Rectangle::new(
                                page_rec.x,
                                page_rec.y,
                                page_rec.width,
                                page_rec.height,
                            );
                        })
                        .or_insert(vec![page_rec]);
                }
            }
        }

        ret
    }

    pub fn get_next_top_position(
        &mut self,
        options: PlacementOptions,
        range: Range<usize>,
    ) -> PlacementInfo {
        let (mut largest_page, mut largest_height_on_largest_page) =
            self.get_largest_for_range(range.clone());
        let mut is_new_page = false;
        if options.move_to_next_page {
            largest_page = largest_page + 1;
            is_new_page = true;
            largest_height_on_largest_page = self.top_margin_inch * self.dpi;
        } else {
            if largest_height_on_largest_page < self.top_margin_inch * self.dpi {
                is_new_page = true;
                largest_height_on_largest_page = self.top_margin_inch * self.dpi;
            }
        }
        let draw_height_left_on_page = self.page_height_pixels
            - self.bottom_margin_inch * self.dpi
            - (largest_height_on_largest_page);

        PlacementInfo {
            rec: Rectangle::new(
                self.page_width_pixels * (range.start as f64 / 100.0),
                largest_height_on_largest_page,
                self.page_width_pixels * ((range.end - range.start) as f64 / 100.0),
                0.0,
            ),
            draw_height_left_on_page,
            page_number: largest_page,
            page_size_info: PageSizeInfo::new(
                self.page_width_pixels,
                self.page_height_pixels,
                self.dpi,
                self.top_margin_inch * self.dpi,
                self.bottom_margin_inch * self.dpi,
            ),
            percent_range: range,
            restricted_area_option: None,
            is_new_page,
        }
    }

    pub fn get_largest_for_range(&self, range: Range<usize>) -> (usize, f64) {
        let largest_page = self.columns[range.clone()]
            .iter()
            .max_by_key(|x| x.0)
            .unwrap()
            .0;

        let largest_height_on_largest_page = self.columns[range.clone()]
            .iter()
            .filter(|x| x.0 == largest_page)
            .map(|x| x.1)
            .fold(0.0, |x, y| f64::max(x, y));

        (largest_page, largest_height_on_largest_page)
    }

    pub fn update_placement(&mut self, placement_info: &PlacementInfo) {
        for (page, height) in self.columns[placement_info.percent_range.clone()].iter_mut() {
            *page = placement_info.page_number;
            *height = placement_info.rec.y + placement_info.rec.height;
        }
    }
}
