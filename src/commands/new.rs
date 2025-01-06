use crate::log;
use crate::types::argument::Argument;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::utils;

/// The definition of the new command.
pub(crate) fn definition() -> Command {
    let mut new_command = Command::new(
        vec!["new".to_string(), "n".to_string()],
        new,
        "Create a new template with the given name.".to_string(),
    );

    new_command.add_argument(Argument::new(
        "template-name".to_string(),
        0,
        true,
        "The name of the new template.".to_string(),
    ));

    new_command.add_flag(Flag::new_value_flag(
        vec!["description".to_string(), "d".to_string()],
        "".to_string(),
        "Provide a description for the new template.".to_string(),
    ));

    new_command.add_flag(Flag::new_value_flag(
        vec!["path".to_string(), "p".to_string()],
        ".".to_string(),
        "Provide a path for the new template.".to_string(),
    ));
    new_command
}

/// The new command is used to create a new template with the given name.
pub(crate) fn new(command: &Command) -> Status {
    let st = utils::functions::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    let template_name = command.get_argument("template-name").value.clone();

    log!("Creating new template: {}", template_name);

    let template_path = format!(".templates/{}", template_name);
    if std::path::Path::new(&template_path).exists() {
        return Status::error(format!("Template {} already exists.", template_name));
    }

    std::fs::create_dir(&template_path).unwrap();

    std::fs::write(
        format!("{}/.templify.yml", template_path),
        crate::data::templify_file_blank(
            template_name.clone(),
            command.get_value_flag("description").clone(),
            command.get_value_flag("path").clone(),
        ),
    )
    .unwrap();

    log!("Template '{}' created successfully.", template_name);

    Status::ok()
}
