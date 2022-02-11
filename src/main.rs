use clap::{Arg, App};

use slint::{ModelRc, VecModel, SharedString};
slint::include_modules!();


fn main() {
    let matches = App::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    let fp = matches.value_of("fp").unwrap_or("");

    let ui = AppWindow::new();
    ui.run();
}

