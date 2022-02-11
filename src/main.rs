use clap::{Arg, App};

fn main() {
    let matches = App::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();
}

