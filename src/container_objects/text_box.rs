use crate::container::container_trait::{ContainerTrait, DrawInfoReq};
use lopdf::content::Operation;
use lopdf::Object;
//use crate::container_objects::lines::*;
use crate::container::page_size_info::PageSizeInfo;
use crate::container::placement_info::PlacementInfo;
use crate::container::rectangle::{Border, Rectangle};
use crate::container_objects::lines::draw_rectangle;
use crate::font::font_info::FontInfo;
use crate::font::font_sizes;
use crate::font::font_sizes::Font;
use std::cell::RefCell;

#[derive(Clone)]
pub enum BorderStyle {
    None,
    Single(f64),
}

#[derive(Clone, PartialOrd, PartialEq)]
pub enum TextAlignment {
    LeftTop,
    LeftCenter,
    LeftBottom,
    LeftJustifyTop(f64),
    LeftJustifyCenter(f64),
    LeftJustifyBottom(f64),
    RightTop,
    RightCenter,
    RightBottom,
    RightJustifyTop(f64),
    RightJustifyCenter(f64),
    RightJustifyBottom(f64),
    CenterTop,
    CenterCenter,
    CenterBottom,
}

pub struct TextBox {
    text: String,
    font: FontInfo,
    border_style: BorderStyle,
    background: (f64, f64, f64),
    alignment: TextAlignment,
    compensate_for_font_decent: bool,
    group: Option<usize>,
}

impl TextBox {
    pub fn new<T: ToString>(
        text: T,
        font: FontInfo,
        alignment: Option<TextAlignment>,
        border_style: Option<BorderStyle>,
        background: Option<(f64, f64, f64)>,
        group: Option<usize>,
    ) -> Self {
        Self {
            text: text.to_string(),
            font,
            border_style: border_style.unwrap_or(BorderStyle::None),
            background: background.unwrap_or((1.0, 1.0, 1.0)),
            alignment: alignment.unwrap_or(TextAlignment::LeftBottom),
            compensate_for_font_decent: false,
            group,
        }
    }
    pub fn set_background(&mut self, back_ground: (f64, f64, f64)) {
        self.background = back_ground;
    }
    pub fn compensate_for_font_decent(&mut self, should_it: bool) {
        self.compensate_for_font_decent = should_it;
    }

    fn adjust_for_font_text_alignment(
        text: &String,
        text_draw_info: &Rectangle,
        font: &FontInfo,
        alignment: &TextAlignment,
        page_info: &PageSizeInfo,
        compensate_decent: bool,
    ) -> (f64, f64) {
        let widths = &font_sizes::GLYPH_WIDTHS[&font.font];

        //   &font_sizes::GLYPH_WIDTHS.iter();

        let line_width = text
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| *widths.get(&c).unwrap_or(&1.0))
            .sum::<f64>()
            * font.size;

        let up_to_fit_in_box = match compensate_decent {
            true => {
                let decent = &font_sizes::FONT_DESCENT[&font.font];
                decent * (font.size / 2048.0)
            }
            false => 0.0,
        };

