#[macro_export]
macro_rules! val {
    ($yml:ident, $yml_key:expr, $default:expr) =>
        (String::from($yml[$yml_key].as_str().unwrap_or($default)))
}

#[macro_export]
macro_rules! val_opt {
    ($yml:ident, $yml_key:expr) =>
        ($yml[$yml_key].as_str().and_then(|s| {Some(s.to_owned())}))
}

#[macro_export]
macro_rules! update_task {
    ($self_:ident $update:ident $field:ident) => {
        if $self_.$field.is_none() {
            $self_.$field = $update.$field.clone();
        }
    }
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