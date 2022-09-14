// Imports
use crate::anno::Arguments;

// Standard Library
use std::cell::{Cell, RefCell};
use std::rc::Rc;

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
        pub path: Rc<RefCell<String>>,
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

            let args = Arguments::current();
            //Arguments::make_current(args.to_owned());
            //self.path.replace(args.fp.clone());
            self.path.replace(args.fp.to_owned());

            //self.width.replace(1280);
            //self.height.replace(720);
            self.width.replace(1440);
            self.height.replace(900);

            //obj.set_draw_func(obj.show_pdf)
            obj.set_draw_func(show_pdf)
            //obj.set_draw_func(super::FileView::show_pdf);
            //self.set_draw_func(super::FileView::show_pdf);
            //obj.set_draw_func(&obj.show_pdf);
            //obj.set_draw_func(self, super::FileView::show_pdf);
            //gtk::DrawingArea::set_draw_func(&self, super::FileView::show_pdf);
            //gtk::DrawingArea::set_draw_func(&obj.upcast::<gtk::DrawingArea>(), super::FileView::show_pdf);
        }
    }
    // Trait shared by all widgets
    impl WidgetImpl for FileView {}

    // Trait shared by all drawing areas
    pub trait CanDowncast<T> {}
    //impl<Super: IsA<Super>, Sub: IsA<Super>> CanDowncast<Sub> for Super {}
    //impl<Super, Sub> CanDowncast<Sub> for Super { }
        //fn some_func<T: DrawingArea>(foo: T) {
        //}
    //impl<FileView, Sub> CanDowncast<Sub> for FileView { }
    impl<Super: IsA<Super>, Sub: IsA<Super>> CanDowncast<Sub> for Super {}

    impl DrawingAreaImpl for FileView {
        //fn is_a_drawing_area<W: IsA<gtk::Object> + IsA<gtk::Widget> + Clone>(widget: &W) -> bool {
            //widget.clone().upcast::<gtk::Widget>().is::<gtk::DrawingArea>()
        //}
    }
    //impl gtk::CanUpcast<gtk::DrawingArea> for FileView {}
    //impl Cast for FileView {}

}

glib::wrapper! {
    pub struct FileView(ObjectSubclass<imp::FileView>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

//impl<Super, Sub> CanDowncast<Sub> for Super { }

impl FileView {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `FileView`.")
    }
}
/// Displays pages from the pdf file
fn show_pdf(_area: &gtk::DrawingArea, ctx: &cairo::Context, _width: i32, _height: i32) {
//fn show_pdf(self: &Self, ctx: &cairo::Context, _width: i32, _height: i32) {
    let fp = "The C Programming Language.pdf";
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
    println!("{}", fp);
    let file: PopplerDocument = PopplerDocument::new_from_file(fp, "").unwrap();
    println!("{:?}", file.get_metadata());
    let page: PopplerPage = file.get_page(0).unwrap();
    page.render(ctx);
    //page.render_for_printing(ctx);
}