        match alignment {
            TextAlignment::LeftTop => (
                text_draw_info.x,
                text_draw_info.height - font.size + up_to_fit_in_box,
            ),
            TextAlignment::LeftCenter => (
                text_draw_info.x,
                text_draw_info.height / 2.0 - font.size / 2.0 + up_to_fit_in_box,
            ),
            TextAlignment::LeftBottom => (text_draw_info.x, up_to_fit_in_box),
            TextAlignment::LeftJustifyTop(amt) => (
                text_draw_info.x + amt * page_info.dpi,
                text_draw_info.height - font.size + up_to_fit_in_box,
            ),
            TextAlignment::LeftJustifyCenter(amt) => (
                text_draw_info.x + amt * page_info.dpi,
                text_draw_info.height / 2.0 - font.size / 2.0 + up_to_fit_in_box,
            ),
            TextAlignment::LeftJustifyBottom(amt) => {
                (text_draw_info.x + amt * page_info.dpi, up_to_fit_in_box)
            }
            TextAlignment::CenterTop => (
                text_draw_info.x + text_draw_info.width / 2.0 - line_width / 2.0,
                text_draw_info.height - font.size + up_to_fit_in_box,
            ),
            TextAlignment::CenterCenter => (
                text_draw_info.x + text_draw_info.width / 2.0 - line_width / 2.0,
                text_draw_info.height / 2.0 - font.size / 2.0 + up_to_fit_in_box,
            ),
            TextAlignment::CenterBottom => (
                text_draw_info.x + text_draw_info.width / 2.0 - line_width / 2.0,
                up_to_fit_in_box,
            ),
            TextAlignment::RightTop => (
                text_draw_info.x + text_draw_info.width - line_width,
                text_draw_info.height - font.size + up_to_fit_in_box,
            ),
            TextAlignment::RightCenter => (
                text_draw_info.x + text_draw_info.width - line_width,
                text_draw_info.height / 2.0 - font.size / 2.0 + up_to_fit_in_box,
            ),
            TextAlignment::RightBottom => (
                text_draw_info.x + text_draw_info.width - line_width,
                up_to_fit_in_box,
            ),
            TextAlignment::RightJustifyTop(pt) => (
                text_draw_info.x - pt * page_info.dpi + text_draw_info.width - line_width,
                text_draw_info.height - font.size + up_to_fit_in_box,
            ),
            TextAlignment::RightJustifyCenter(pt) => (
                text_draw_info.x - pt * page_info.dpi + text_draw_info.width - line_width,
                text_draw_info.height / 2.0 - font.size / 2.0 + up_to_fit_in_box,
            ),
            TextAlignment::RightJustifyBottom(pt) => (
                text_draw_info.x - pt * page_info.dpi + text_draw_info.width - line_width,
                up_to_fit_in_box,
            ),
        }
    }
}
impl Default for TextBox {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            font: FontInfo {
                font: Font::TimesRoman,
                size: 12.0,
                font_color: (0.0, 0.0, 0.0),
            },
            border_style: BorderStyle::None,
            background: (1.0, 1.0, 1.0),
            alignment: TextAlignment::LeftTop,
            compensate_for_font_decent: false,
            group: None,
        }
    }
}

