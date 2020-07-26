use lopdf::content::Operation;

use crate::container::placement_info::PlacementInfo;
use crate::container::container_trait::DrawInfoReq;

pub fn draw_rectangle(
    draw_to: &mut dyn DrawInfoReq,
    placement_info: &PlacementInfo,
    size: f64,
    border_color: (f64, f64, f64),
) {
    let rec = placement_info
        .rec
        .get_pdf_version(placement_info.page_size_info.clone());

    draw_to.insert_into_page(placement_info.page_number, Operation::new("q", vec![]));

    draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
    draw_to.insert_into_page(placement_info.page_number, Operation::new(
        "rg",
        vec![
            border_color.0.into(),
            border_color.1.into(),
            border_color.2.into(),
        ],
    )); //stroke color
    draw_to.insert_into_page(placement_info.page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
    draw_to.insert_into_page(placement_info.page_number, Operation::new(
        "SC",
        vec![
            border_color.0.into(),
            border_color.1.into(),
            border_color.2.into(),
        ],
    ));

    //rec = vec![(placement_info.left_pixel_x + size / 2.0).into(),(real_height + size / 2.0).into(), (placement_info.pixels_x_width - size).into(), (placement_info.object_pixel_height - size).into()];
    //rec = vec![(placement_info.left_pixel_x).into(),(real_height).into(), (placement_info.pixels_x_width).into(), (placement_info.object_pixel_height).into()];
    draw_to.insert_into_page(placement_info.page_number, Operation::new("w", vec![size.into()]));
    draw_to.insert_into_page(placement_info.page_number, Operation::new("re", rec.into()));
    draw_to.insert_into_page(placement_info.page_number, Operation::new("s", vec![]));

    draw_to.insert_into_page(placement_info.page_number, Operation::new("Q", vec![]));
}

pub fn draw_horizontal_line<T:DrawInfoReq>(
    draw_to: &mut T,
    page_number: usize,
    top_pixel: f64,
    left_x_pixel: f64,
    width_pixel: f64,
    page_height_pixels: f64,
    size: f64,
    border_color: (f64, f64, f64),
) {
    draw_to.insert_into_page(page_number, Operation::new("q", vec![]));

    let real_height = page_height_pixels - top_pixel;

    //color
    let color = border_color;
    draw_to.insert_into_page(page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
    draw_to.insert_into_page(page_number, Operation::new(
        "rg",
        vec![color.0.into(), color.1.into(), color.2.into()],
    )); //stroke color
    draw_to.insert_into_page(page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
    draw_to.insert_into_page(page_number, Operation::new(
        "SC",
        vec![color.0.into(), color.1.into(), color.2.into()],
    ));
    //color

    draw_to.insert_into_page(page_number, Operation::new("w", vec![size.into()]));
    draw_to.insert_into_page(page_number, Operation::new(
        "m",
        vec![(left_x_pixel).into(), (real_height).into()],
    ));
    draw_to.insert_into_page(page_number, Operation::new(
        "l",
        vec![(left_x_pixel + width_pixel).into(), (real_height).into()],
    ));

    draw_to.insert_into_page(page_number, Operation::new("h", vec![]));

    draw_to.insert_into_page(page_number, Operation::new("S", vec![]));
    draw_to.insert_into_page(page_number, Operation::new("Q", vec![]));
}

pub fn draw_vertical_line(
    draw_to: &mut dyn DrawInfoReq,
    page_number: usize,
    top_pixel: f64,
    left_x_pixel: f64,
    height_of_object: f64,
    page_height_pixels: f64,
    size: f64,
    border_color: (f64, f64, f64),
) {
    draw_to.insert_into_page(page_number, Operation::new("q", vec![]));

    let real_height = page_height_pixels - top_pixel;
    let bottom_point = real_height - height_of_object;

    //color
    let color = border_color;
    draw_to.insert_into_page(page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
    draw_to.insert_into_page(page_number, Operation::new(
        "rg",
        vec![color.0.into(), color.1.into(), color.2.into()],
    )); //stroke color
    draw_to.insert_into_page(page_number, Operation::new("CS", vec!["DeviceRGB".into()]));
    draw_to.insert_into_page(page_number, Operation::new(
        "SC",
        vec![color.0.into(), color.1.into(), color.2.into()],
    ));
    //color
    draw_to.insert_into_page(page_number, Operation::new(
        "m",
        vec![(left_x_pixel).into(), (real_height).into()],
    ));
    draw_to.insert_into_page(page_number, Operation::new(
        "l",
        vec![(left_x_pixel).into(), (bottom_point).into()],
    ));

    draw_to.insert_into_page(page_number, Operation::new("h", vec![]));
    draw_to.insert_into_page(page_number, Operation::new("w", vec![size.into()]));

    draw_to.insert_into_page(page_number, Operation::new("S", vec![]));

    draw_to.insert_into_page(page_number, Operation::new("Q", vec![]));
}
