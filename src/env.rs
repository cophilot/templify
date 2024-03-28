use std::env;

pub static mut BASE_COMMAND_NAME: String = String::new();

/// Returns if the current OS is Windows
pub fn is_windows() -> bool {
    env::consts::OS == "windows"
}

/// Returns if the current OS is macOS
pub fn is_mac() -> bool {
    env::consts::OS == "macos"
}
