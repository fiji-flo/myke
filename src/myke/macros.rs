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

#[macro_export]
macro_rules! concat_task {
    ($self_:ident $update:ident $field:ident) => {
        $self_.$field = match ($self_.$field.clone(), $update.$field.clone()) {
            #[cfg(not(windows))]
            (Some(s), Some(u)) => Some(format!("{} ; {}", s, u)),
            #[cfg(windows)]
            (Some(s), Some(u)) => Some(format!("{} & {}", s, u)),
            (Some(s), None) => Some(s),
            (None, Some(u)) => Some(u),
            _ => None
        }
    }
}

#[cfg(not(test))]
#[macro_export]
macro_rules! out {
    () => (println!());
    ($fmt:expr) => (println!($fmt));
    ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
}

#[cfg(test)]
#[macro_export]
macro_rules! out {
    () => (capture!());
    ($fmt:expr) => (capture!($fmt));
    ($fmt:expr, $($arg:tt)*) => (capture!($fmt, $($arg)*));
}

#[macro_export]
macro_rules! info {
    () => ({
        use ::colored::*;
        out!("•".blue());
    });
    ($str:expr) => ({
        use ::colored::*;
        out!("{} {}", "•".blue(), $str);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        use ::colored::*;
        out!("{} {}", "•".blue(), format_args!($fmt, $($arg)*));
    });
}

#[macro_export]
macro_rules! error {
    () => ({
        use ::colored::*;
        out!("⨯".red());
    });
    ($str:expr) => ({
        use ::colored::*;
        out!("{} {}", "⨯".red(), $str);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        use ::colored::*;
        out!("{} {}", "⨯".red(), format_args!($fmt, $($arg)*));
    });
}
