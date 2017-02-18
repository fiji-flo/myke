use std::mem;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

#[macro_export]
macro_rules! capture {
    () => ($crate::out("\n"));
    ($fmt:expr) => ($crate::out(format!(concat!($fmt, "\n"))));
    ($fmt:expr, $($arg:tt)*) => ($crate::out(format!(concat!($fmt, "\n"), $($arg)*)));
}
static mut OUT: *const Out = &Void;

pub trait Out: Sync+Send {
    fn out(&self, buf: String);
}

struct Void;

impl Out for Void {
    fn out(&self, _: String) {}
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

pub fn void() {
    unsafe {
        OUT = &Void;
    }
}

pub struct Cappy {
    pub tx: Mutex<Sender<String>>,
}

impl Out for Cappy {
    fn out(&self, frag: String) {
        let _ = self.tx.lock().unwrap().send(frag);
    }
}
