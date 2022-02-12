//extern crate clap;
//extern crate poppler;
//extern crate slint;
extern crate cairo;
use clap::{Arg, App};
use poppler::{PopplerDocument, PopplerPage};
use slint::{ModelRc, VecModel, SharedString};
slint::include_modules!();
use cairo::{ ImageSurface, Format, Context };
use std::rc::Rc;
use std::fs::File

fn main() {
    let matches = App::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    let fp = matches.value_of("fp").unwrap_or("");
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    
:   let page: PopplerPage = file.get_page(0).unwrap();
    
    let text = page.get_text().unwrap_or("");
    // Display text in ui here
    //let pdf_model = Rc::new(PDFData { content: SharedString::from(text.to_owned())});
    let pdf_model = PDFData { content: SharedString::from(text.to_owned())};

    /*
    let pdf_model = PDFData {
        content: SharedString::from(text.to_owned()),
        source: "output.png"
    };

    // Create Cairo surface
    //let surface = ImageSurface::create(Format::ARgb32, 3200, 1800).expect(“Couldn’t create surface”);
    let context = page.render();
    context.paint();
    let mut file = File::create("output.png").expect("Couldn’t create file"); 
    context.write_to_png(&mut file).expect("Couldn’t write to png");
    //surface.write_to_png(&mut file).expect("Couldn’t write to png");

    */

    let ui = AppWindow::new();

    ui.set_pdf(pdf_model);
    let _appwin_weak = ui.as_weak();
    ui.run();
}

