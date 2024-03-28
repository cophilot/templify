use crate::log;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::types::version_number::VersionNumber;
use crate::utils;

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

    let version = command.get_value_flag("version").clone();

    if !utils::version_control::is_newer_version_available() && version.is_empty() {
        log!("templify is already up to date.");
        return Status::ok();
    }

    if !version.is_empty() {
        if !VersionNumber::new().parse_from_string(&version) {
            return Status::error(format!("Invalid version number: {}", version));
        }
        log!("Updating templify to version {}...", version);
    } else {
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
