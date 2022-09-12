use std::sync::{Arc, RwLock};

/**
Atomically reference counted "singleton"
 */
#[derive(Default)]
pub struct Arguments {
    pub verbose: bool,
    pub fp: String,
}

/** 
  * Usage:
  * fn main() {
  *     Arguments { debug_mode: true }.make_current();
  *     if Arguments::current().debug_mode {
  *         // do something
  *     }
  * }
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
