use clap::{Arg, App};
//use poppler::{PopplerDocument, PopplerPage};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let matches = App::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    let _fp = matches.value_of("fp").unwrap_or("");

    //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    
    // Get the first page
    //let page: PopplerPage = file.get_page(0).unwrap();
    //let text = page.get_text().unwrap_or("");

    // GTK4 
    let app = Application::builder()
        .application_id("jmdaemon.github.io.anno")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Anno")
        .build();

    // Present window
    window.present();
}
