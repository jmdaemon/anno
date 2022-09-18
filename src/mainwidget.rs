// Imports
use crate::fileview::imp::FileView;
use crate::fileview::FileView as FV;
use crate::pageobject::PageObject;

// Standard Library
use std::cell::RefCell;

// Third Party Libraries
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

pub mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/jmdaemon/anno/../resources/gtk/mainwidget.ui")]
    pub struct MainWidget {
        // Template widgets
        #[template_child]
        pub gc_toolbar: TemplateChild<gtk::CenterBox>,
        #[template_child]
        pub gc_pgnumbar: TemplateChild<gtk::CenterBox>,
        #[template_child]
        pub nb_sidebar: TemplateChild<gtk::Notebook>,

        // Custom Widgets

        /// FileView
        pub fileview: FileView,
        pub lbl: gtk::Label,

        /// Table of Contents
        pub lv_toc: RefCell<gtk::ListView>,
        pub lbl_toc: RefCell<gtk::Label>,
        //pub lv_toc: gtk::ListView,
        //pub filesidebar: FileSidebar,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWidget {
        const NAME: &'static str = "MainWidget";
        type Type = super::MainWidget;
        type ParentType = gtk::Grid;

        // Initialize template
        fn class_init(klass: &mut Self::Class) { Self::bind_template(klass); }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) { obj.init_template(); }
    }

    impl ObjectImpl for MainWidget {
        fn constructed(&self, obj: &Self::Type) {
            let slif_toc = gtk::SignalListItemFactory::new();
            let toc_model = gio::ListStore::new(PageObject::static_type());
            let ss_toc = gtk::SingleSelection::new(Some(&toc_model));
            let lv_toc = gtk::ListView::new(Some(&ss_toc), Some(&slif_toc));
            let lbl_toc = gtk::Label::new(Some(&"Table of Contents"));
            
            self.lv_toc.replace_with(|_| lv_toc);
            self.lbl_toc.replace_with(|_| lbl_toc);

            //self.lv_toc = lv_toc

            //self.mainwidget.nb_sidebar.append_page(&lv_toc, Some(&lbl_toc));

            //self.mainwidget.nb_sidebar.append_page(&lv_toc, Some(&lbl_toc));
            //self.mainwidget.nb_sidebar.append_page(&lv_toc, Some(&lbl_toc));
            //window.imp().mainwidget.nb_sidebar.append_page(&lv_toc, Some(&lbl_toc));
        }
    }
    impl WidgetImpl for MainWidget {}
    impl GridImpl for MainWidget {}
}

glib::wrapper! {
    pub struct MainWidget(ObjectSubclass<imp::MainWidget>)
        @extends gtk::Widget, gtk::Grid,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MainWidget {
    pub fn new() -> Self {
        let mainwidget: MainWidget = glib::Object::new(&[])
            .expect("Failed to create MainWidget");

        // Setup the file view
        let fv = FV::new();

        // Add it to the main grid
        fv.set_height_request(860);
        fv.set_width_request(600);
        mainwidget.attach(&fv, 1, 1, 1, 1);

        let label = gtk::Label::new(Some("Table of Contents"));
        mainwidget.imp().nb_sidebar.append_page(&label, Some(&label));
        mainwidget
    }
}
