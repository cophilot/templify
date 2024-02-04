use std::env;

pub static mut BASE_COMMAND_NAME: String = String::new();

pub fn is_windows() -> bool {
    return env::consts::OS == "windows";
}
