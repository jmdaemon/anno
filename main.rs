//extern crate clap;
//extern crate poppler;
//extern crate slint;
//extern crate cairo;
use clap::{Arg, App};
use poppler::{PopplerDocument, PopplerPage};
//use slint::{ModelRc, VecModel, SharedString};
//slint::include_modules!();
//use cairo::{ ImageSurface, Format, Context };
use gtk::cairo::{ ImageSurface, Format, Context };
//use std::rc::Rc;
use std::fs::File
use gtk::prelude::*;
use gdk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ScrolledWindow};

//fn build_ui(ctx: Context) {
fn build_ui(ctx: Context) {
    let application = Application::builder()
        .application_id("jmdaemon.github.io.anno")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Anno")
            .default_width(350)
            .default_height(70)
            .build();

        let vertbox = gtk::Box::new(gtk::Orientation::Vertical, 24);
        let scrollwin = ScrolledWindow::new();
        scrollwin.set_child(Some(&ctx));

        // Opens the File Dialog to select a file
        let btnbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let openbtn = Button::with_label("Open");
        openbtn.connect_clicked(|_| {
            eprintln!("Open handler");
        });

        // Creates a new pdf file
        let newpdf = Button::with_label("New");
        newpdf.connect_clicked(|_| {
            eprintln!("New PDF");
        });

        // Show application
        let about = Button::with_label("About");
        about.connect_clicked(|_| {
            eprintln!("About");
        });
        btnbox.append(&openbtn);
        btnbox.append(&newpdf);
        btnbox.append(&about);
        //window.set_child(Some(&openbtn));
        //window.set_child(Some(&newpdf));
        //window.set_child(Some(&about));
        //window.set_child(Some(&scrollwin));
        vertbox.append(&btnbox);
        vertbox.append(&scrollwin);
        window.set_child(Some(&vertbox));

        window.show();
    });

    application.run();
}

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

    //let canvas = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let surface = ImageSurface::create(Format::ARgb32, 3200, 1800).expect("Couldn’t create surface");
    let context = Context::new(&surface).expect("");
    //context.paint();
    //page.render(&context);
    //page.render(&context);
    let mut file = File::create("output.png").expect("Couldn’t create file"); 
    surface.write_to_png(&mut file).expect("Couldn’t write to png");
    
    // GDK
    //let pixbuf = gdk::pixbuf_get_from_surface(&surface);
    //let pixbuf = gdk::pixbuf_get_from_surface(&surface, 0, 0, 3200, 1800);
    //let pixbuf = gdk::pixbuf_get_from_surface(&(*surface as gtk::cairo::Surface), 0, 0, 3200, 1800);
    //let pixbuf = gdk::pixbuf_get_from_surface(&(*surface), 0, 0, 3200, 1800);
    let pixbuf = gdk::pixbuf_get_from_surface(surface, 0, 0, 3200, 1800);

    
    build_ui(context);
    

    //surface.write_to_png(&mut file).expect("Couldn’t write to png");
    // Display text in ui here
    //let pdf_model = Rc::new(PDFData { content: SharedString::from(text.to_owned())});
    //let pdf_model = PDFData { content: SharedString::from(text.to_owned())};

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

    //let ui = AppWindow::new();

    //ui.set_pdf(pdf_model);
    //let _appwin_weak = ui.as_weak();
    //ui.run();
}

