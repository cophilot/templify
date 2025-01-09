use crate::log;
use crate::placeholder_storage::get_all_placeholders;
use crate::types::command::Command;
use crate::types::status::Status;

/// Definition of the placeholder command.
pub fn definition() -> Command {
    Command::new(
        vec!["placeholder".to_string(), "ph".to_string()],
        placeholder,
        "Print all available placeholders.".to_string(),
    )
}

pub fn placeholder(_command: &Command) -> Status {
    log!("Available placeholders:");
    for ph in get_all_placeholders() {
        log!(
            "  $${}$$ - {}: {}",
            ph.name,
            ph.description,
            (ph.get_value)()
        );
    }

    Status::ok()
}
