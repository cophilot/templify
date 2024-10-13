use crate::commands::load::URLType;
use crate::log;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::utils;

/// The definition of the init command.
pub(crate) fn definition() -> Command {
    let mut init_command = Command::new(
        vec!["init".to_string(), "i".to_string()],
        init,
        "Initialize templify in the current directory.".to_string(),
    );

    init_command.add_flag(Flag::new_bool_flag(
        vec!["offline".to_string(), "o".to_string()],
        "Do not fetch the example template from the internet.".to_string(),
    ));
    init_command.add_flag(Flag::new_bool_flag(
        vec!["blank".to_string(), "b".to_string()],
        "Initialize only a blank .templates folder.".to_string(),
    ));

    init_command
}

/// The init command is used to initialize templify in the current directory.
pub(crate) fn init(command: &Command) -> Status {
    log!("Initializing templify...");

    // check if .templates folder exists
    if std::path::Path::new(".templates").exists() {
        return Status::error("templify is already initialized in this project.".to_string());
    }

    std::fs::create_dir(".templates").unwrap();

    if command.get_bool_flag("blank") {
        log!("templify initialized successfully.");
        return Status::ok();
    }
    std::fs::write(
        ".templates/README.md",
        crate::data::get_init_readme_content(),
    )
    .unwrap();

    // check if there is an internet connection
    if utils::functions::check_internet_connection() && !command.get_bool_flag("offline") {
        log!("Loading example template from templify-vault...");
        utils::template_handler::load_remote_template_collection(
            ".templates",
            "https://github.com/cophilot/templify-vault/tree/main/Example",
            true,
            &URLType::GitHub,
        );
    }
    log!("templify initialized successfully.");

    Status::ok()
}
