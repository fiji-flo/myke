#[macro_export]
macro_rules! val {
    ($yml:ident, $yml_key:expr, $default:expr) =>
        (String::from($yml[$yml_key].as_str().unwrap_or($default)))
}

#[cfg(not(test))]
#[macro_export]
macro_rules! out {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(test)]
#[macro_export]
macro_rules! out {
    () => (capture!());
    ($fmt:expr) => (capture!($fmt));
    ($fmt:expr, $($arg:tt)*) => (capture!($fmt, $($arg)*));
}
