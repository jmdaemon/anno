// Module Struture
mod anno;
mod fileview;
mod pageobject;
mod mainwidget;
mod config;
mod window;
mod application;

// Imports
use self::application::AnnoApplication;
use self::window::AnnoWindow;
use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};

// Third Party Libraries
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::gio;
use gtk::prelude::*;
use clap::{Arg, Command};

fn main() {
    // Parse cli args using clap
    let matches = Command::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();

    // Set new arguments
    let file = matches.value_of("fp").unwrap_or("").to_owned();
    let mut args = anno::global();
    args.set_path(&file);

    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/anno.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = AnnoApplication::new("io.github.jmdaemon.anno", &gio::ApplicationFlags::empty());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    std::process::exit(app.run_with_args(&[""]));
}
