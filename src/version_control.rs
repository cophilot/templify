use crate::{env, utils};

pub fn update(v: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut binary_ending = "";
    if env::is_windows() {
        binary_ending = ".exe";
    }

    let mut version = v;

    if version == "" {
        version = get_latest_version();
    }

    if version.starts_with("v") {
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

pub fn print_update_message() {
    if !utils::check_internet_connection() {
        return;
    }
    if is_newer_version_available() {
        println!("");
        println!(
            "A new version of templify is available: {} -> {}",
            env!("CARGO_PKG_VERSION"),
            get_latest_version()
        );
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };
        println!(
            "Run `{} update` to update to the newest version.",
            command_name
        );
    }
}

pub fn is_newer_version_available() -> bool {
    let current_version = env!("CARGO_PKG_VERSION");
    let latest_version = get_latest_version();

    if latest_version == "" {
        return false;
    }

    if current_version == latest_version {
        return false;
    }

    return true;
}

pub fn get_latest_version() -> String {
    let latest_version_url =
        "https://raw.githubusercontent.com/cophilot/templify/master/.phil-project";

    let response = reqwest::blocking::get(latest_version_url).unwrap();
    let response_text = response.text().unwrap();

    for line in response_text.lines() {
        if line.to_lowercase().starts_with("version:") {
            let latest_version = line.split(":").collect::<Vec<&str>>()[1].trim();
            return latest_version.to_string();
        }
    }
    return "".to_string();
}
