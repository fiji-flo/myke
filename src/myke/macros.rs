#[macro_export]
macro_rules! val {
    ($yml:ident, $yml_key:expr, $default:expr) =>
        (String::from($yml[$yml_key].as_str().unwrap_or($default)))
}
