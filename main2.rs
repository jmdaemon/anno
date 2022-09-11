use clap::{Arg, App};
//use poppler::{PopplerDocument, PopplerPage};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, ScrolledWindow};

fn build_ui() {
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

        // Opens the File Dialog to select a file
        let btnbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let openbtn = Button::with_label("Open");
        openbtn.connect_clicked(|_| {
            eprintln!("Open handler");
        });

        // Creates a new annotation
        let newanno = Button::with_label("New");
        newanno.connect_clicked(|_| {
            eprintln!("New Annotation");
        });

        // Show application
        let about = Button::with_label("About");
        about.connect_clicked(|_| {
            eprintln!("About");
        });
        btnbox.append(&openbtn);
        btnbox.append(&newanno);
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
    //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    
    // Get the first page
    //let page: PopplerPage = file.get_page(0).unwrap();
    //let text = page.get_text().unwrap_or("");

    // GTK4 

    build_ui();
}

