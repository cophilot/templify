use crate::log;
use crate::types::argument::Argument;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::utils;
use std::fs::read_dir;

/// The definition of the reload command.
pub(crate) fn definition() -> Command {
    let mut reload_command = Command::new(
        vec!["reload".to_string(), "rl".to_string()],
        reload,
        "Reload templates from a github repository.".to_string(),
    );

    reload_command.add_argument(Argument::new(
        "template-name".to_string(),
        0,
        false,
        "The name of the template to reload (reload all if not provided).".to_string(),
    ));

    reload_command.add_flag(Flag::new_bool_flag(
        vec!["strict".to_string()],
        "If enabled the template name must match exactly.".to_string(),
    ));

    reload_command.add_flag(Flag::new_bool_flag(
        vec!["reset".to_string(), "r".to_string()],
        "If enabled the template will be deleted and reloaded.".to_string(),
    ));

    reload_command
}

/// The reload command is used to reload templates from a remote repository.
pub(crate) fn reload(command: &Command) -> Status {
    if !utils::functions::check_internet_connection() {
        log!("You need a internet connection for this command!");
        return Status::error("You need a internet connection for this command!".to_string());
    }

    let st = utils::functions::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }
    let strict = command.get_bool_flag("strict");
    let reset = command.get_bool_flag("reset");
    let name = command.get_argument("template-name").value.clone();
    if !name.is_empty() {
        return utils::template_handler::reload_template(name, strict, reset);
    }

    let paths: std::fs::ReadDir = read_dir(".templates").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if !path.is_dir() {
            continue;
        }
        let template_name = path.file_name().unwrap().to_str().unwrap();

        let st = utils::template_handler::reload_template(template_name.to_string(), false, reset);

        if !st.is_ok {
            log!(
                "Error: Template {} could not be reloaded ({})",
                template_name,
                st.message
            );
        } else {
            log!("Template {} reloaded successfully.", template_name);
        }
        log!(" ");
    }

    Status::ok()
}
