//extern crate cairo;
//extern crate gdk;
//extern crate glib;

use cairo::{PdfSurface, Context};
//use clap::{Arg, Command, App};
use clap::{Arg, Command};

//use glib::ffi::gpointer;
//use gtk::ffi::gtk_drawing_area_set_draw_func;
use poppler::{PopplerDocument, PopplerPage};
use gtk::prelude::*;
//use gdk::prelude::*;
//use std::cell::RefCell;
//use std::ffi::c_void;
//use std::ptr::null;
//use std::rc::Rc;
//use gtk::glib::clone;
use gtk::{Application, ApplicationWindow, DrawingArea};
//use gtk::cairo::PdfSurface;

//static mut FILEPATH: &str = "Test";
//static mut match_clone: clap::ArgMatches;
//static mut matches: clap::ArgMatches = ;
//const app
//let app = Command::new("Anno")

//const APP: clap::Command = Command::new("Anno")
    //.version("0.1.0")
    //.author("Joseph Diza <josephm.diza@gmail.com>")
    //.about("Easily create beautiful, customizable annotations for pdfs")
    //.arg(Arg::new("fp").help("File path to the pdf"));

//const matches: clap::ArgMatches = app.get_matches();
//static mut FILEPATH: &str = "";
//static mut FILEPATH: &String = &("".to_string());
//static mut FILEPATH: &String = &String { vec: vec![] };

//static mut FILEPATH: String = String { vec: vec![] };

//static mut FILEPATH: String = String::new();

fn main() {
    //let matches: &'static clap::ArgMatches = &App::new("Anno")
    let matches = Command::new("Anno")
        .version("0.1.0")
        .author("Joseph Diza <josephm.diza@gmail.com>")
        .about("Easily create beautiful, customizable annotations for pdfs")
        .arg(Arg::new("fp").help("File path to the pdf"))
        .get_matches();
    //let app = &App::new("Anno")

    //let app = Command::new("Anno")
        //.version("0.1.0")
        //.author("Joseph Diza <josephm.diza@gmail.com>")
        //.about("Easily create beautiful, customizable annotations for pdfs")
        //.arg(Arg::new("fp").help("File path to the pdf"));

    //let matches: &'static clap::ArgMatches = app.get_matches();

    //let matches = app.get_matches();

    //let match_ref: &'static clap::ArgMatches = &matches;


    //let fp = match_clone.value_of("fp").unwrap_or("");
    //let fp = matches.value_of("fp").as_deref().unwrap_or("");
    //let fp = matches.value_of("fp").to_owned();
    //let fp = matches.value_of("fp").unwrap_or("");
    let fp = matches.value_of("fp").unwrap_or("").to_owned();
    //unsafe {
        //FILEPATH = fp.unwrap();
    //}
    //unsafe {
        //FILEPATH = fp.as_ref();
        //FILEPATH = fp;
    //}

    //let fp = matches.value_of("fp");

    //unsafe {
    ////let match_clone = matches.clone();
        //FILEPATH = std::clone(fp);
    //}
    //FILEPATH = fp;


    //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    //let page: PopplerPage = file.get_page(5).unwrap();
    //let (width, height) = page.get_size();


    //let surface = PdfSurface::new(1920, 1080, fp);
    //let ctx = cairo::Context::new(surface);

    


    // Test
    //let surface = PdfSurface::new(width, height, fp).unwrap();
    //let ctx = Context::new(&surface).unwrap();
    //ctx.paint();

    //page.render(&ctx);
    // Test



    
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
    //app.connect_activate(|self, fp| -> {
        //build_ui(self, fp);
    //});
    //app.connect_activate(build_ui(ctx));
    //app.connect_activate(|ctx| -> { build_ui(app, ctx) });
    //build_ui(&app, ctx);
    //build_ui(&app, ctx);
    //build_ui(&app, fp.unwrap_or(""));
    //build_ui(&app, &fp);
    //build_ui(&app, &fp);
    //build_ui(&app);
    //unsafe {
        //build_ui(&app, FILEPATH);
    //}

    // Accept command line arguments but don't do anything
    // This is a temporary hack to be able to pass in command line arguments
    // Run the application
    app.run_with_args(&[""]);
}

//fn draw_event() {
//}

//fn draw_event(ctx: &Context, fp: &str) {
//fn draw_event(area: gtk::DrawingArea , ctx: &Context, fp: &str, width: i64, height: i64) {
//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32) {
//fn draw_event(ctx: &cairo::Context, fp: &str) {

//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32, fp: &str) {
//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32, fp: String) {
//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32, fp: Rc<String>) {
//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32, fp: Rc<&String>) {
//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32, fp: Rc<String>) {
//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32, fp: String) {
//fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32, fp: &String) {
fn draw_event(area: &gtk::DrawingArea, ctx: &cairo::Context, width: i32, height: i32) {
    let fp = "The C Programming Language.pdf";
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    let page: PopplerPage = file.get_page(1).unwrap();
    page.render(ctx);

    //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    //let file: PopplerDocument = PopplerDocument::new_from_file(fp.as_str(), "").unwrap();
    //let file: PopplerDocument;
    //unsafe { 
        //file = PopplerDocument::new_from_file(FILEPATH, "").unwrap();
    //}
    //let page: PopplerPage = file.get_page(5).unwrap();

    //let (width, height) = page.get_size();
    //page.render(ctx);
}

