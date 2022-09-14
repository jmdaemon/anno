use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};
//use crate::anno::fileview::imp::FileView;
//use crate::anno::fileview::FileView as FV;
use crate::fileview::imp::FileView;
use crate::fileview::FileView as FV;

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
        pub fileview: FileView,
        pub lbl: gtk::Label,
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

    impl ObjectImpl for MainWidget {}
    impl WidgetImpl for MainWidget {}
    impl GridImpl for MainWidget {}
}

glib::wrapper! {
    pub struct MainWidget(ObjectSubclass<imp::MainWidget>)
        @extends gtk::Widget, gtk::Grid,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MainWidget {
    //pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
    pub fn new() -> Self {
        //let mainwidget: MainWidget = glib::Object::new(&[("application", application)])
        let mainwidget: MainWidget = glib::Object::new(&[])
            .expect("Failed to create MainWidget");

        // Setup the file view
        let fv = FV::new();
        //window.set_child(Some(&fv));
        //mainwidget.buildable_id();

        // Add main file view
        //mainwidget.attach(&fv, 1, 1, 2, 1);
        //mainwidget.attach(&fv, 1, 1, 1, 1);
        //mainwidget.imp().nb_sidebar.append_page(&fv, Some(&"FileView".to_string()));
        //mainwidget.imp().lbl = gtk::Label::new(Some("FileView"));
        let label = gtk::Label::new(Some("Table of Contents"));
        //mainwidget.imp().nb_sidebar.append_page(&fv, None);
        //mainwidget.imp().nb_sidebar.append_page(&fv, lbl);
        mainwidget.imp().nb_sidebar.append_page(&label, Some(&label));

        //fv.imp().
        fv.set_height_request(860);
        fv.set_width_request(600);

        //mainwidget.attach(&fv, 1, 1, 2, 1);
        //mainwidget.attach(&fv, 1, 1, 1, 1);
        mainwidget.attach(&fv, 1, 1, 1, 1);
        mainwidget
    }
}
