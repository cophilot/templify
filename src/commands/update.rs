use crate::log;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::types::version_number::VersionNumber;
use crate::utils;
use crate::utils::version_control::get_latest_version;

/// The definition of the update command.
pub(crate) fn definition() -> Command {
    let mut update_command = Command::new(
        vec!["update".to_string()],
        update,
        "Update templify to the latest version.".to_string(),
    );

    update_command.add_flag(Flag::new_value_flag(
        vec!["version".to_string(), "v".to_string()],
        "".to_string(),
        "Update to a specific version.".to_string(),
    ));

    update_command
}

/// The update command is used to update templify to the latest version.
pub(crate) fn update(command: &Command) -> Status {
    if !utils::functions::check_internet_connection() {
        return Status::error("You need a internet connection to update templify.".to_string());
    }
    let mut current_version = VersionNumber::new();
    current_version.parse_from_string(env!("CARGO_PKG_VERSION"));

    let version = command.get_value_flag("version").clone();

    if !utils::version_control::is_newer_version_available() && version.is_empty() {
        log!("templify is already up to date.");
        return Status::ok();
    }

    if !version.is_empty() {
        let mut version_number = VersionNumber::new();
        if !version_number.parse_from_string(&version) {
            return Status::error(format!("Invalid version number: {}", version));
        }
        if version_number.is_older(&VersionNumber::from_string("1.0.0")) {
            return Status::error("Versions older than 1.0.0 are not supported.".to_string());
        }

        if current_version.is_major_update(&version_number) {
            log!(" ");
            log!("Warning, Updating Major Version can have breaking changes. Please consider reading documentation after update.");
            log!(" ");
            log!("To get more information please visit: https://github.com/cophilot/templify/blob/master/CHANGELOG.md");
            log!(" ");
        }

        log!("Updating templify to version {}...", version);
    } else {
        let mut latest_verion = VersionNumber::new();
        latest_verion.parse_from_string(&get_latest_version());

        if current_version.is_major_update(&latest_verion) {
            log!(" ");
            log!("Warning, Latest Version has major release. Please consider reading documentation after update");
            log!(" ");
            log!("To get more information please visit: https://github.com/cophilot/templify/blob/master/CHANGELOG.md");
            log!(" ");
        }

        log!("Updating templify...");
    }

    let st = utils::version_control::update(version.clone());
    if st.is_err() {
        return Status::error(format!("{}", st.err().unwrap()));
    }

    log!("templify updated successfully.");

    if version.is_empty() {
        log!("Visit https://templify.philipp-bonin.com/#/whats-new to see what's new.");
    }

    std::process::exit(0);
}
