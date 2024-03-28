use crate::log;
use crate::types::version_number::VersionNumber;
use crate::{env, utils};

/// This function is used to update templify to a version or to the latest version if an empty string is passed.
pub fn update(v: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut binary_ending = "";
    if env::is_windows() {
        binary_ending = ".exe";
    }
    if env::is_mac() {
        binary_ending = "-macos";
    }

    let mut version = v;

    if version.is_empty() {
        version = get_latest_version();
    }

    if version.starts_with('v') {
        version = version[1..].to_string();
    }

    let url = format!(
        "https://github.com/cophilot/templify/releases/download/{}/tpy{}",
        version, binary_ending
    );
    // download the new binary and save it somewhere temporarily
    let response = reqwest::blocking::get(url).unwrap();

    // check if the download was successful
    if response.status() != 200 {
        if response.status() == 404 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Could not download the new binary. The version {} does not exist.",
                    version
                ),
            )));
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Could not download the new binary. Status code: {}",
                    response.status()
                ),
            )));
        }
    }

    let mut response = response;
    let mut dest = {
        let mut d = std::env::temp_dir();
        d.push("tpy");
        std::fs::File::create(d)?
    };
    std::io::copy(&mut response, &mut dest)?;

    // replace the current binary with the new one
    let new_binary = format!("{}/tpy", std::env::temp_dir().to_str().unwrap());

    self_replace::self_replace(&new_binary)?;
    std::fs::remove_file(&new_binary)?;
    Ok(())
}

/// This function is used to print a message if a new version of templify is available.
pub fn print_update_message() {
    if !utils::functions::check_internet_connection() {
        return;
    }
    if is_newer_version_available() {
        log!(" ");
        log!(
            "A new version of templify is available: {} -> {}",
            env!("CARGO_PKG_VERSION"),
            get_latest_version()
        );
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };
        log!(
            "Run `{} update` to update to the newest version.",
            command_name
        );
    }
}

/// This function is used to check if a newer version of templify is available.
pub fn is_newer_version_available() -> bool {
    let mut current_version = VersionNumber::new();
    current_version.parse_from_string(env!("CARGO_PKG_VERSION"));

    let mut latest_version = VersionNumber::new();
    if !latest_version.parse_from_string(&get_latest_version()) {
        return false;
    }
    latest_version.is_newer(&current_version)
}

/// This function is used to get the latest version of templify.
pub fn get_latest_version() -> String {
    let latest_version_url =
        "https://raw.githubusercontent.com/cophilot/templify/master/.phil-project";

    let response = reqwest::blocking::get(latest_version_url).unwrap();
    let response_text = response.text().unwrap();

    for line in response_text.lines() {
        if line.to_lowercase().starts_with("version:") {
            let latest_version = line.split(':').collect::<Vec<&str>>()[1].trim();
            return latest_version.to_string();
        }
    }
    "".to_string()
}
