use web_sys::console;

pub fn console_log(s: &str) {
    console::log_1(&s.into());
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (crate::debug::console_log(&format!($($t)*)))
}

pub use log;
