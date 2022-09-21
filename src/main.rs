//#![feature(once_cell)] 
#![feature(once_cell)] 

mod anno;
mod fileview;
mod mainwidget;
mod pageobject;

mod application;
mod config;
mod window;
//mod anno;

// Imports
use self::application::AnnoApplication;
use self::window::AnnoWindow;
use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
//use crate::anno::{Arguments, CURRENT_ARGS};
use crate::anno::{Arguments, ArgumentsState, ARGS};
//use crate::anno::Arguments;

// Standard Libraries
use std::sync::{Arc, RwLock};
use std::sync::Mutex;

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
    //let mut fp = file.as_str();
    //CURRENT_ARGS = RwLock::new(Arc::new(Arguments { verbose: false, fp: file }));
    //println!("Set arguments to: {}, {}\n", curr.verbose, curr.fp);

    //anno::CURRENT_ARGS = Arguments { verbose: false, fp: file };

    //let args = Arguments::get();
    //let args = Arguments::global();
    //let mut args = anno::global();
    //args.lock();
    //args.set_path(file.as_str());
    //ARGS.set(Mutex::new(args));
    //let args = ArgumentsState { verbose: false, fp: file };

    let mut args = anno::global();
    args.set_path(&file);
    //Arguments::set_path(&mut self, path)
    //ARGS.set(args);
    //args.lock().set_path(file.as_str());
    
    //let args = Arguments { verbose: false, fp: file };
    ////ARGS = args;
    ////Arguments::from_args(args);
    //ARGS.set(args).unwrap();
    

    //CURRENT_ARGS = RwLock::new(args);
    //CURRENT_ARGS.lock().unwrap().push(args);

    //{
        //let mut rw_args = CURRENT_ARGS.write().unwrap();
        //*rw_args = args;
    //}

    //let curr = CURRENT_ARGS.read().unwrap();
    //println!("Set arguments to: {}, {}\n", curr.verbose, curr.fp);

    //RwLock::new(args)

    //Arguments { verbose: false, fp: file }.make_current();
    //println!("Set arguments to: {}, {}\n", Arguments::current().verbose, Arguments::current().fp);

    //let args: Arguments = Arguments { verbose: false, fp: file };
    //CURRENT_ARGS.store(Arc::new(args));
    //CURRENT_ARGS.store(Arc::new(Arguments { verbose: false, fp: file }));
    //println!("Set arguments to: {}, {}\n", curr.verbose, curr.fp);

    //let curr = CURRENT_ARGS.load();
    //println!("Set arguments to: {}, {}\n", curr.verbose.clone(), curr.fp.clone());


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
