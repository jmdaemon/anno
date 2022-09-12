//mod fileview;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};
//use crate::fileview::FileView;
//use ::FileView;
//use fileview::FileView;
//use crate::fileview::FileView;
//use fileview::imp::FileView;
//use crate::fileview::imp::FileView;
//use anno::fileview::imp::FileView;
//use anno::fileview::FileView;
use anno::fileview::imp::FileView;
use anno::fileview::FileView as FV;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/jmdaemon/anno/../resources/gtk/window.ui")]
    pub struct AnnoWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        pub fileview: FileView,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AnnoWindow {
        const NAME: &'static str = "AnnoWindow";
        type Type = super::AnnoWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
            //obj.set_instance_data(Self, FileView);
        }


    }

    impl ObjectImpl for AnnoWindow {}
    impl WidgetImpl for AnnoWindow {}
    impl WindowImpl for AnnoWindow {}
    impl ApplicationWindowImpl for AnnoWindow {}
}

glib::wrapper! {
    pub struct AnnoWindow(ObjectSubclass<imp::AnnoWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AnnoWindow {
    //pub fn get_fileview(&self) {
        //self.inner fileview
    //}
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window: AnnoWindow = glib::Object::new(&[("application", application)])
            .expect("Failed to create AnnoWindow");

        //let window_priv = window.imp().fileview = FileView::with_fp();
        
        //let window_priv = window.imp().fileview = FileView::with_fp();
        //window_priv.inner.Err
        //window.set_child(Some(&window.imp().instance()));
        //window.set_child(Some(&window.imp().fileview.instance()));

        //window.set_child(Some(&window.imp().fileview.instance()));
        //window.set_child(Some(&window.imp().fileview));
        //let fv = FV::with_fp("The C Programming Language.pdf", 1280, 720);
        let fv = FV::new();
        //window.set_child(Some(FV));
        window.set_child(Some(&fv));

        //let &mut win = window.imp();

        // Setup the file view
        //win.fileview = FileView::new();
        //win.fileview.height = 1280;
        //win.fileview.width = 720;
        window
        //window
        //window.imp().fileview;
        //window.fileview = glib::Object::new(FileView);
        //window.imp().fileview = glib::Object::new(FileView);
        //window.imp().fileview = FileView::new();
        //window.imp().fileview = Default::default();
        //window.imp().fileview = FileView::new().upcast(imp::FileView);
        //window.imp().fileview = FileView::new();
        //windo
        //window
    }
}
