use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::sync::MutexGuard;

// Arguments Singleton Struct

#[derive(Debug, Default)]
pub struct ArgumentsState {
    pub verbose: bool,
    pub fp: String,
}

// TODO: Make more generic

pub struct Arguments {
    inner: &'static Mutex<ArgumentsState>,
}

pub static ARGS: OnceCell<Mutex<ArgumentsState>> = OnceCell::new();
pub fn global() -> Arguments {
    Arguments {
        inner: ARGS.get_or_init(|| Mutex::new(Default::default() )),
    }
}

pub struct ArgumentsLock<'a> {
    inner: MutexGuard<'a, ArgumentsState>,
}

// Lock for multiple callers
impl ArgumentsLock<'_> {
    pub fn get_path(&mut self) -> String {
        self.inner.fp.clone()
    }

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

    pub fn get_path(&self) -> String {
        self.lock().inner.fp.clone()
    }

    pub fn set_path(&mut self, path: &str) {
        self.lock().inner.fp = path.to_string();
    }
}
