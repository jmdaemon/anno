use clap::{Arg, App};
use poppler::{PopplerDocument, PopplerPage};
use slint::{ModelRc, VecModel, SharedString};
use std::rc::Rc;
slint::include_modules!();

fn main() {
    let matches = App::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    let fp = matches.value_of("fp").unwrap_or("");
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    let page: PopplerPage = file.get_page(0).unwrap();
    
    let text = page.get_text().unwrap_or("");
    // Display text in ui here
    //let pdf_model = Rc::new(PDFData { content: SharedString::from(text.to_owned())});
    let pdf_model = PDFData { content: SharedString::from(text.to_owned())};
    let ui = AppWindow::new();

    ui.set_pdf(pdf_model);
    let _appwin_weak = ui.as_weak();
    ui.run();
}

