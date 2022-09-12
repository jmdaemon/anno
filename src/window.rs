use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};
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
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window: AnnoWindow = glib::Object::new(&[("application", application)])
            .expect("Failed to create AnnoWindow");

        // Setup the file view
        let fv = FV::new();
        window.set_child(Some(&fv));
        window
    }
}
