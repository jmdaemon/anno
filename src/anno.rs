use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};

/**
  * Arguments Singleton Struct
  * Struct with a mutex that locks for callers when the struct's methods are called
  */
#[derive(Debug, Default)]
pub struct ArgumentsState {
    pub verbose: bool,
    pub fp: String,
}

pub struct Arguments {
    inner: &'static Mutex<ArgumentsState>,
}

pub struct ArgumentsLock<'a> {
    inner: MutexGuard<'a, ArgumentsState>,
}


pub static ARGS: OnceCell<Mutex<ArgumentsState>> = OnceCell::new();

pub fn global() -> Arguments {
    Arguments {
        inner: ARGS.get_or_init(|| Mutex::new(Default::default() )),
    }
}

// Lock for multiple callers
impl ArgumentsLock<'_> { }

impl Arguments {
    pub fn lock(&self) -> ArgumentsLock<'_> { 
        ArgumentsLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }

    pub fn get_path(&self) -> String {
        self.lock().inner.fp.clone()
    }

    pub fn set_path(&mut self, path: &str) {
        self.lock().inner.fp = path.to_string();
    }
}
