use crate::log;
use crate::types::command::Command;
use crate::types::status::Status;

/// The definition of the version command.
pub(crate) fn definition() -> Command {
    Command::new(
        vec!["version".to_string(), "v".to_string()],
        version,
        "Print the current version of templify.".to_string(),
    )
}

/// The version command is used to print the current version of templify.
pub(crate) fn version(_command: &Command) -> Status {
    log!("templify version {}", env!("CARGO_PKG_VERSION"));
    Status::ok()
}
