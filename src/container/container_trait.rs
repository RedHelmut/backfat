use crate::container::placement_info::PlacementInfo;
use crate::container::rectangle::Border;
use std::cell::RefCell;
use lopdf::content::Operation;

pub trait GroupTracker {}

pub trait DrawInfoReq {
    fn increment_page_buffer(&mut self, page_number: usize);
    fn page_array_size(&self) -> usize;
    fn insert_into_page(&mut self, page_num: usize, operation: Operation);
}

pub trait ContainerTrait {
    fn on_draw<T: DrawInfoReq>(
        &mut self,
        placement_info: PlacementInfo,
        draw_info: &mut T,
        borders: &Option<RefCell<Vec<Border>>>,
    ) -> Option<PlacementInfo>;
    fn get_group(&self) -> Option<usize>;
}
