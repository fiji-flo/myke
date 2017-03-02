extern crate itertools;

use std::mem;
use std::sync::Mutex;

#[macro_export]
macro_rules! capture {
    () => ($crate::out(""));
    ($fmt:expr) => ($crate::out(format!($fmt)));
    ($fmt:expr, $($arg:tt)*) => ($crate::out(format!($fmt, $($arg)*)));
}
static mut OUT: *const Out = &Void;

pub trait Out: Sync + Send {
    fn out(&self, buf: String);
    fn dump(&self) -> Option<String>;
}

struct Void;

impl Out for Void {
    fn out(&self, _: String) {}
    fn dump(&self) -> Option<String> {
        None
    }
}

pub fn out(buf: String) {
    unsafe {
        (*OUT).out(buf);
    }
}

pub fn set(cap: Box<Out>) {
    unsafe {
        OUT = mem::transmute(cap);
    }
}

pub fn dump() -> Option<String> {
    unsafe { (*OUT).dump() }
}

pub fn void() {
    unsafe {
        OUT = &Void;
    }
}

pub struct Cappy {
    pub buf: Mutex<Vec<String>>,
}

impl Out for Cappy {
    fn out(&self, frag: String) {
        let _ = self.buf.lock().unwrap().push(frag);
    }
    fn dump(&self) -> Option<String> {
        match self.buf.lock() {
            Ok(mut b) => Some(itertools::join(b.drain(0..), "")),
            Err(_) => None,
        }
    }
}
