use std::sync::{Arc, RwLock};

//pub mod imp {
    //use super::*;
    /**
    Atomically reference counted "singleton"
     */
    //#[derive(Default)]
    //pub struct ArgumentsInner {
        //verbose: bool,
        //fp: String,
    //}

    //#[derive(Default)]
    //pub struct Arguments {
        //inner: RwLock<ArgumentsInner>,
    //}
    #[derive(Default)]
    pub struct Arguments {
        pub verbose: bool,
        pub fp: String,
    }

    /** Immutable version
      * Usage:
    fn main() {
        Arguments { debug_mode: true }.make_current();
        if Arguments::current().debug_mode {
            // do something
        }
    }
    */
    impl Arguments {
        pub fn current() -> Arc<Arguments> {
            CURRENT_ARGS.with(|c| c.read().unwrap().clone())
        }
        pub fn make_current(self) {
            CURRENT_ARGS.with(|c| *c.write().unwrap() = Arc::new(self))
        }
    }

    thread_local! {
        static CURRENT_ARGS: RwLock<Arc<Arguments>> = RwLock::new(Default::default());
    }


    // Interior mutable version
    //#[derive(Default)]
    //pub struct Arguments {
        //inner: RwLock<ArgumentsInner>,
    //}

    //impl Arguments {
        //pub fn new() -> Arc<Arguments> {
            //Arc::new(Arguments { inner: RwLock::new(Default::default()) })
        //}
        //pub fn current() -> Arc<Arguments> {
            //CURRENT_CONFIG.with(|c| c.clone())
        //}
        //pub fn debug_mode(&self) -> bool {
            //self.inner.read().unwrap().debug_mode
        //}
        //pub fn set_debug_mode(&self, value: bool) {
            //self.inner.write().unwrap().debug_mode = value;
        //}
    //}

    //thread_local! {
        //static CURRENT_ARGS: Arguments = Default::default();
    //}
//}
