// Imports
//use crate::anno::{Arguments, CURRENT_ARGS};
//use crate::anno::CURRENT_ARGS;
//use crate::anno::{Arguments, ARGS};
use crate::anno::{global, Arguments, ARGS};

use std::borrow::Borrow;
// Standard Library
use std::cell::{Cell, RefCell};
//use std::rc::Rc;
use std::sync::Arc;

// Third Party Library
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use glib::Object;
use poppler::{PopplerDocument, PopplerPage};

pub mod imp {
    //use std::borrow::Borrow;

    use super::*;
    // Object holding the state
    #[derive(Debug, Default)]
    pub struct FileView {
        //pub path: Rc<RefCell<String>>,
        pub path: Arc<RefCell<String>>,
        pub height: Cell<i32>,
        pub width: Cell<i32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for FileView {
        const NAME: &'static str = "FileView";
        type Type = super::FileView;
        type ParentType = gtk::DrawingArea;
    }

    //impl FileView {
        ////fn show_pdf(self: &imp::FileView, ctx: &cairo::Context, _width: i32, _height: i32) {
        //fn show_pdf(self, ctx: &cairo::Context, _width: i32, _height: i32) {
            ////let fp = Arguments::current().fp.clone();
            ////let fp = self.imp().path.take();
            //let fp = self.path.take();
            //println!("{}", fp);
            //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
            //println!("{:?}", file.get_metadata());
            //let page: PopplerPage = file.get_page(0).unwrap();
            //page.render(ctx);
            ////page.render_for_printing(ctx);
        //}
    //}

    // Trait shared by all GObjects
    impl ObjectImpl for FileView {

        fn constructed(&self, obj: &Self::Type) {
            // Call "constructed" on parent
            self.parent_constructed(obj);

            // Populate members
            //let args = Arguments::current();
            //self.path.replace(args.fp.to_owned());

            //let args = Arguments::current();
            //self.path.replace(args.fp.clone());

            //let args = CURRENT_ARGS;
            //let fp = args.into_inner().fp.clone();

            //let args = CURRENT_ARGS.load();
            //let fp = args.fp.clone();

            //let args = CURRENT_ARGS.read().unwrap();
            //let fp = args.fp.clone();
            //let args = CURRENT_ARGS.read().unwrap();
            //let fp = args.fp;
            //self.path.replace(fp.to_owned());

            //let args = CURRENT_ARGS.read().unwrap();
            //self.path.replace(args.fp.clone());
            //self.path.replace
            //self.path.replace(*args.fp.borrow());
            
            //let args = Arguments::global();
            //let fp = args.fp.clone();
            //self.path.replace(fp);

            self.width.replace(1440);
            self.height.replace(900);

            //obj.set_draw_func(super::show_pdf);
            obj.set_draw_func(show_pdf);
            println!("Constructed FileView");
            //println!("Filepath, {}", args.fp.clone());

            //self.set_draw_func(super::FileView::show_pdf);
            //obj.set_draw_func(show_pdf);
            //self.set_draw_func(super::FileView::show_pdf);
            //self.set_draw_func(self.show_pdf);
            //self.connect_draw();
            //obj.connect_draw({});

              //area.connect_draw(clone!(graph => move |w, c| {
                  //graph.borrow()
                      //.draw(...}));
                  //graph
            //}
            //Arguments::make_current(*args.borrow());
            //Arguments::make_current(args.borrow());
        }
    }
    // Trait shared by all widgets
    impl WidgetImpl for FileView {}

    // Trait shared by all drawing areas
    impl DrawingAreaImpl for FileView {}

    //pub unsafe trait IsA<T: DrawingAreaImpl>: ObjectType + AsRef<T> + 'static { }

    //impl DrawingAreaExtManual for FileView {}
    //impl IsA<DrawingAreaExt> for FileView {}
}

glib::wrapper! {
    pub struct FileView(ObjectSubclass<imp::FileView>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl FileView {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `FileView`.")
    }

}
 /// Displays pages from the pdf file
fn show_pdf(_area: &gtk::DrawingArea, ctx: &cairo::Context, _width: i32, _height: i32) {
//fn show_pdf(A: IsA<imp::FileView>, ctx: &cairo::Context, _width: i32, _height: i32) {

    //let args = Arguments::current();

    //let args = CURRENT_ARGS.load();
    //let fp = args.fp.clone();

    //let args = CURRENT_ARGS.read().unwrap();
    //let fp: &String = args.fp.borrow();
    //let fp = args.fp.clone();
    //let fp = args.fp.to_owned();

    //let args = Arguments::global();
    //let fp = args.fp.clone();

    //let args = Arguments::get();
    //let fp = args.fp.clone();
    //let args = crate::anno::global();
    //let fp = args.get_path();
    let args = global();
    let fp = args.get_path();


    //self.path.replace(fp);

    //let fp = "The C Programming Language.pdf";
    println!("showpdf: fp: {}", fp);
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    println!("{:?}", file.get_metadata());
    let page: PopplerPage = file.get_page(0).unwrap();
    page.render(ctx);
    //page.render_for_printing(ctx);

    //let fp = args.into_inner().fp.clone();
    //self.path.replace(args.fp.clone());
    //let fp = args.fp.clone();

    //let fp = "The C Programming Language.pdf";
    //let fp = Arguments::current().fp.clone();
    //let fv = _area.downcast::<imp::FileView>().unwrap();
    //let fv = _area.upcast::<imp::FileView>().unwrap();

//fn show_pdf(self: &imp::FileView, ctx: &cairo::Context, _width: i32, _height: i32) {
//fn show_pdf(self: Self, ctx: &cairo::Context, _width: i32, _height: i32) {
    //let fp = Arguments::current().fp.clone();
    //let fp = self.imp().path.take();
    //let fp = self.path.take();
//fn show_pdf(self: &Self, ctx: &cairo::Context, _width: i32, _height: i32) {
    //let fileview = _area.downcast::<imp::FileView>().expect("");
    //let fileview = _area.downcast::<imp::FileView>().expect("FileView does not exist");

    //let args = Arguments::current();
    //let fp = args.fp.to_owned();

    //let fp = args.fp.clone();
    //let fp = fileview.path.take();
    //let fp = self.imp().path.take();

}
