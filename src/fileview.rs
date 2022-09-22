// Imports
use crate::anno::global;

// Standard Library
use std::cell::{Cell, RefCell};
use std::sync::Arc;

// Third Party Library
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use glib::Object;
use poppler::{PopplerDocument, PopplerPage};

pub mod imp {
    use super::*;

    // Object holding the state
    #[derive(Debug, Default)]
    pub struct FileView {
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

    // Trait shared by all GObjects
    impl ObjectImpl for FileView {

        fn constructed(&self, obj: &Self::Type) {
            // Call "constructed" on parent
            self.parent_constructed(obj);

            // Populate members
            let args = global();
            let fp = args.get_path();
            self.path.replace(fp.clone());

            self.width.replace(1440);
            self.height.replace(900);

            obj.set_draw_func(show_pdf);
            println!("Constructed FileView");
            println!("Filepath, {}", fp);
        }
    }
    // Trait shared by all widgets
    impl WidgetImpl for FileView {}

    // Trait shared by all drawing areas
    impl DrawingAreaImpl for FileView {}
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
    let args = global();
    let fp = args.get_path();
    println!("showpdf: fp: {}", fp);
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    println!("{:?}", file.get_metadata());
    let page: PopplerPage = file.get_page(0).unwrap();
    page.render(ctx);
    //page.render_for_printing(ctx);
}
