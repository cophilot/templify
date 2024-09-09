use crate::log;
use crate::types::argument::Argument;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::utils;

pub(crate) enum URLType {
    GitHub,
    GitLab,
}

/// The definition of the load command.
pub(crate) fn definition() -> Command {
    let mut load_command = Command::new(
        vec!["load".to_string(), "l".to_string()],
        load,
        "Load templates from a remote repository.".to_string(),
    );

    load_command.add_argument(Argument::new(
        "url".to_string(),
        0,
        true,
        "The url of the github repository.".to_string(),
    ));

    load_command.add_flag(Flag::new_bool_flag(
        vec!["force".to_string(), "f".to_string()],
        "Force the load, even if the folder already exists.".to_string(),
    ));

    load_command.add_flag(Flag::new_bool_flag(
        vec!["template".to_string(), "t".to_string()],
        "Load only one template.".to_string(),
    ));

    load_command
}

/// The load command is used to load templates from a remote repository.
pub(crate) fn load(command: &Command) -> Status {
    if !utils::functions::check_internet_connection() {
        log!("You need a internet connection for this command!");
        return Status::error("You need a internet connection for this command!".to_string());
    }

    let st = utils::functions::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    let url = command.get_argument("url").value.clone();
    // if !url.starts_with("https://github.com") && !url.starts_with("https://gitlab.com") {
    //     return Status::error(format!(
    //         "Invalid url: {}\nOnly templates from GitHub and Gitlab are supported at the moment.",
    //         url
    //     ));
    // }

    let url_type = if url.starts_with("https://github.com") {
        URLType::GitHub
    } else if url.starts_with("https://gitlab.com") {
        URLType::GitLab
    } else {
        return Status::error(format!(
            "Invalid url: {}\nOnly templates from GitHub and Gitlab are supported at the moment.",
            url
        ));
    };

    let load_template = command.get_bool_flag("template");
    if load_template {
        log!("Loading template from {}...", url);
        let name = url.split('/').last().unwrap();
        let st = utils::template_handler::load_remote_template(
            format!(".templates/{}", name).as_str(),
            url.as_str(),
            command.get_bool_flag("force"),
            &url_type,
        );
        if !st.is_ok {
            return st;
        }
    } else {
        log!("Loading template collection from {}...", url);
        let st = utils::template_handler::load_remote_template_collection(
            ".templates",
            url.as_str(),
            command.get_bool_flag("force"),
            &url_type,
        );
        if !st.is_ok {
            return st;
        }
    }
    Status::ok()
}
