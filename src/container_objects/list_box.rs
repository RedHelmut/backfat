use crate::container::container_trait::ContainerTrait;
use crate::container::manager::{CurrentPlacement, Manager};
use crate::container::page_size_info::PageSizeInfo;
use crate::container::placement_info::PlacementInfo;
use crate::container::rectangle::{Border, Rectangle};
use crate::container_objects::lines::*;
use crate::container_objects::text_box::*;
use crate::container_objects::PdfDrawInfo;
use crate::font::font_info::FontInfo;
use crate::font::font_sizes::Font;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Range;

pub enum ListBoxBorder {
    Inner(f64),
    Outer(f64),
    All(f64, f64),
    None,
}

#[derive(Clone)]
pub enum TypeOfItem {
    Currency(usize),
    Number(usize),
    String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum RowPositionType {
    Item,
    Top,
    Header(bool),
    Bottom,
    FullPageItem,
    Searching,
    ItemWithTopBorder,
}
impl Display for RowPositionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let txt = match self {
            RowPositionType::Searching => "Searching",
            RowPositionType::Top => "Top",
            RowPositionType::Bottom => "Bottom",
            RowPositionType::Item => "Item",
            RowPositionType::Header(_) => "Header",
            RowPositionType::ItemWithTopBorder => "ItemWithTopBorder",
            RowPositionType::FullPageItem => "FullPageItem",
        }
        .to_string();
        write!(f, "{}", txt)
    }
}

