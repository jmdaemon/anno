//#[macro_use]
//extern crate lazy_static; // 1.1.0

//use std::sync::{Arc, RwLock};
//use std::{env, io, sync::OnceCell};
//use once_cell::sync::Lazy;
//use lazy_static::lazy_static;
//use std::sync::OnceCell;
//use std::io;
//use once_cell::sync::OnceCell;

//use lazy_static::lazy_static;

//use arc_swap::ArcSwap;
//use std::lazy::Lazy;
//use std::

//[eature(once_cell)] // Nightly feature, see https://github.com/rust-lang/rust/issues/74465
//use std::lazy::SyncOnceCell;
//use std::sync::SyncOnceCell;

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;

#[derive(Debug, Default)]
pub struct ArgumentsState {
    pub verbose: bool,
    pub fp: String,
}

pub struct Arguments {
    inner: &'static Mutex<ArgumentsState>,
}

//pub fn get() -> Arguments {

pub static ARGS: OnceCell<Mutex<ArgumentsState>> = OnceCell::new();
pub fn global() -> Arguments {
    //static INSTANCE: SyncOnceCell<Mutex<ArgumentsState>> = SyncOnceCell::new();
    Arguments {
        //inner: ARGS.get_or_init(|| Mutex::new(Default::default() )),
        //inner: ARGS.get().expect("Arguments was not initialized."),
        inner: ARGS.get_or_init(|| Mutex::new(Default::default() )),
    }
}

pub struct ArgumentsLock<'a> {
    inner: MutexGuard<'a, ArgumentsState>,
}

// If one wants to support client to lock for multiple calls at once insteaf of per call
impl ArgumentsLock<'_> {
    //pub fn add_value(&mut self, value: u8) {
        //self.inner.value += value;
    //}

    pub fn set_path(&mut self, path: &str) {
        self.inner.fp = path.to_string();
    }
}

impl Arguments {
    pub fn lock(&self) -> ArgumentsLock<'_> { 
        ArgumentsLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }

    //pub fn get_value(&self) -> u8 {
    pub fn get_path(&self) -> String {
        //self.lock().inner.value
        self.lock().inner.fp.clone()
    }

    //pub fn add_value(&mut self, value: u8) {
        //self.lock().inner.value += value;
    //}
    pub fn set_path(&mut self, path: &str) {
        self.lock().inner.fp = path.to_string();
    }
}

// Usage
//fn main() {
    //let singleton = get();
    //{
        //let mut singleton_lock = singleton.lock();
        //singleton_lock.add_value(1);

        //// Do more things with the singleton here
    //} // lock is dropped here.

    //let handle = thread::spawn(move || {
        //let mut singleton = get();
        //singleton.add_value(1);
    //});
    //handle.join().unwrap();

    //assert_eq!(2, singleton.get_value());
//}

//#[derive(Debug, Default)]
//pub struct Arguments {
    //pub verbose: bool,
    //pub fp: String,
//}
////pub struct Arguments { ... }

//pub static ARGS: OnceCell<Arguments> = OnceCell::new();

////lazy_static! {
    ////pub static ref ARGS: Arguments = Arguments { verbose: false, fp: "".to_string() };
////}

//impl Arguments {
    //pub fn from_args(args: Arguments) {
        //////ARGS.set(args).unwrap();
        //ARGS.set(args).unwrap();
    //}

    ////pub fn set(args: Arguments) {
        //////ARGS = args
        //////lazy_static::initialize(&args);
        //////ARGS.initialize(&args);
    ////}

    //pub fn global() -> &'static Arguments {
        //ARGS.get().expect("Arguments is not initialized.")
        ////&ARGS
    //}
//}
    //fn from_args(args: Arguments) -> Result<Arguments, std::io::Error> {
        //let result = match ARGS.set(args) {
            //Ok(result) => Ok(result),
            //Err(e) => return Err(e)
        //};
    //}

    //fn from_cli(args: env::Args) -> Result<Arguments, std::io::Error> { ... }

//fn main() {
    //let logger = Logger::from_cli(env::args()).unwrap();

    //// Note how we use locally-created value for initialization.
    //INSTANCE.set(logger).unwrap();

    //// use `Logger::global()` from now on
//}



/* *
Atomically reference counted "singleton"
 */
//#[derive(Default)]
//pub struct Arguments {
    //pub verbose: bool,
    //pub fp: String,
//}

/* * 
  * Usage:
  * fn main() {
  *     Arguments { debug_mode: true }.make_current();
  *     if Arguments::current().debug_mode {
  *         // do something
  *     }
  * }
  */

//impl Arguments {
    //pub fn current() -> Arc<Arguments> {
        ////CURRENT_ARGS.with(|c| c.read().unwrap().clone())
    //}
    //pub fn make_current(self) {
        //CURRENT_ARGS.with(|c| *c.write().unwrap() = Arc::new(self))
    //}
//}


//impl Arguments {}

//lazy_static! {
    //pub static ref CURRENT_ARGS: RwLock<Arguments> = RwLock::new(Default::default());
//}
    //pub static CURRENT_ARGS: RwLock<Arc<Arguments>> = RwLock::new(Default::default());
    //pub static ref CURRENT_ARGS: RwLock<Arc<Arguments>> = RwLock::new(Default::default());

//thread_local! {
    //static CURRENT_ARGS: RwLock<Arc<Arguments>> = RwLock::new(Default::default());
//}

//pub static CURRENT_ARGS: Lazy<ArcSwap<Arguments>> = Lazy::new(|| {
    //ArcSwap::from_pointee(Arguments::default())
//});
    //ArcSwap::from_pointee(Arguments)
    //ArcSwap::from_pointee(Default::default())
    //ArcSwap::from_pointee(RoutingTable)
