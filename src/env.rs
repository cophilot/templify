use std::env;

pub fn is_linux() -> bool {
    return env::consts::OS == "linux";
}
pub fn is_windows() -> bool {
    return env::consts::OS == "windows";
}
