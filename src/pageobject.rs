// Standard Library
use std::cell::Cell;

// Third Party Library
use glib::{Object, ParamSpec, ParamSpecInt, ParamSpecString, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

pub mod imp {
    use super::*;
    // Object holding the state
    #[derive(Default)]
    pub struct PageObject {
        title: Cell<String>,
        page: Cell<i32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for PageObject {
        const NAME: &'static str = "PageObject";
        type Type = super::PageObject;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for PageObject {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> =Lazy::new(|| vec![
                //ParamSpecInt::builder("page").build()
                ParamSpecString::builder("title").build(),
                ParamSpecInt::builder("page").build()
            ]);
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec,) {
            match pspec.name() {
                "page" => {
                    let input_number = value.get().expect("The value needs to be of type `i32`.");
                    self.page.replace(input_number);
                },
                "title" => {
                    let input_title = value.get().expect("The value needs to be of type `String`.");
                    self.title.replace(input_title);
                },

                //"number" => {
                    //let input_number =
                        //value.get().expect("The value needs to be of type `i32`.");
                    //self.number.replace(input_number);
                //}
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                //"number" => self.number.get().to_value(),
                "page" => self.page.get().to_value(),
                "title" => self.page.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}


glib::wrapper! {
    pub struct PageObject(ObjectSubclass<imp::PageObject>);
}

impl PageObject {
    pub fn new(title: String, page: i32) -> Self {
        Object::new(&[("title", &title), ("page", &page)]).expect("Failed to create `PageObject`.")
    }
}
