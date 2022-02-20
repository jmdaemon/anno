use poppler::{PopplerDocument, PopplerPage};
use clap::{Arg, App};

fn main() {
    let matches = App::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    let fp = matches.value_of("fp").unwrap_or("");
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    
    // Get the first page
    let page: PopplerPage = file.get_page(0).unwrap();
    let text = page.get_text().unwrap_or("");
}

