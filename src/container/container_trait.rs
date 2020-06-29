use crate::container::placement_info::PlacementInfo;
use crate::container::rectangle::Border;
use std::cell::RefCell;

pub trait GroupTracker {}

pub trait DrawInfoReq {
    fn increment_page_buffer(&mut self, page_number: usize);
    fn page_array_size(&self) -> usize;
}

pub trait ContainerTrait<T: DrawInfoReq> {
    fn on_draw(
        &mut self,
        placement_info: PlacementInfo,
        draw_info: &mut T,
        borders: &Option<RefCell<Vec<Border>>>,
    ) -> Option<PlacementInfo>;
    fn get_group(&self) -> Option<usize>;
}
