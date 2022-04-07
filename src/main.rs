extern crate cairo;
use cairo::{PdfSurface, Context};
use clap::{Arg, App};
use poppler::{PopplerDocument, PopplerPage};
use gtk::prelude::*;
//use gdk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
//use gtk::cairo::PdfSurface;

fn main() {
    let matches = App::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    let fp = matches.value_of("fp").unwrap_or("");

    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();

    //let surface = PdfSurface::new(1920, 1080, fp);
    //let ctx = cairo::Context::new(surface);

    let page = file.get_page(0).unwrap();
    let (width, height) = page.get_size();
    
    let surface = PdfSurface::new(width, height, fp).unwrap();
    let ctx = Context::new(&surface).unwrap();
    ctx.paint();

    page.render(&ctx);
    
    //file.();

    //let pdf_surface = PdfSurface::new();
    
    // Get the first page
    //let page: PopplerPage = file.get_page(0).unwrap();
    //let text = page.get_text().unwrap_or("");

    // GTK4 
    let app = Application::builder()
        .application_id("jmdaemon.github.io.anno")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Accept command line arguments but don't do anything
    // This is a temporary hack to be able to pass in command line arguments
    // Run the application
    app.run_with_args(&[""]);
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Anno")
        .build();

    //gtk::DrawingArea::new();
    //let drawarea = gtk::DrawingArea::new()
    //drawarea.set_content_width(640)
        //.set_content_height(480);

    // Present window
    window.present();
}
