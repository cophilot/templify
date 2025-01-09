pub mod commands;
pub mod data;
pub mod env;
pub mod executer;
pub mod global_flag_storage;
pub mod logger;
pub mod placeholder_storage;
pub mod types;
pub mod utils;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logger::write_log(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::write_error(&format!($($arg)*));
    };
}
