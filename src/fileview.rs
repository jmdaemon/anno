//mod fileview;

use std::cell::{Cell, RefCell};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use glib::Object;
//use anno::Arguments;
use crate::anno::Arguments;
use poppler::{PopplerDocument, PopplerPage};

pub mod imp {
    use super::*;
    // Object holding the state
    #[derive(Debug, Default)]
    pub struct FileView {
        pub path: RefCell<String>,
        pub height: Cell<i32>,
        pub width: Cell<i32>,
    }

    //#[derive(Default)]
    //pub struct PdfView {
        //document: PopplerDocument,
        //page: PopplerPage,
    //}

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for FileView { const NAME: &'static str = "FileView";
        type Type = super::FileView;
        //type Type = crate::fileview::FileView;
        //type Type = FileView;
        type ParentType = gtk::DrawingArea;
    }

    impl FileView {
    //fn show_pdf(_area: &gtk::DrawingArea, ctx: &cairo::Context, _width: i32, _height: i32) {
        // /// Displays pages from the pdf file

        //fn show_pdf(&self, ctx: &cairo::Context, _width: i32, _height: i32) {
            ////let fp = "The C Programming Language.pdf";
            //let fp = Arguments::current().fp.clone();
            ////let fp = anno::current().fp;
            //let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
            //let page: PopplerPage = file.get_page(0).unwrap();
            //page.render(ctx);
        //}
    }

    // Trait shared by all GObjects
    impl ObjectImpl for FileView {

        fn constructed(&self, obj: &Self::Type) {
            // Call "constructed" on parent
            self.parent_constructed(obj);
            //self.width = 1280;
            //self.height = 720;
            //self.parent_constructed(self).set_draw_func(self.show_pdf);
            //obj.set_draw_func(self.show_pdf)

            let args = Arguments::current();
            self.path.replace(args.fp.clone());
            self.width.replace(1280);
            self.height.replace(720);
            obj.set_draw_func(show_pdf)

            //self.parent_constructed(obj).set_draw_func(self.show_pdf);

            // Connect to "clicked" signal of `button`
            //self.drawarea.set.connect_clicked(move |button| {
                //// Set the label to "Hello World!" after the button has been clicked on
                //button.set_label("Hello World!");
            //});
        }
    }
    impl WidgetImpl for FileView {} // Trait shared by all widgets
    impl DrawingAreaImpl for FileView {} // Trait shared by all buttons
}

//mod imp;

//use glib::Object;
//use gtk::glib;

glib::wrapper! {
    pub struct FileView(ObjectSubclass<imp::FileView>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

        //@extends gtk::DrawingArea, gtk::Widget,
        //@implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
        //@implements gtk::DrawingAreaExt, gtk::WidgetExt, glib::ObjectExt,
            //gtk::AccessibleExt, gtk::BuildableExt, gtk::ConstraintTargetExt,
            //gtk::DrawingAreaExtManual, gtk::WidgetExtManual, gtk::AccessibleExtManual;
        //@implements gtk::DrawingAreaExt, gtk::AccessibleExt, gtk::BuildableExt,
        //gtk::ConstraintTargetExt, gtk::DrawingAreaExtManual, gtk::WidgetExtManual, gtk::AccessibleExtManual;
        //@implements gtk::DrawingAreaExt, gtk::AccessibleExt, gtk::BuildableExt,
        //@implements DrawingAreaExt, WidgetExt, gtkAccessibleExt, BuildableExt, ConstraintTargetExt,
            //DrawingAreaExtManual, WidgetExtManual, AccessibleExtManual;
 

}

impl FileView {
//impl imp::FileView {
    pub fn new() -> Self {
        //Object::new(&[]).expect("Failed to create `FileView`.")
        //glib::Object::new(&[]).expect("Failed to create `FileView`.")
        Object::new(&[]).expect("Failed to create `FileView`.")
    }

    pub fn with_fp(fp: &str, width: i32, height: i32) -> Self {
        //Object::new(&[("path", &fp), ("width", &width), ("height", &height)]).expect("Failed to create `CustomButton`.")
        Object::new(&[ ("path", &fp), ("height", &height), ("width", &width) ])
            .expect("Failed to create `FileView`.")
    }

    //pub fn with_label(label: &str) -> Self {
        //Object::new(&[("label", &label)]).expect("Failed to create `FileView`.")
    //}
    // /// Displays pages from the pdf file
}

//impl Default for FileView {
    //fn default() -> Self {
        ////Self { path: Default::default() }
        //Self { inner: Default::default(), phantom: Default::default() }
    //}
//}

/// Displays pages from the pdf file
fn show_pdf(_area: &gtk::DrawingArea, ctx: &cairo::Context, _width: i32, _height: i32) {
    let fp = "The C Programming Language.pdf";
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    let page: PopplerPage = file.get_page(0).unwrap();
    page.render(ctx);
}
