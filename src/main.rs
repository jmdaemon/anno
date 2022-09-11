use cairo::{PdfSurface, Context};
use clap::{Arg, Command};
use poppler::{PopplerDocument, PopplerPage};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, DrawingArea};

fn main() {
    let matches = Command::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    let _fp = matches.value_of("fp").unwrap_or("").to_owned();

    // Create application
    let app = Application::builder()
        .application_id("io.github.jmdaemon.anno")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Accept command line arguments but don't do anything
    // This is a temporary hack to be able to pass in command line arguments
    // Run the application
    app.run_with_args(&[""]);
}

/// Displays pages from the pdf file
fn show_pdf(_area: &gtk::DrawingArea, ctx: &cairo::Context, _width: i32, _height: i32) {
    let fp = "The C Programming Language.pdf";
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    let page: PopplerPage = file.get_page(0).unwrap();
    page.render(ctx);
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Anno")
        .build();

    // Pack Widgets into Box
    let gtkbox = Box::new(gtk::Orientation::Vertical, 0);

    // Create drawing widget
    let drawarea = DrawingArea::new();
    drawarea.set_content_width(1280);
    drawarea.set_content_height(720);
    drawarea.set_draw_func(show_pdf);

    gtkbox.append(&drawarea);

    window.set_child(Some(&gtkbox));

    // Present window
    window.present();
}