pub struct ListBox<'a> {
    list_data: ListData<'a>,
    manager: &'a mut Manager,
}
impl<'a> ContainerTrait<PdfDrawInfo> for ListBox<'a> {
    fn on_draw(
        &mut self,
        placement_info: PlacementInfo,
        pdf_draw: &mut PdfDrawInfo,
        borders: &Option<RefCell<Vec<Border>>>,
    ) -> Option<PlacementInfo> {
        if self.list_data.data.len() == 0 {
            return None;
        }
        let blank_string = "".to_owned();

        let (inner_border_size, outer_border_size) =
            Self::get_half_border_sizes(&self.list_data.list_box_border);

        //state variables
        let mut is_start_row: bool = true;
        let mut row_index = 0;
        let has_header = self.list_data.header.is_some();
        let mut dont_change_row = false;
        let mut index_on_page = 0;
        //utility variables?

        let mut why = placement_info.rec.y;
        let mut line_spaces: Vec<(f64, f64)> = Vec::new();

        //this just levels the top of the list box so always even
        let mut last_placement = self.place_level_line(
            &mut why,
            placement_info.percent_range.start,
            placement_info.percent_range.end - placement_info.percent_range.start,
            pdf_draw,
            borders,
        );
        let mut lines_to_ignore_top_border = 0;
        let mut no_more_under_row_on_page = false;
        let mut last_connected = false;
        let mut ignore_horizontal_line = false;
        let mut ignore_next_horizontal = false;
        while row_index < self.list_data.data.len() {
            let is_on_last_row = row_index == self.list_data.data.len() - 1;
            //row state variables for switching from header to list item.
            let mut this_row_font = self.list_data.list_item_font.clone();
            let mut this_row_color = self.get_row_color(index_on_page); //(1.0, 1.0, 1.0);
            let mut this_text_color = (0.0, 0.0, 0.0);
            let mut this_row_is_header = false;
            let mut first_row_placement = PlacementInfo::default();
            let mut last_row_placement = PlacementInfo::default();
            let mut column_width_total: usize = 0;
            let mut current_row = &self.list_data.data[row_index];

            let mut is_left_border = false;
            let mut is_right_border = false;

            for column_index in 0..self.list_data.column_widths_percentage.len() {
                is_left_border = false;
                is_right_border = false;

                if column_index == 0 {
                    is_left_border = true;
                } else if column_index == self.list_data.column_widths_percentage.len() - 1 {
                    is_right_border = true;
                }

                let mut no_top_border_exception = false;

                let mut current_column_start =
                    column_width_total + placement_info.percent_range.start;
                let mut current_column_end =
                    current_column_start + self.list_data.column_widths_percentage[column_index];
                column_width_total =
                    column_width_total + self.list_data.column_widths_percentage[column_index];

                let is_single_row = match current_row.param {
                    RowDataTypes::SingleNoBorder(_)
                    | RowDataTypes::Single(_)
                    | RowDataTypes::SingleWithColor(_,_)
                    | RowDataTypes::SingleNoBorderWithColor(_,_) => true,
                    _ => false,
                };

                let mut bpd = if is_single_row {
                    current_column_start = placement_info.percent_range.start;
                    current_column_end = placement_info.percent_range.end;
                    column_width_total =
                        placement_info.percent_range.end - placement_info.percent_range.start;
                    let mut bpd = self.get_position_border_info(
                        row_index,
                        current_column_start..current_column_end,
                        is_start_row,
                        is_on_last_row,
                        &mut index_on_page,
                    );

                    if is_single_row
                        && bpd.list_placement_handle.get_placement_info().is_new_page
                        && (bpd.row_type_and_size == RowPositionType::Header(true)
                            || bpd.row_type_and_size == RowPositionType::Header(false))
                    {
                        column_width_total = 0;
                        current_column_start =
                            column_width_total + placement_info.percent_range.start;
                        current_column_end = current_column_start
                            + self.list_data.column_widths_percentage[column_index];
                        column_width_total = column_width_total
                            + self.list_data.column_widths_percentage[column_index];
                        current_row = self.list_data.header.unwrap();
                        self.get_position_border_info(
                            row_index,
                            current_column_start..current_column_end,
                            is_start_row,
                            is_on_last_row,
                            &mut index_on_page,
                        )
                    } else {
                        bpd
                    }
                } else {
                    self.get_position_border_info(
                        row_index,
                        current_column_start..current_column_end,
                        is_start_row,
                        is_on_last_row,
                        &mut index_on_page,
                    )
                };

                if bpd.list_placement_handle.get_placement_info().is_new_page {
                    index_on_page = 0;
                }
                if column_index == 0 {
                    if self.list_data.exclude_border_on_header
                        && has_header
                        && !bpd.list_placement_handle.get_placement_info().is_new_page
                        && index_on_page > 0
                    {
                        this_row_color = self.get_row_color(index_on_page + 1); //(1.0, 1.0, 1.0);
                    } else {
                        this_row_color = self.get_row_color(index_on_page); //(1.0, 1.0, 1.0);
                    }
                }

                match bpd.row_type_and_size {
                    RowPositionType::Header(header_has_border) => {
                        current_row = self.list_data.header.unwrap();
                        this_row_is_header = true;
                        dont_change_row = true;
                        this_text_color = self.list_data.header_font.font_color;
                        this_row_font = self.list_data.header_font.clone();
                        if !header_has_border {
                            bpd.is_bottom_border = true;
                            bpd.is_top_border = false;
                            lines_to_ignore_top_border = 1;
                            no_top_border_exception = true;
                            match self.list_data.list_box_border {
                                ListBoxBorder::All(_, _) => {
                                    bpd.list_placement_handle.set_pixel_height(
                                        self.list_data.row_header_pixels + outer_border_size,
                                    );
                                }
                                ListBoxBorder::Outer(_) => {
                                    bpd.list_placement_handle.set_pixel_height(
                                        self.list_data.row_header_pixels + outer_border_size,
                                    );
                                }
                                ListBoxBorder::Inner(_) => {
                                    bpd.list_placement_handle.set_pixel_height(
                                        self.list_data.row_header_pixels + inner_border_size,
                                    );
                                }
                                ListBoxBorder::None => {}
                            }
                        } else {
                            bpd.list_placement_handle.set_pixel_height(
                                self.list_data.row_header_pixels
                                    + outer_border_size
                                    + inner_border_size,
                            );
                            self.list_data.page_top =
                                bpd.list_placement_handle.get_placement_info().rec.y;
                            //pdf_draw.list_box_page_height_info[bpd.list_placement_handle.get_placement_info().page_number] = (bpd.list_placement_handle.get_placement_info().rec.y, 0.0);
                            //           found_line_start = true;
                        }
                    }
                    RowPositionType::Top => {
                        bpd.list_placement_handle.set_pixel_height(
                            bpd.item_height_pixels + outer_border_size + inner_border_size,
                        );
                        //pdf_draw.list_box_page_height_info[bpd.list_placement_handle.get_placement_info().page_number] = (bpd.list_placement_handle.get_placement_info().rec.y, 0.0);
                        self.list_data.page_top =
                            bpd.list_placement_handle.get_placement_info().rec.y;
                    }
                    RowPositionType::Item => {
                        if bpd.is_top_border {
                            self.list_data.page_top =
                                bpd.list_placement_handle.get_placement_info().rec.y;
                        }
                        //
                        bpd.list_placement_handle.set_pixel_height(
                            bpd.item_height_pixels + inner_border_size + inner_border_size,
                        );
                    }
                    RowPositionType::Bottom => {
                        if bpd.is_top_border {
                            //                            pdf_draw.list_box_page_height_info[bpd.list_placement_handle.get_placement_info().page_number] = (bpd.list_placement_handle.get_placement_info().rec.y, 0.0);
                            bpd.list_placement_handle.set_pixel_height(
                                bpd.item_height_pixels + outer_border_size + outer_border_size,
                            );
                            if column_index == 0 {
                                let vs = VertSpacing::new(
                                    bpd.list_placement_handle.get_placement_info().page_number,
                                    bpd.list_placement_handle.get_placement_info().rec.y,
                                    bpd.list_placement_handle.get_placement_info().rec.y
                                        + bpd.list_placement_handle.get_placement_info().rec.height,
                                );
                                self.list_data.list_box_page_height_info.push(vs);
                            }
                        } else {
                            bpd.list_placement_handle.set_pixel_height(
                                bpd.item_height_pixels + outer_border_size + inner_border_size,
                            );
                            if column_index == 0 {
                                let vs = VertSpacing::new(
                                    bpd.list_placement_handle.get_placement_info().page_number,
                                    self.list_data.page_top,
                                    bpd.list_placement_handle.get_placement_info().rec.y
                                        + bpd.list_placement_handle.get_placement_info().rec.height,
                                );
                                self.list_data.list_box_page_height_info.push(vs);
                            }
                        }
                        no_more_under_row_on_page = true;
                        self.list_data.page_top = 0.0;
                    }
                    RowPositionType::ItemWithTopBorder => {
                        self.list_data.page_top =
                            bpd.list_placement_handle.get_placement_info().rec.y;
                        bpd.list_placement_handle.set_pixel_height(
                            bpd.item_height_pixels + outer_border_size + inner_border_size,
                        );
                    }
                    RowPositionType::FullPageItem => {
                        bpd.list_placement_handle.set_pixel_height(
                            bpd.item_height_pixels + outer_border_size + outer_border_size,
                        );
                        no_more_under_row_on_page = true;
                        if column_index == 0 {
                            let vs = VertSpacing::new(
                                bpd.list_placement_handle.get_placement_info().page_number,
                                bpd.list_placement_handle.get_placement_info().rec.y,
                                bpd.list_placement_handle.get_placement_info().rec.y
                                    + bpd.list_placement_handle.get_placement_info().rec.height,
                            );
                            self.list_data.list_box_page_height_info.push(vs);
                        }
                        self.list_data.page_top = 0.0;
                    }

                    RowPositionType::Searching => {
                        //Will never hit this.
                        unimplemented!()
                    }
                }

                // used to draw vertical lines
                if column_index == 0 {
                    if !(has_header
                        && self.list_data.exclude_border_on_header
                        && index_on_page == 0)
                    {
                        if is_single_row {
                            last_connected = false;
                        } else {
                            if last_connected {
                                if line_spaces.len() > 0 {
                                    let li = line_spaces.len() - 1;
                                    let v = line_spaces[li].1.clone();
                                    line_spaces[li].1 = v + bpd
                                        .list_placement_handle
                                        .get_placement_info()
                                        .rec
                                        .height;
                                } else {
                                    line_spaces.push((
                                        bpd.list_placement_handle.get_placement_info().rec.y,
                                        bpd.list_placement_handle.get_placement_info().rec.height,
                                    ));
                                }
                            } else {
                                line_spaces.push((
                                    bpd.list_placement_handle.get_placement_info().rec.y,
                                    bpd.list_placement_handle.get_placement_info().rec.height,
                                ));
                            }
                            last_connected = true;
                        }
                    }
                }
                //this is used to set the drawable area of this rectangle, used because of border overlap to ensure the vertical spacing is the same
                let (shrink_top, shrink_left, shrink_right, shrink_bottom) =
                    Self::adjust_for_border(
                        &mut bpd.list_placement_handle,
                        &self.list_data.list_box_border,
                        is_left_border,
                        bpd.is_top_border,
                        is_right_border,
                        bpd.is_bottom_border,
                        no_top_border_exception,
                    );
                bpd.list_placement_handle.set_restricted_interior(
                    shrink_top,
                    shrink_left,
                    shrink_right,
                    shrink_bottom,
                );
                let text_alignment = self.get_row_align(column_index, this_row_is_header);
                let col_data = if column_index < current_row.data.len() {
                    &current_row.data[column_index]
                } else {
                    &blank_string
                };
                let (display_text, text_color) = Self::format_column(
                    &self.list_data.types_of_items[column_index],
                    col_data,
                    this_text_color,
                    this_row_is_header,
                );
                let mut font_i = this_row_font.clone();
                let (color,align) = if let RowDataTypes::SingleWithColor(col, align) = &current_row.param {
                    (col.clone(),align.clone())
                } else if let RowDataTypes::SingleNoBorderWithColor(col, align) = &current_row.param {
                    (col.clone(),align.clone())
                } else {
                    (text_color.clone(),text_alignment.clone())
                };
                font_i.font_color = color.to_owned();
                //font_i.size = 6.0;
                let mut text_box = if this_row_is_header && self.list_data.exclude_border_on_header
                {
                    TextBox::new(
                        display_text,
                        font_i,
                        Some(align.clone()),
                        None,
                        Some(this_row_color),
                        None,
                    )
                } else {
                    TextBox::new(
                        display_text,
                        font_i,
                        Some(align.clone()),
                        None,
                        Some(this_row_color),
                        self.list_data.group,
                    )
                };
                bpd.list_placement_handle
                    .draw(&mut text_box, pdf_draw, borders);

                //first_row_placement, last_row_placement are to draw the horizontal line
                if column_index == 0 {
                    first_row_placement = bpd.list_placement_handle.get_placement_info();
                } else if column_index == self.list_data.column_widths_percentage.len() - 1 {
                    last_row_placement = bpd.list_placement_handle.get_placement_info();
                }

                //last placement is to draw the vertical lines and rectangle border
                if column_index == self.list_data.column_widths_percentage.len() - 1 {
                    last_placement = bpd.list_placement_handle.get_placement_info();
                }

                if column_index == 0 {
                    match current_row.param {
                        RowDataTypes::Single(_) | RowDataTypes::SingleWithColor(_,_) => {
                            last_placement = bpd.list_placement_handle.get_placement_info();
                            last_row_placement = bpd.list_placement_handle.get_placement_info();
                            if ignore_next_horizontal {
                                ignore_horizontal_line = true;
                            } else {
                                ignore_horizontal_line = false;
                            }
                            ignore_next_horizontal = false;
                            break;
                        }
                        RowDataTypes::Normal => {
                            if ignore_next_horizontal {
                                ignore_horizontal_line = true;
                            } else {
                                ignore_horizontal_line = false;
                            }
                            ignore_next_horizontal = false;
                        }
                        RowDataTypes::SingleNoBorder(_) | RowDataTypes::SingleNoBorderWithColor(_,_) => {
                            last_placement = bpd.list_placement_handle.get_placement_info();
                            last_row_placement = bpd.list_placement_handle.get_placement_info();
                            ignore_horizontal_line = true;
                            ignore_next_horizontal = true;
                            break;
                        }
                    }
                }
            } // end current row

            //increment color line per page
            is_start_row = false;

            //draw horizontal lines
            if index_on_page > lines_to_ignore_top_border && !ignore_horizontal_line {
                //so we dont draw on top of box with this function, save for outer rectangle
                lines_to_ignore_top_border = 0;
                match self.list_data.list_box_border {
                    ListBoxBorder::All(inner_size, _) => {
                        draw_horizontal_line(
                            pdf_draw,
                            first_row_placement.page_number,
                            first_row_placement.rec.y,
                            first_row_placement.rec.x, // + outer_size,
                            last_row_placement.rec.x + last_row_placement.rec.width
                                - first_row_placement.rec.x,
                            first_row_placement.page_size_info.page_height_pixels,
                            inner_size,
                            self.list_data.border_color,
                        );
                    }
                    ListBoxBorder::Inner(inner_size) => {
                        draw_horizontal_line(
                            pdf_draw,
                            first_row_placement.page_number,
                            first_row_placement.rec.y,
                            first_row_placement.rec.x,
                            last_row_placement.rec.x + last_row_placement.rec.width
                                - first_row_placement.rec.x,
                            first_row_placement.page_size_info.page_height_pixels,
                            inner_size,
                            self.list_data.border_color,
                        );
                    }
                    _ => {}
                }
            }

            if no_more_under_row_on_page {
                //draw vertical lines
                for lines in line_spaces.clone() {
                    let mut column_width_total: usize = 0;

                    for ci in 0..self.list_data.column_widths_percentage.len() {
                        let current_column_start =
                            column_width_total + placement_info.percent_range.start;
                        column_width_total =
                            column_width_total + self.list_data.column_widths_percentage[ci];

                        if ci > 0 {
                            match self.list_data.list_box_border {
                                ListBoxBorder::All(inner_size, _) => {
                                    draw_vertical_line(
                                        pdf_draw,
                                        last_row_placement.page_number,
                                        lines.0, // putting -0.5 got rid of artifact
                                        current_column_start as f64 / 100.0
                                            * self.manager.get_page_pixel_dims().0,
                                        lines.1,
                                        last_row_placement.page_size_info.page_height_pixels,
                                        inner_size,
                                        self.list_data.border_color,
                                    );
                                }
                                ListBoxBorder::Inner(inner_size) => {
                                    draw_vertical_line(
                                        pdf_draw,
                                        last_row_placement.page_number,
                                        lines.0, // putting -0.5 got rid of artifact
                                        current_column_start as f64 / 100.0
                                            * self.manager.get_page_pixel_dims().0,
                                        lines.1,
                                        last_row_placement.page_size_info.page_height_pixels,
                                        inner_size,
                                        self.list_data.border_color,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    line_spaces = Vec::new();
                }
            }

            if !dont_change_row {
                row_index = row_index + 1;
            }
            dont_change_row = false;

            index_on_page = index_on_page + 1;
        } // end row

        match self.list_data.list_box_border {
            ListBoxBorder::All(_, border_size) | ListBoxBorder::Outer(border_size) => {
                //draw border, outer
                for i in 0..self.list_data.list_box_page_height_info.len() {
                    if self.list_data.list_box_page_height_info[i].top > 0.0000001
                        && self.list_data.list_box_page_height_info[i].bottom > 0.00000001
                    {
                        let pl = PlacementInfo {
                            rec: Rectangle::new(
                                placement_info.percent_range.start as f64 / 100.0
                                    * self.manager.get_page_pixel_dims().0,
                                self.list_data.list_box_page_height_info[i].top,
                                (placement_info.percent_range.end
                                    - placement_info.percent_range.start)
                                    as f64
                                    / 100.0
                                    * self.manager.get_page_pixel_dims().0,
                                self.list_data.list_box_page_height_info[i].bottom
                                    - self.list_data.list_box_page_height_info[i].top,
                            ),

                            is_new_page: false,
                            draw_height_left_on_page: 0.0,
                            page_number: self.list_data.list_box_page_height_info[i].page,
                            percent_range: placement_info.percent_range.clone(),
                            restricted_area_option: None,
                            page_size_info: PageSizeInfo::new(
                                last_placement.page_size_info.page_width_pixels,
                                last_placement.page_size_info.page_height_pixels,
                                last_placement.page_size_info.dpi,
                                last_placement.page_size_info.top_margin,
                                last_placement.page_size_info.bottom_margin,
                            ),
                        };
                        match borders {
                            None => {
                                draw_rectangle(
                                    pdf_draw,
                                    &pl,
                                    border_size,
                                    self.list_data.border_color,
                                );
                            }
                            Some(rec) => {
                                rec.borrow_mut().push(Border::new(
                                    pl,
                                    self.list_data.border_color,
                                    border_size,
                                ));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Some(last_placement)
    }

    fn get_group(&self) -> Option<usize> {
        self.list_data.group
    }
}
#[derive(PartialOrd, PartialEq)]
pub enum RowDataTypes {
    Normal,
    Single(TextAlignment),
    SingleNoBorder(TextAlignment),
    SingleWithColor((f64, f64, f64),TextAlignment),
    SingleNoBorderWithColor((f64, f64, f64), TextAlignment),
}
impl Default for RowDataTypes {
    fn default() -> Self {
        RowDataTypes::Normal
    }
}
pub struct RowData {
    pub data: Vec<String>,
    pub param: RowDataTypes,
}
impl RowData {
    pub fn new(data: Vec<String>, param: RowDataTypes) -> Self {
        Self { data, param }
    }
}
struct VertSpacing {
    page: usize,
    top: f64,
    bottom: f64,
}
impl VertSpacing {
    pub fn new(page: usize, top: f64, bottom: f64) -> Self {
        Self { page, top, bottom }
    }
}
struct ListData<'a> {
    data: &'a Vec<RowData>,
    column_widths_percentage: Vec<usize>,
    header: Option<&'a RowData>,
    header_column_text_alignments: Option<Vec<TextAlignment>>,
    alternate_row_colors: Option<((f64, f64, f64), (f64, f64, f64))>,
    column_text_alignments: Option<Vec<TextAlignment>>,
    list_box_border: ListBoxBorder,
    exclude_border_on_header: bool,
    types_of_items: Vec<TypeOfItem>,
    border_color: (f64, f64, f64),
    header_font: FontInfo,
    list_item_font: FontInfo,
    row_cell_size: Option<Vec<f64>>,
    row_header_pixels: f64,
    inner_border_size: f64,
    outer_border_size: f64,
    group: Option<usize>,
    list_box_page_height_info: Vec<VertSpacing>,
    page_top: f64,
}
impl<'a> ListBox<'a> {
    pub fn new(
        data: &'a Vec<RowData>,
        column_widths_percentage: Vec<usize>,
        header: Option<&'a RowData>,
        manager: &'a mut Manager,
        list_item_font: FontInfo,
        header_font: FontInfo,
        border: ListBoxBorder,
        group: Option<usize>,
    ) -> Self {
        let (inner_border_size, outer_border_size) = Self::get_half_border_sizes(&border);
        let margins = manager.get_page_pixel_margins();
        let page_size_info = manager.get_page_pixel_dims();
        let mut row_header_pixels =
            0.0159708658854167 * header_font.size * manager.get_page_pixel_dims().2; //get_font_breakdown(self.list_data.header_font.size).2 * self.manager.get_page_pixel_dims().2 * 1.2;
        row_header_pixels = row_header_pixels * 1.2;

        if row_header_pixels
            > page_size_info.1
                - margins.top_margin_pixels
                - margins.bottom_margin_pixels
                - outer_border_size * 4.0
        {
            row_header_pixels = page_size_info.1
                - margins.top_margin_pixels
                - margins.bottom_margin_pixels
                - outer_border_size * 4.0;
        }

        let col_wdth = column_widths_percentage.len();
        Self {
            list_data: ListData {
                data,
                column_widths_percentage,
                header,
                header_column_text_alignments: None,
                alternate_row_colors: Some(((1.0, 1.0, 1.0), (0.9, 1.0, 1.0))),
                column_text_alignments: None,
                list_box_border: border,
                exclude_border_on_header: false,
                types_of_items: vec![TypeOfItem::String; col_wdth], //FIX
                border_color: (0.0, 0.0, 0.0),
                header_font,
                list_item_font,
                row_cell_size: None,
                row_header_pixels,
                inner_border_size,
                outer_border_size,
                group,
                list_box_page_height_info: vec![],
                page_top: 0.0,
            },
            manager,
        }
    }

    //this row, next row
    fn get_item_height_pixels(&mut self, row_index: usize) -> (f64, f64) {
        let (_, page_height, _) = self.manager.get_page_pixel_dims();
        let margins = self.manager.get_page_pixel_margins();
        let (mut item_height_pixels, mut next_juan) = match &self.list_data.row_cell_size {
            Some(row_sizes) => (
                row_sizes[row_index],
                row_sizes
                    .get(row_index + 1)
                    .unwrap_or(&row_sizes[row_index])
                    .to_owned() as f64,
            ),
            None => {
                let mut item_height_pixels = 0.0159708658854167
                    * self.list_data.list_item_font.size
                    * self.manager.get_page_pixel_dims().2; //13.798828125;//get_font_breakdown(self.list_data.list_item_font.size).2 * self.manager.get_page_pixel_dims().2 * 1.2;
                item_height_pixels = item_height_pixels * 1.2;

                (item_height_pixels, item_height_pixels)
            }
        };

        if item_height_pixels
            > page_height
                - margins.top_margin_pixels
                - margins.bottom_margin_pixels
                - self.list_data.outer_border_size * 2.0
                - self.list_data.inner_border_size * 2.0
        {
            item_height_pixels = page_height
                - margins.top_margin_pixels
                - margins.bottom_margin_pixels
                - self.list_data.outer_border_size * 2.0
                - self.list_data.inner_border_size * 2.0;
        }
        if next_juan
            > page_height
                - margins.top_margin_pixels
                - margins.bottom_margin_pixels
                - self.list_data.outer_border_size * 2.0
                - self.list_data.inner_border_size * 2.0
        {
            next_juan = page_height
                - margins.top_margin_pixels
                - margins.bottom_margin_pixels
                - self.list_data.outer_border_size * 2.0
                - self.list_data.inner_border_size * 2.0;
        }
        (item_height_pixels, next_juan)
    }

    fn get_position_border_info(
        &mut self,
        row_index: usize,
        range: Range<usize>,
        is_start_row: bool,
        is_on_last_row: bool,
        page_index: &mut usize,
    ) -> BorderPositionData {
        // let mut list_placement_handle: CurrentPlacement = CurrentPlacement::default();
        let mut next = false;
        let go_to_next_page = false;
        let (item_height_pixels, next_juan) = self.get_item_height_pixels(row_index);
        let mut border_position_data = BorderPositionData::default();
        let has_header = self.list_data.header.is_some();
        border_position_data.item_height_pixels = item_height_pixels;
        while RowPositionType::Searching == border_position_data.row_type_and_size {
            border_position_data.list_placement_handle = self
                .manager
                .get_placement_handle(range.clone(), next || go_to_next_page);
            let is_new_page = border_position_data
                .list_placement_handle
                .get_placement_info()
                .is_new_page;

            let is_room_for_normal_row = border_position_data
                .list_placement_handle
                .get_placement_info()
                .draw_height_left_on_page
                >= (item_height_pixels
                    + self.list_data.inner_border_size
                    + self.list_data.inner_border_size)
                    + (next_juan
                        + self.list_data.outer_border_size
                        + self.list_data.inner_border_size);
            let is_room_for_bottom_row = border_position_data
                .list_placement_handle
                .get_placement_info()
                .draw_height_left_on_page
                > (item_height_pixels
                    + self.list_data.outer_border_size
                    + self.list_data.inner_border_size);
            let is_room_for_header_row = border_position_data
                .list_placement_handle
                .get_placement_info()
                .draw_height_left_on_page
                > (self.list_data.row_header_pixels
                    + self.list_data.outer_border_size
                    + self.list_data.inner_border_size)
                    + (next_juan
                        + self.list_data.outer_border_size
                        + self.list_data.inner_border_size);

            border_position_data.row_type_and_size = match (
                is_on_last_row,
                is_new_page,
                is_room_for_header_row,
                is_room_for_normal_row,
                is_room_for_bottom_row,
                is_start_row,
            ) {
                (_, _, _, _, false, _) => {
                    next = true;
                    RowPositionType::Searching
                } //true,true,true,true,true,false
                (false, true, true, true, true, _) => {
                    //class 1
                    next = false;
                    *page_index = 0;
                    if has_header {
                        border_position_data.is_top_border =
                            !self.list_data.exclude_border_on_header;
                        RowPositionType::Header(!self.list_data.exclude_border_on_header)
                    } else {
                        border_position_data.is_top_border = true;
                        RowPositionType::Top
                    }
                }
                (false, true, true, false, true, _) => {
                    //class 1
                    next = false;
                    border_position_data.is_top_border = true;
                    border_position_data.is_bottom_border = true;
                    RowPositionType::FullPageItem
                }
                (false, false, _, true, _, false) => {
                    //class 2
                    next = false;
                    if *page_index == 1 && has_header && self.list_data.exclude_border_on_header {
                        border_position_data.is_top_border = true;
                        RowPositionType::ItemWithTopBorder
                    } else {
                        RowPositionType::Item
                    }
                }
                //is_on_last_row, is_new_page, is_room_for_header_row, is_room_for_normal_row, is_room_for_bottom_row, is_start_row
                (false, false, false, false, true, false) => {
                    //class 3
                    next = false;
                    border_position_data.is_bottom_border = true;
                    if *page_index == 1 && has_header && self.list_data.exclude_border_on_header {
                        border_position_data.is_top_border = true;
                        RowPositionType::Bottom
                    } else {
                        RowPositionType::Bottom
                    }
                }
                (false, false, true, _, _, true) => {
                    //class 4
                    next = false;
                    *page_index = 0;
                    if has_header {
                        border_position_data.is_top_border =
                            !self.list_data.exclude_border_on_header;
                        RowPositionType::Header(!self.list_data.exclude_border_on_header)
                    } else {
                        border_position_data.is_top_border = true;
                        RowPositionType::Top
                    }
                } //is_on_last_row, is_new_page, is_room_for_header_row, is_room_for_normal_row, is_room_for_bottom_row, is_start_row
                (_, true, false, false, true, _) => {
                    //class 5
                    next = false;
                    border_position_data.is_top_border = true;
                    border_position_data.is_bottom_border = true;
                    RowPositionType::FullPageItem
                }
                (true, false, false, _, true, false) => {
                    //class 5
                    next = false;
                    border_position_data.is_bottom_border = true;
                    if *page_index == 1 && has_header && self.list_data.exclude_border_on_header {
                        border_position_data.is_top_border = true;
                        RowPositionType::Bottom
                    } else {
                        RowPositionType::Bottom
                    }
                }

                (true, true, false, _, true, _) => {
                    //class 5
                    next = false;
                    border_position_data.is_top_border = true;
                    border_position_data.is_bottom_border = true;
                    RowPositionType::FullPageItem
                }
                (true, false, false, _, true, true) => {
                    //class 5
                    next = false;
                    border_position_data.is_top_border = true;
                    border_position_data.is_bottom_border = true;
                    RowPositionType::FullPageItem
                } //is_on_last_row, is_new_page, is_room_for_header_row, is_room_for_normal_row, is_room_for_bottom_row, is_start_row
                (false, true, false, true, _, _) => {
                    next = false;
                    if *page_index == 1 && has_header && self.list_data.exclude_border_on_header {
                        border_position_data.is_top_border = true;
                        RowPositionType::ItemWithTopBorder
                    } else {
                        RowPositionType::Item
                    }
                }
                (false, false, false, true, true, true) => {
                    next = false;
                    if *page_index == 1 && has_header && self.list_data.exclude_border_on_header {
                        border_position_data.is_top_border = true;
                        RowPositionType::ItemWithTopBorder
                    } else {
                        RowPositionType::Item
                    }
                }
                (false, false, false, false, true, true) => {
                    next = true;
                    RowPositionType::Searching
                }
                (true, false, true, _, true, false) => {
                    next = false;
                    border_position_data.is_bottom_border = true;
                    if *page_index == 1 && has_header && self.list_data.exclude_border_on_header {
                        border_position_data.is_top_border = true;
                        RowPositionType::Bottom
                    } else {
                        RowPositionType::Bottom
                    }
                }
                (true, false, true, _, true, true) => {
                    next = false;
                    *page_index = 0;
                    if has_header {
                        border_position_data.is_top_border =
                            !self.list_data.exclude_border_on_header;
                        RowPositionType::Header(!self.list_data.exclude_border_on_header)
                    } else {
                        border_position_data.is_top_border = true;
                        RowPositionType::Top
                    }
                } //false,false,true,false,true,false
                (false, false, true, false, true, false) => {
                    next = false;
                    border_position_data.is_bottom_border = true;
                    next = false;
                    if *page_index == 1 && has_header && self.list_data.exclude_border_on_header {
                        border_position_data.is_top_border = true;
                        RowPositionType::Bottom
                    } else {
                        RowPositionType::Bottom
                    }
                }
                (true, true, true, _, _, _) => {
                    //class 1
                    next = false;
                    *page_index = 0;
                    if has_header {
                        border_position_data.is_top_border =
                            !self.list_data.exclude_border_on_header;
                        RowPositionType::Header(!self.list_data.exclude_border_on_header)
                    } else {
                        border_position_data.is_top_border = true;
                        border_position_data.is_bottom_border = true;
                        RowPositionType::FullPageItem
                    }
                }
            };

            /*if border_position_data.row_type_and_size != RowPositionType::Searching {
                println!("Row:{}, is_on_last_row({}), is_new_page({}), is_room_for_header_row({}), is_room_for_normal_row({}), is_room_for_bottom_row({})", row_index, is_on_last_row, is_new_page, is_room_for_header_row, is_room_for_normal_row, is_room_for_bottom_row);
                border_position_data.str_test = format!("{},{},{},{},{},{},{},{}", border_position_data.row_type_and_size, row_index, is_on_last_row, is_new_page, is_room_for_header_row, is_room_for_normal_row, is_room_for_bottom_row,is_start_row);
            }*/
        }
        border_position_data
    }

    pub fn set_item_column_alignments(&mut self, row_alignments: Vec<TextAlignment>) {
        self.list_data.column_text_alignments = Some(row_alignments);
    }
    pub fn set_header_column_alignments(&mut self, column_alignments: Vec<TextAlignment>) {
        self.list_data.header_column_text_alignments = Some(column_alignments);
    }
    pub fn set_all_row_sizes(&mut self, cell_size: Vec<f64>) {
        self.list_data.row_cell_size = Some(cell_size);
    }

    pub fn set_row_types(&mut self, types: Vec<TypeOfItem>) {
        self.list_data.types_of_items = types;
    }
    fn place_level_line(
        &mut self,
        top_y_current_page: &mut f64,
        percentage_start_x: usize,
        percentage_width_x: usize,
        pdf_draw: &mut PdfDrawInfo,
        borders: &Option<RefCell<Vec<Border>>>,
    ) -> PlacementInfo {
        let mut placement_handle = self.manager.get_placement_handle(
            percentage_start_x..percentage_start_x + percentage_width_x,
            false,
        );
        *top_y_current_page = placement_handle.get_placement_info().rec.y;

        let mut blank = TextBox::new(
            "",
            FontInfo::new(0.0, Font::Courier),
            None,
            None,
            None,
            self.list_data.group,
        );
        placement_handle.draw(&mut blank, pdf_draw, borders);

        placement_handle.get_placement_info()
    }
    pub fn set_border_color(&mut self, border_color: (f64, f64, f64)) {
        self.list_data.border_color = border_color;
    }
    pub fn header_has_border(&mut self, does_it: bool) {
        self.list_data.exclude_border_on_header = !does_it;
    }
    fn get_row_color(&self, index_on_page: usize) -> (f64, f64, f64) {
        if let Some((color_a, color_b)) = self.list_data.alternate_row_colors {
            if index_on_page % 2 == 0 {
                color_a
            } else {
                color_b
            }
        } else {
            (1.0, 1.0, 1.0)
        }
    }
    fn get_row_align(&self, column_index: usize, is_header_row: bool) -> TextAlignment {
        if !is_header_row {
            match self.list_data.column_text_alignments {
                Some(ref align) => align
                    .get(column_index)
                    .unwrap_or(&TextAlignment::LeftBottom)
                    .clone(),
                None => TextAlignment::LeftBottom,
            }
        } else {
            match self.list_data.header_column_text_alignments {
                Some(ref align) => align
                    .get(column_index)
                    .unwrap_or(&TextAlignment::LeftBottom)
                    .clone(),
                None => TextAlignment::LeftBottom,
            }
        }
    }
    fn format_column(
        type_of_item: &TypeOfItem,
        text: &String,
        default_color: (f64, f64, f64),
        is_header_row: bool,
    ) -> (String, (f64, f64, f64)) {
        if is_header_row {
            return (text.clone(), default_color);
        }
        let nan_color = (0.5, 0.3, 0.5);
        match type_of_item {
            TypeOfItem::String => (text.clone(), default_color),
            TypeOfItem::Currency(precision) => {
                if let Ok(currency) = text.parse::<f64>() {
                    if currency < 0.0 {
                        (format!("(${:.1$})", -currency, precision), (1.0, 0.0, 0.0))
                    } else {
                        (format!("${:.1$}", currency, precision), default_color)
                    }
                } else {
                    ("NAN".into(), nan_color) //(current_row[column_index].clone(), this_text_color)
                }
            }
            TypeOfItem::Number(precision) => {
                if let Ok(num) = text.parse::<f64>() {
                    if num < 0.0 {
                        (format!("{:.1$}", text, precision), (1.0, 0.0, 0.0))
                    } else {
                        (format!("{:.1$}", text, precision), default_color)
                    }
                } else {
                    ("NAN".into(), nan_color) // (current_row[column_index].clone(), this_text_color)
                }
            }
        }
    }
    fn adjust_for_border(
        _: &mut CurrentPlacement,
        border: &ListBoxBorder,
        is_left_border: bool,
        is_top_border: bool,
        is_right_border: bool,
        is_bottom_border: bool,
        no_top_border_exception: bool,
    ) -> (f64, f64, f64, f64) {
        //column specialty format section

        let interior_margin = 0.2;
        match (
            is_top_border,
            is_left_border,
            is_right_border,
            is_bottom_border,
        ) {
            (true, true, false, false) => match border {
                ListBoxBorder::All(inner, outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    interior_margin,
                    interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (true, false, true, false) => match border {
                ListBoxBorder::All(inner, outer) => (
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, true, false, false) => match border {
                ListBoxBorder::All(inner, outer) => (
                    inner / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, false, true, false) => match border {
                ListBoxBorder::All(inner, outer) => (
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (true, true, true, false) => match border {
                ListBoxBorder::All(inner, outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (true, false, false, false) => match border {
                ListBoxBorder::All(inner, outer) => (
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, true, true, false) => match border {
                ListBoxBorder::All(inner, outer) => {
                    if no_top_border_exception {
                        (
                            interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                        )
                    } else {
                        (
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                        )
                    }
                }
                ListBoxBorder::Outer(outer) => (
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, false, false, false) => match border {
                ListBoxBorder::All(inner, _) => {
                    if no_top_border_exception {
                        (
                            interior_margin,
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                        )
                    } else {
                        (
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                        )
                    }
                }
                ListBoxBorder::Outer(_) => (
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (true, true, false, true) => match border {
                ListBoxBorder::All(inner, outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    interior_margin,
                    interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (true, false, true, true) => match border {
                ListBoxBorder::All(inner, outer) => (
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, true, false, true) => match border {
                ListBoxBorder::All(inner, outer) => {
                    if no_top_border_exception {
                        (
                            interior_margin,
                            outer / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    } else {
                        (
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    }
                }
                ListBoxBorder::Outer(outer) => (
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, false, true, true) => match border {
                ListBoxBorder::All(inner, outer) => {
                    if no_top_border_exception {
                        (
                            interior_margin,
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    } else {
                        (
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    }
                }
                ListBoxBorder::Outer(outer) => (
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (true, true, true, true) => match border {
                ListBoxBorder::All(_, outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(_) => (
                    interior_margin,
                    interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (true, false, false, true) => match border {
                ListBoxBorder::All(inner, outer) => (
                    outer / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Outer(outer) => (
                    outer / 2.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, true, true, true) => match border {
                ListBoxBorder::All(inner, outer) => {
                    if no_top_border_exception {
                        (
                            interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    } else {
                        (
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    }
                }
                ListBoxBorder::Outer(outer) => (
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
            //is_top_border, is_left_border, is_right_border, is_bottom_border
            (false, false, false, true) => match border {
                ListBoxBorder::All(inner, outer) => {
                    if no_top_border_exception {
                        (
                            interior_margin,
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    } else {
                        (
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            inner / 2.0 + interior_margin,
                            outer / 2.0 + interior_margin,
                        )
                    }
                }
                ListBoxBorder::Outer(outer) => (
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    0.0 + interior_margin,
                    outer / 2.0 + interior_margin,
                ),
                ListBoxBorder::Inner(inner) => (
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    inner / 2.0 + interior_margin,
                    0.0 + interior_margin,
                ),
                _ => (0.0, 0.0, 0.0, 0.0),
            },
        }
    }

    pub fn get_half_border_sizes(border: &ListBoxBorder) -> (f64, f64) {
        match border {
            ListBoxBorder::All(inner, outer) => (*inner / 2.0, *outer / 2.0),
            ListBoxBorder::Outer(outer) => (0.0, *outer / 2.0),
            ListBoxBorder::Inner(inner) => (*inner / 2.0, 0.0),
            _ => (0.0, 0.0),
        }
    }
}
pub struct BorderPositionData {
    pub is_top_border: bool,
    pub is_bottom_border: bool,
    pub row_type_and_size: RowPositionType,
    pub list_placement_handle: CurrentPlacement,
    pub item_height_pixels: f64,
    pub str_test: String,
}

impl Default for BorderPositionData {
    fn default() -> Self {
        Self {
            is_top_border: false,
            is_bottom_border: false,
            row_type_and_size: RowPositionType::Searching,
            list_placement_handle: Default::default(),
            item_height_pixels: 0.0,
            str_test: "".to_string(),
        }
    }
}
impl Display for BorderPositionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Height Pixels: {}, Row Type: {}, Top Border: {}, Bottom Border: {} ",
            self.item_height_pixels,
            self.row_type_and_size,
            self.is_top_border,
            self.is_bottom_border
        )
    }
}
