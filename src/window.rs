// Imports
use anno::mainwidget::imp::MainWidget;
use anno::mainwidget::MainWidget as MW;

// Third Party Libraries
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

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
        //pub fileview: FileView,
        pub mainwidget: MainWidget,

        //pub lv_toc: gtk::ListView,
        //pub slif_toc: gtk::SignalListItemFactory,
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

    impl ObjectImpl for AnnoWindow {

    }
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

        // Setup the main widget
        let mw = MW::new();
        //self.mainwidget.nb_sidebar.append_page(&lv_toc, Some(&lbl_toc));
        mw.imp().nb_sidebar.append_page(&mw.imp().lv_toc.take(), Some(&mw.imp().lbl_toc.take()));

        window.set_child(Some(&mw));

        // Setup the header bar
        // We need to setup an item factory, model
        //let toc_model = gio::ListStore::new(gtk::GObject::static_type());

        //let slif_toc = gtk::SignalListItemFactory::new();
        //let toc_model = gio::ListStore::new(PageObject::static_type());
        //let ss_toc = gtk::SingleSelection::new(Some(&toc_model));
        //let lv_toc = gtk::ListView::new(Some(&ss_toc), Some(&slif_toc));
        //let lbl_toc = gtk::Label::new(Some(&"Table of Contents"));
        //window.imp().mainwidget.nb_sidebar.append_page(&lv_toc, Some(&lbl_toc));

        //window.imp().lv_toc = lv_toc;
        window
    }
}