impl ContainerTrait for TextBox {
    fn on_draw<T: DrawInfoReq>(
        &mut self,
        placement_info: PlacementInfo,
        draw_to: &mut T,
        borders: &Option<RefCell<Vec<Border>>>,
    ) -> Option<PlacementInfo> {
        let text_color = self.font.font_color;

        let outer_rec = placement_info.get_outer_rec();
        let text_draw_info = placement_info
            .restricted_area_option
            .clone()
            .unwrap_or(outer_rec)
            .get_pdf_version(placement_info.page_size_info.clone());

        let (start_x, start_y) = Self::adjust_for_font_text_alignment(
            &self.text,
            &text_draw_info,
            &self.font,
            &self.alignment,
            &placement_info.page_size_info,
            false,
        );

        draw_to.insert_into_page(placement_info.page_number, Operation::new("q", vec![]));
        draw_to.insert_into_page(placement_info.page_number, Operation::new("re", text_draw_info.clone().into()));
        draw_to.insert_into_page(placement_info.page_number, Operation::new("W", vec![]));
        draw_to.insert_into_page(placement_info.page_number, Operation::new("n", vec![]));

        draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
        draw_to.insert_into_page(placement_info.page_number, Operation::new(
            "rg",
            vec![
                self.background.0.into(),
                self.background.1.into(),
                self.background.2.into(),
            ],
        )); //stroke color
        draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
        draw_to.insert_into_page(placement_info.page_number, Operation::new(
            "SC",
            vec![
                self.background.0.into(),
                self.background.1.into(),
                self.background.2.into(),
            ],
        ));
        //draw rectangle color for column
        draw_to.insert_into_page(placement_info.page_number, Operation::new("re", text_draw_info.clone().into()));
        draw_to.insert_into_page(placement_info.page_number, Operation::new("f", vec![]));

        //border here
        // let rec_text = vec![(text_draw_info.left_pixel_x).into(),text_draw_height.into(), (text_draw_info.pixels_x_width ).into(), (text_draw_info.object_pixel_height).into()];

        draw_to.insert_into_page(placement_info.page_number, Operation::new("re", text_draw_info.clone().into()));
        draw_to.insert_into_page(placement_info.page_number, Operation::new("W", vec![]));
        draw_to.insert_into_page(placement_info.page_number, Operation::new("n", vec![]));

        draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
        draw_to.insert_into_page(placement_info.page_number, Operation::new(
            "rg",
            vec![
                text_color.0.into(),
                text_color.1.into(),
                text_color.2.into(),
            ],
        )); //stroke color
        draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
        draw_to.insert_into_page(placement_info.page_number, Operation::new(
            "SC",
            vec![
                text_color.0.into(),
                text_color.1.into(),
                text_color.2.into(),
            ],
        ));

        draw_to.insert_into_page(placement_info.page_number, Operation::new("BT", vec![]));

        //get F1 or whatever for font
        let fnt = font_sizes::CROSS_FONT_PDF[&self.font.font].clone();

        draw_to.insert_into_page(placement_info.page_number, Operation::new(
            "Tf",
            vec![fnt.into(), self.font.size.into()],
        ));
        draw_to.insert_into_page(placement_info.page_number, Operation::new(
            "Td",
            vec![
                start_x.into(),                      //placement_info.left_pixel_x.into(),
                (start_y + text_draw_info.y).into(), //(real_height + font_break.1 * placement_info.dpi).into(),
            ],
        ));
        draw_to.insert_into_page(placement_info.page_number, Operation::new(
            "Tj",
            vec![Object::string_literal(self.text.clone())],
        ));
        draw_to.insert_into_page(placement_info.page_number, Operation::new("ET", vec![]));
        /*
        {
            if let Some(ref restr) = placement_info.restricted_area_option {
                let by_amt = 0.0;
                let inner_rec = vec![(restr.left + by_amt).into(), (text_draw_height + by_amt).into(), (restr.width - by_amt * 2.0).into(), (restr.height - by_amt * 2.0).into()];
                {
                    let border_color = (1.0,0.0,0.0);
                    draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
                    draw_to.insert_into_page(placement_info.page_number, Operation::new(
                        "rg",
                        vec![border_color.0.into(), border_color.1.into(), border_color.2.into()],
                    )); //stroke color
                    draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
                    draw_to.insert_into_page(placement_info.page_number, Operation::new(
                        "SC",
                        vec![border_color.0.into(), border_color.1.into(), border_color.2.into()],
                    ));

                   // rec = vec![(placement_info.left_pixel_x + size / 2.0).into(),(real_height + size / 2.0).into(), (placement_info.pixels_x_width - size).into(), (placement_info.object_pixel_height - size).into()];

                    draw_to.insert_into_page(placement_info.page_number, Operation::new("w", vec![1.0.into()]));
                    draw_to.insert_into_page(placement_info.page_number, Operation::new(
                        "re",
                        inner_rec,
                    ));
                    draw_to.insert_into_page(placement_info.page_number, Operation::new("s", vec![]));

                }
            }


        }*/
        draw_to.insert_into_page(placement_info.page_number, Operation::new("Q", vec![]));

        match self.border_style {
            BorderStyle::Single(size) => match borders {
                None => {
                    draw_rectangle(draw_to, &placement_info, size, (0.0, 0.0, 0.0));
                }
                Some(border) => {
                    border
                        .borrow_mut()
                        .push(Border::new(placement_info, (0.0, 0.0, 0.0), size));
                }
            },
            BorderStyle::None => {}
        };

        None
    }

    fn get_group(&self) -> Option<usize> {
        self.group
    }
}