//fn build_ui(app: &Application, ctx: Context) {
//fn build_ui(app: &Application, fp: &'static str) {
//fn build_ui(app: &Application, fp: &str) {
//fn build_ui(app: &Application, fp: String) {
//fn build_ui(app: &Application, fp: &String) {
fn build_ui(app: &Application) {
//fn build_ui(app: &Application, fp: &String) {
//fn build_ui(app: &Application) {
//fn build_ui<'a>(app: &Application, fp: &'a String) {
//fn build_ui<'a>(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Anno")
        //.child(&ctx)
        //.child(&page)
        .build();

    //gtk::DrawingArea::new();
    //let drawarea = gtk::DrawingArea::new()
    //drawarea.set_content_width(640)
        //.set_content_height(480);



//fn build_ui(app: &Application, page: PdfSurface) {
    //let drawarea = DrawingArea::new();

    // Pack Box
    let gtkbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let drawarea = gtk::DrawingArea::new();
    drawarea.set_content_width(1280);
    drawarea.set_content_height(720);
    drawarea.set_draw_func(draw_event);

    gtkbox.append(&drawarea);

    window.set_child(Some(&gtkbox));

    // Present window
    window.present();


    // Create Context
    //let ctx = cairo::Context::new();
    //let surface = PdfSurface::new(1920 as f64, 1080 as f64, fp).unwrap();

    //let surface = PdfSurface::new(1920 as f64, 1080 as f64, &FILEPATH.as_str()).unwrap();
    //let surface: PdfSurface;

    //unsafe {
        //surface = PdfSurface::new(1920 as f64, 1080 as f64, &FILEPATH.as_str()).unwrap();
    //}
    //let ctx = cairo::Context::new(&surface);

    // Create Drawing Area
    //drawarea.set_content_width(640).set_content_height(480);


    //let h = 1280;
    //let w = 720;

    //drawarea.connect("draw", false, draw_event);
    //drawarea.connect("draw", false, move |ctx| {

    //drawarea.set_draw_func(draw_event, ctx, 1280, 720);
    //DrawingAreaExtManual(&drawarea, &ctx, 1280, 720);
    //drawarea.set_draw_func(|ctx, fp| { draw_event(ctx, fp) });

    //let src: Rc<RefCell<&str>> = Rc::new(RefCell::new(fp));
    //let src: Rc<RefCell<&str>> = Rc::new(RefCell::new(FILEPATH));
    //let src = Rc::new(fp.clone());
    //let src = Rc::new(fp);
    //let src:  Rc<&String>;
    //let src:  Rc<&String>;
    //unsafe {
        //src = Rc::new(&FILEPATH);
    //}

    //drawarea.set_draw_func(clone!(@weak src => move |drawarea, ctx, 1280, 720| {

    //drawarea.set_draw_func(clone!(@weak src => move |drawarea, ctx, h,w| {
        //draw_event(drawarea, ctx, 1280, 720, src.borrow().as_ref());
    //}));

    //drawarea.set_draw_func(clone!(@weak FILEPATH => move |drawarea, ctx, h,w| {
        //draw_event(drawarea, ctx, 1280, 720, FILEPATH.borrow().as_ref());
    //}));
    //drawarea.set_draw_func(clone!(@weak src => move |drawarea, ctx, h,w| {
    //unsafe {
        //let userdata: gpointer;
        //let null = null();
        //gtk_drawing_area_set_draw_func(self_, draw_func, user_data, destroy)

        //gtk_drawing_area_set_draw_func(drawarea.as_ptr(), |drawarea, ctx, h,w, fp| {

        //}, userdata, null);
            //|drawarea, ctx, h,w, fp| {
            //draw_event(drawarea, ctx, h, w, fp);
        //} ctx.as_ptr(), h, w, userdata, null);
        //} ctx.as_ptr(), h, w, userdata, null);
        //} userdata, null);
    //}

    //drawarea.set_draw_func(clone!((fp => |drawarea, ctx, 1280, 720| {
        //draw_event(drawarea, ctx, 1280, 720, fp);
    //}));
    
    //drawarea.connect("draw", false, |drawarea, ctx, h,w, fp| {
        //draw_event(drawarea, ctx, h, w, fp);
    //});


        //draw_event(drawarea, ctx, h, w, fp);
        //)

    //drawarea.set_draw_func(|drawarea, ctx, h,w| {
        //draw_event(drawarea, ctx, h, w, fp);
    //});
        //draw_event(drawarea, ctx, 1280, 720, src.as_ref());
        //draw_event(drawarea, ctx, 1280, 720, src);
        //draw_event(drawarea, ctx, 1280, 720, fp);
        //draw_event(drawarea, ctx, 1280, 720, src.to_owned());
        //draw_event(drawarea, ctx, 1280, 720, src.borrow().as_ref());
    //}));



        //draw_event(drawarea, ctx, 1280, 720, fp);
        //draw_event(drawarea, ctx, 1280, 720, &((*src).into_inner()));
        //draw_event(drawarea, ctx, 1280, 720, (*src).into_inner());

    //drawarea.set_draw_func(clone!((fp => |drawarea, ctx, 1280, 720| {
        //draw_event(drawarea, ctx, 1280, 720, fp);
    //}));

    //DrawingAreaExtManual::set_draw_func(&drawarea, move |drawarea, ctx, 1280, 720, fp| {
        //draw_event(drawarea, ctx, 1280, 720, fp);
    //});


    //drawarea.connect("draw", false, |_| {
        //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
        //let page: PopplerPage = file.get_page(5).unwrap();
        //let (width, height) = page.get_size();

        //page.render(ctx);

        ////Some(drawarea)

    //});


    //drawarea.connect("draw", false, move |drawarea| {
        //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
        //let page: PopplerPage = file.get_page(5).unwrap();
        //let (width, height) = page.get_size();

        //page.render(ctx);

        ////Some(drawarea)

    //});

    //drawarea.connect_draw(move |ctx| {
    //});

//}
}
