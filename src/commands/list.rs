use crate::log;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::types::template_meta::TemplateMeta;
use std::fs::read_dir;

/// The definition of the list command.
pub(crate) fn definition() -> Command {
    let mut list_commmand = Command::new(
        vec!["list".to_string(), "ls".to_string()],
        list,
        "List all available templates in the current project.".to_string(),
    );

    list_commmand.add_flag(Flag::new_bool_flag(
        vec!["name".to_string(), "n".to_string()],
        "Show only the names of the templates.".to_string(),
    ));

    list_commmand.add_flag(Flag::new_bool_flag(
        vec!["path".to_string(), "p".to_string()],
        "Show the path of the templates.".to_string(),
    ));
    list_commmand
}

/// The list command is used to list all available templates in the current project.
pub(crate) fn list(command: &Command) -> Status {
    let st = crate::utils::functions::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    // get all folders in .templates
    let paths = read_dir(".templates").unwrap();

    let print_path = command.get_bool_flag("path");
    let only_name = command.get_bool_flag("name");

    log!("Available templates:");
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let template_name = path.file_name().unwrap().to_str().unwrap();

            let meta = TemplateMeta::parse(template_name.to_string());

            let mut print_string = template_name.to_string();

            if !meta.get_description().is_empty() && !only_name {
                print_string = format!("{} - {}", print_string, meta.get_description());
            }
            if print_path {
                print_string = format!("{} [{}]", print_string, meta.get_path());
            }
            log!("  {}", print_string);
        }
    }

    Status::ok()
}
