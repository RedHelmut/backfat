use crate::container::container_trait::DrawInfoReq;
use lopdf::content::Operation;

pub mod lines;
pub mod list_box;
pub mod text_box;

pub struct PdfDrawInfo {
    pub pdf: Vec<Vec<Operation>>,
}
impl DrawInfoReq for PdfDrawInfo {
    fn increment_page_buffer(&mut self, page_number: usize) {
        if page_number >= self.page_array_size() {
            self.pdf.resize(page_number + 1, Vec::new());
        }
    }

    fn page_array_size(&self) -> usize {
        self.pdf.len()
    }
}
