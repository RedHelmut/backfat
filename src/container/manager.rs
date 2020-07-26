use crate::container::container_trait::{ContainerTrait, DrawInfoReq};
use crate::container::page_master::{PageMargins, PageMaster};
use crate::container::page_size_info::PageSizeInfo;
use crate::container::placement_info::{PlacementInfo, PlacementOptions};
use crate::container::rectangle::{Border, Rectangle};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::sync::{Arc, Mutex};

pub struct LocationInfo {
    pub range: Range<usize>,
}

pub struct Manager {
    //   container: VecDeque<(LocationInfo, Box<dyn ContainerTrait<T>>)>,
    page_master: Arc<Mutex<PageMaster>>,
}

impl Manager {
    pub fn get_page_info(&self) -> PageSizeInfo {
        let info = self.page_master.lock().unwrap();
        let page_info = info.get_page_info();
        let margin_info = info.get_margins_pixels();
        PageSizeInfo::new(
            page_info.0,
            page_info.1,
            page_info.2,
            margin_info.top_margin_pixels,
            margin_info.bottom_margin_pixels,
        )
    }
    pub fn get_groups(&self) -> BTreeMap<usize, Vec<Rectangle>> {
        self.page_master.lock().unwrap().get_group_borders()
    }
    pub fn new(
        width_inches: f64,
        height_inches: f64,
        dpi: f64,
        top_margin_inch: f64,
        bottom_margin_inch: f64,
    ) -> Self {
        Self {
            //    container: VecDeque::new(),
            page_master: Arc::new(Mutex::new(PageMaster::new(
                width_inches * dpi,
                height_inches * dpi,
                dpi,
                top_margin_inch,
                bottom_margin_inch,
            ))),
        }
    }
    pub fn get_page_pixel_dims(&self) -> (f64, f64, f64) {
        self.page_master.lock().unwrap().get_page_info()
    }
    pub fn get_page_pixel_margins(&self) -> PageMargins {
        self.page_master.lock().unwrap().get_margins_pixels()
    }

    pub fn get_placement_handle(
        &mut self,
        range: Range<usize>,
        start_new_page: bool,
    ) -> CurrentPlacement {
        CurrentPlacement::new(&self.page_master, Some(range), start_new_page)
    }
    /*pub fn alter_placement_by_options(&mut self, info:PlacementInfo, opts: PlacementOptions ) -> PlacementInfo {

        self.page_master.update( opts, info.percent_range )
    }*/
    pub fn get_page_cnt(&self) -> usize {
        self.page_master.lock().unwrap().get_page_cnt()
    }
    pub fn place_now<T: DrawInfoReq, F: ContainerTrait>(&mut self, height_pixels: f64, range: Range<usize>, draw_info: &mut T, f: &mut F, border: &Option<RefCell<Vec<Border>>>) {
        let mut placement_handle = self.get_placement_handle( range, false );
        placement_handle.set_pixel_height(height_pixels);
        let draw_rec = placement_handle.placement_info.clone();
        if draw_rec.page_number >= draw_info.page_array_size() {
            draw_info.increment_page_buffer(draw_rec.page_number);
        }
        if let Some(last_placement) = f.on_draw(draw_rec, draw_info, border) {
            placement_handle.placement_info = last_placement;
        }

        self.page_master.lock().unwrap().set_group(f.get_group(), &placement_handle.placement_info);

        self.page_master
            .lock()
            .unwrap()
            .update_placement(&placement_handle.placement_info);
    }
}

//#[derive(Clone)]
pub struct CurrentPlacement {
    pm: Arc<Mutex<PageMaster>>,
    placement_info: PlacementInfo,
    placement_info_to_pass_for_draw: Option<PlacementInfo>,
}
impl Default for CurrentPlacement {
    fn default() -> Self {
        Self {
            pm: Arc::new(Mutex::new(PageMaster::default())),
            placement_info: Default::default(),
            placement_info_to_pass_for_draw: None,
        }
    }
}
impl Display for CurrentPlacement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Range({}..{})\r\nHeight: {}\r\nPage: {}\r\nTop: {}",
            self.placement_info.percent_range.start,
            self.placement_info.percent_range.end,
            self.placement_info.rec.height,
            self.placement_info.page_number,
            self.placement_info.rec.y
        )
    }
}
impl CurrentPlacement {
    pub fn new(
        page_master: &Arc<Mutex<PageMaster>>,
        range: Option<Range<usize>>,
        start_new_page: bool,
    ) -> Self {

        let mut rn = range.unwrap_or( 0..100 ).clone();
        let col_len = page_master.lock().unwrap().get_column_len();

        if rn.start > col_len {
            rn.start = 0;
        }

        if rn.end > col_len {
            rn.end = col_len - 1;
        }
        let placement_info =
            page_master
                .clone()
                .lock()
                .unwrap()
                .get_next_top_position(
                PlacementOptions {
                    move_to_next_page: start_new_page,
                    ignore: false,
                },rn
            ,
        );
        let this = Self {
            pm: page_master.clone(),
            placement_info,
            placement_info_to_pass_for_draw: None,
        };
        this
    }
    ///Updates the placement info if we change the height or location.
    fn update(&mut self, height: f64) {
        // let mut placement = self.pm.lock().unwrap().
        //   get_next_top_position(PlacementOptions{ move_to_next_page: false, ignore: false },
        //                       self.placement_info.percent_range.clone(), );

        if self.placement_info.draw_height_left_on_page >= height {
            self.placement_info.rec.height = height;
        } else {
            self.placement_info = self.pm.lock().unwrap().get_next_top_position(
                PlacementOptions {
                    move_to_next_page: true,
                    ignore: false,
                },
                self.placement_info.percent_range.clone(),
            );
            self.placement_info.rec.height = height;
        }
    }

    pub fn set_pixel_height(&mut self, size: f64) -> () {
        self.update(size);
    }

    pub fn get_placement_info(&mut self) -> PlacementInfo {
        self.placement_info.clone()
    }

    pub fn draw<F, T>(
        &mut self,
        f: &mut F,
        draw_info: &mut T,
        border: &Option<RefCell<Vec<Border>>>,
    ) -> ()
    where
        F: ContainerTrait,
        T: DrawInfoReq,
    {
        let draw_rec = if let Some(place) = self.placement_info_to_pass_for_draw.clone() {
            place
        } else {
            self.placement_info.clone()
        };
        if draw_rec.page_number >= draw_info.page_array_size() {
            draw_info.increment_page_buffer(draw_rec.page_number);
        }
        if let Some(last_placement) = f.on_draw(draw_rec, draw_info, border) {
            self.placement_info = last_placement;
        }

        self.pm
            .lock()
            .unwrap()
            .set_group(f.get_group(), &self.placement_info);

        self.pm
            .lock()
            .unwrap()
            .update_placement(&self.placement_info);
    }
    pub fn set_restricted_interior(
        &mut self,
        shrink_top: f64,
        shrink_left: f64,
        shrink_right: f64,
        shrink_bottom: f64,
    ) {
        let c_info = self.placement_info.clone();
        let area_opt = Rectangle::new(
            c_info.rec.x + shrink_left,
            c_info.rec.y + shrink_top,
            c_info.rec.width - shrink_left - shrink_right,
            c_info.rec.height - shrink_top - shrink_bottom,
        );
        self.placement_info.restricted_area_option = Some(area_opt);
    }
}
