use backfat::container::manager::Manager;
use lopdf::{Object, Stream};
use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use backfat::container_objects::text_box::{TextBox, TextAlignment, BorderStyle};
use backfat::font::font_sizes::{Font, create_font_recource_id};
use backfat::font::font_info::{FontInfo};
use backfat::container::rectangle::Border;
use std::cell::RefCell;
use backfat::container_objects::list_box::{ListBoxBorder, TypeOfItem, ListBox, RowData, RowDataTypes};
use backfat::container_objects::lines::{draw_rectangle, draw_vertical_line};
use backfat::container::placement_info::PlacementInfo;

use rand::Rng;
use backfat::container::container_trait::DrawInfoReq;

struct PdfDox {
    manager: Manager,
}
impl PdfDox {
    pub fn new(width_inch: f64, height_inch: f64, dpi: f64, margin_top:f64, margin_bot:f64) -> Self {
        Self {
            manager: Manager::new(width_inch, height_inch, dpi, margin_top, margin_bot),
        }
    }
}

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

    fn insert_into_page(&mut self, page_num: usize, operation: Operation) {
        self.pdf[page_num].push(operation);
    }
}
fn mimic_report() {
    let borders: Option<RefCell<Vec<Border>>> = Some(RefCell::new(Vec::new()));

    let mut pdf_draw = PdfDrawInfo{ pdf: vec![] };
    let mut dox = PdfDox::new( 8.5, 11.0, 72.0, 0.25,0.25 );

    {
        let mut txt = TextBox::new("Generic Report", FontInfo::new(28.0, Font::Helvetica), Some(TextAlignment::CenterCenter), Some(BorderStyle::Single(1.0)), Some((0.8, 0.8, 0.8)), None);
        let mut placement_handle = dox.manager.get_placement_handle(8..92, false);
        placement_handle.set_pixel_height(0.42 * 72.0);
        placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    {
        let mut txt = TextBox::new("", FontInfo::new(12.0, Font::Helvetica), Some(TextAlignment::CenterCenter), None, None, None);
        let mut placement_handle = dox.manager.get_placement_handle(8..92, false);
        placement_handle.set_pixel_height(0.25 * 72.0);
        placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    {
        let mut txt = TextBox::new("Stuff", FontInfo::new(13.0, Font::Helvetica), Some(TextAlignment::CenterCenter), Some(BorderStyle::Single(1.0)), Some((0.9, 0.9, 0.9)), None);
        let mut placement_handle = dox.manager.get_placement_handle(50..92, false);
        placement_handle.set_pixel_height(0.25 * 72.0);
        placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    {
        let mut txt = TextBox::new("", FontInfo::new(12.0, Font::Helvetica), Some(TextAlignment::CenterCenter), None, None, None);
        let mut placement_handle = dox.manager.get_placement_handle(15..50, false);
        placement_handle.set_pixel_height(0.25 * 72.0);
        placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    {
        let mut txt = TextBox::new("Enter Month", FontInfo::new(11.0, Font::Helvetica), Some(TextAlignment::CenterCenter), None, Some((0.9,0.9,0.9)), None);
        let mut placement_handle = dox.manager.get_placement_handle(12..24, false);
        placement_handle.set_pixel_height(0.25 * 72.0);
        placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    {
        let mut txt = TextBox::new("Enter Year", FontInfo::new(12.0, Font::Helvetica), Some(TextAlignment::CenterCenter), None, None, None);
        let mut placement_handle = dox.manager.get_placement_handle(31..41, false);
        placement_handle.set_pixel_height(0.25 * 72.0);
        placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    {
        let mut txt = TextBox::new("May",FontInfo::new(14.0, Font::Helvetica), Some(TextAlignment::CenterBottom), Some(BorderStyle::Single(1.0)), Some((0.5, 0.7, 0.9)), None );
        let mut placement_handle = dox.manager.get_placement_handle(10..26, false);
        placement_handle.set_pixel_height(0.25 * 72.0);
        placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    {
        let mut txt = TextBox::new("2020",FontInfo::new(14.0, Font::Helvetica), Some(TextAlignment::CenterBottom), Some(BorderStyle::Single(2.0)), Some((0.5, 0.7, 0.9)) , None);
        dox.manager.place_now( 0.25 * 72.0,28..44, &mut pdf_draw, &mut txt, &borders)
        //let mut placement_handle = dox.manager.get_placement_handle(28..44, false);
        //placement_handle.set_pixel_height(0.25 * 72.0);
        //placement_handle.draw(&mut txt, &mut pdf_draw, &borders);
    }
    let mut test_data = Vec::new();
    for i in 0..177 {
        let row_data = vec!["5/12/2020".to_owned(),"".to_owned(),"2032.90".to_owned(),format!( "{}", i).to_owned()];
        test_data.push(RowData::new(row_data, RowDataTypes::default()));
    }
    //doing a single row
    test_data.insert(4, RowData::new(vec!["Bacon".into()], RowDataTypes::SingleNoBorderWithColor((0.0,1.0,1.0), TextAlignment::CenterCenter)));
    test_data.push(RowData::new(vec!["Ham".into()], RowDataTypes::SingleWithColor((0.0,1.0,1.0), TextAlignment::RightBottom)) );
    test_data.insert(42, RowData::new(vec!["Honey".into()], RowDataTypes::SingleNoBorderWithColor((1.0,0.0,0.2), TextAlignment::CenterCenter)));
    //this gets the top most place on the page at that page range.
    let mut placement_handle = dox.manager.get_placement_handle(50..90, false);

    let col_size: Vec<usize> = vec![10, 10, 10, 10];//vec![9, 10, 11, 12];
    let header_data: Vec<String> = vec!["Date".to_owned(), "Invoice Number".to_owned(), "Invoice Amount".to_owned(), "Remaining Budget".to_owned() ];
    let header = RowData::new(header_data, RowDataTypes::default());
    //create list box
    let mut list_box = ListBox::new(&test_data,
                                    col_size,
                                    Some(&header),
                                    &mut dox.manager,
                                    FontInfo::new(5.0, Font::Helvetica),
                                    FontInfo::new(6.0, Font::Helvetica),
                                    ListBoxBorder::All(1.0,1.0), Some(0),
    );

    let mut rng = rand::thread_rng();
    let mut sizes:Vec<f64> = vec![0.0;test_data.len()];
    //randomizing heights of the rows for demonstration.
    for row_ind in 0..test_data.len() {
        let v:f64 = rng.gen_range(10.0, 20.0);
        sizes[row_ind] = v as f64;
    }
    //if ignored the sizes will all be the same based on the ListBox list_item_font
    list_box.set_all_row_sizes(sizes);

    //if wanting something besides strings set here
    list_box.set_row_types(vec![TypeOfItem::String,TypeOfItem::String,TypeOfItem::Currency(2),TypeOfItem::Currency(2)]);
    //align list box items columns
    list_box.set_item_column_alignments( vec![TextAlignment::CenterTop, TextAlignment::CenterCenter, TextAlignment::CenterCenter, TextAlignment::RightBottom, TextAlignment::CenterCenter] );
    //align list box header columns
    list_box.set_header_column_alignments( vec![TextAlignment::LeftJustifyTop(0.05);4] );
    //if false will appear below header row
    list_box.header_has_border(false);

    placement_handle.draw( &mut list_box, &mut pdf_draw, &borders);

    //drawing a border around the group
    for group_rec in dox.manager.get_groups() {
        for page_index in 0..group_rec.1.len() {

            let mut pl: PlacementInfo = PlacementInfo::default();
            pl.page_number = page_index;
            pl.rec = group_rec.1[page_index].clone();
            pl.page_size_info = dox.manager.get_page_info();

            //println!("{}",pl.page_size_info);
        /*        draw_rectangle(&mut pdf_draw,
                               &pl,
                               5.0,
                               (1.0,0.0,0.0));*/
        }
    }

    //drawing borders, drawing after since it will look better
    match borders {
        Some(brd) => {

            for border in brd.into_inner().into_iter() {
            //println!("Border {}, {}", &border.rec.page_size_info);
                draw_rectangle(&mut pdf_draw,
                               &border.rec,
                border.pixel_size,
                               border.color);
            };
        },
        None => {

        }

    }

    //finish pdf
    let mut doc = lopdf::Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(dictionary! {
		"Type" => "Font",
		"Subtype" => "Type1",
		"BaseFont" => "Courier",
	});
    let font_id_bold = doc.add_object(dictionary! {
		"Type" => "Font",
		"Subtype" => "Type1",
		"BaseFont" => "Courier-Bold",
	});
    let resources_id = doc.add_object(dictionary! {
		"Font" => dictionary! {
			"F1" => font_id,
			"F2" => font_id_bold,
		},
	});
    let mut v:Vec<lopdf::Object> = Vec::new();

    for page in 0..pdf_draw.pdf.len() {
        let content = Content {
            operations: pdf_draw.pdf[page].clone()//draw_data_pdf(0.2, 1.0, &mut columns)
        };
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

        let page_id = doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            });
        v.push( page_id.into() )
    };
    let page_count = v.len() as i32;
    let pages = dictionary! {
		"Type" => "Pages",
		"Kids" => v,
		"Count" => page_count,
		"Resources" => resources_id,
		"MediaBox" => vec![0.into(), 0.into(), (dox.manager.get_page_pixel_dims().0).into(), (dox.manager.get_page_pixel_dims().1).into()],
	};
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(dictionary! {
		"Type" => "Catalog",
		"Pages" => pages_id,
	});
    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save("report.pdf").unwrap();

}


fn main() {
    mimic_report();
}
