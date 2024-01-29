use crate::env;

pub fn update() -> Result<(), Box<dyn std::error::Error>> {
    let mut binary_ending = "";
    if env::is_windows() {
        binary_ending = ".exe";
    }

    let url = format!(
        "
        https://github.com/cophilot/templify/releases/download/{}/tpy{}",
        get_latest_version(),
        binary_ending
    );
    // download the new binary and save it somewhere temporarily
    let mut response = reqwest::blocking::get(url)?;
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
    if is_newer_version_available() {
        println!("");
        println!(
            "A new version of Templify is available: {} -> {}",
            env!("CARGO_PKG_VERSION"),
            get_latest_version()
        );
        println!("Run `tpy update` to update to the newest version.");
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
