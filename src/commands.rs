use std::fs::read_dir;

use crate::{
    types::{Command, Status},
    utils, version_control,
};

pub fn list(_command: &Command) -> Status {
    let st = utils::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    // get all folders in .templates
    let paths = read_dir(".templates").unwrap();

    println!("Available templates:");
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let template_name = path.file_name().unwrap().to_str().unwrap();
            let description =
                utils::parse_templify_file(&format!(".templates/{}/.templify", template_name))
                    ["description"]
                    .clone();
            if description.is_empty() {
                println!("  {}", template_name);
            } else {
                println!("  {} - {}", template_name, description);
            }
        }
    }
    return Status::ok();
}

pub fn load(command: &Command) -> Status {
    if !utils::check_internet_connection() {
        println!("You need a internet connection for this command!");
        return Status::error("You need a internet connection for this command!".to_string());
    }

    let st = utils::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    let url = command.get_argument("url").value.clone();
    if !url.starts_with("https://github.com") {
        println!("Could not load template: {}", url);
        println!("Only github templates are supported at the moment.");
        return Status::error(format!(
            "Invalid url: {}\nOnly github templates are supported at the moment.",
            url
        ));
    }
    println!("Loading template from {}...", url);
    utils::load_remote_template_dir(".templates", url.as_str(), true);
    return Status::ok();
}

pub fn generate(command: &Command) -> Status {
    let st = utils::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    let template_name = command.get_argument("template-name").value.clone();
    let given_name = command.get_argument("new-name").value.clone();

    let paths = std::fs::read_dir(".templates").unwrap();
    let mut found = false;
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap() == template_name.to_string()
        {
            found = true;

            break;
        }
    }

    if !found {
        return Status::error(format!("Template {} not found.", template_name));
    }

    println!("Generating new files from template {}...", template_name);

    let new_path = utils::parse_templify_file(&format!(".templates/{}/.templify", template_name))
        ["path"]
        .clone()
        .replace("$$name$$", given_name.as_str());

    // create dir and all subdirs if they don't exist
    std::fs::create_dir_all(&new_path).unwrap();

    if utils::generate_template_dir(
        &format!(".templates/{}", template_name),
        &new_path,
        given_name.as_str(),
    ) {
        println!("Files generated successfully.");
        return Status::ok();
    } else {
        return Status::error("Files could not be generated.".to_string());
    }
}

pub fn new(command: &Command) -> Status {
    let st = utils::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    let template_name = command.get_argument("template-name").value.clone();

    println!("Creating new template: {}", template_name);

    let template_path = format!(".templates/{}", template_name);
    if std::path::Path::new(&template_path).exists() {
        return Status::error(format!("Template {} already exists.", template_name));
    }

    std::fs::create_dir(&template_path).unwrap();

    std::fs::write(
        format!("{}/.templify", template_path),
        crate::data::templify_file_blank(
            command.get_value_flag("description").clone(),
            command.get_value_flag("path").clone(),
        ),
    )
    .unwrap();

    println!("Template {} created successfully.", template_name);
    return Status::ok();
}

pub fn update(_command: &Command) -> Status {
    if !utils::check_internet_connection() {
        return Status::error("You need a internet connection for this command!".to_string());
    }

    if !version_control::is_newer_version_available() {
        println!("templify is already up to date.");
        return Status::ok();
    }

    println!("Updating templify...");

    version_control::update().unwrap();

    println!("templify updated successfully.");
    std::process::exit(0);
}

pub fn version(_command: &Command) -> Status {
    println!("templify version {}", env!("CARGO_PKG_VERSION"));
    return Status::ok();
}

pub fn init(command: &Command) -> Status {
    println!("Initializing templify...");

    // check if .templates folder exists
    if std::path::Path::new(".templates").exists() {
        println!("templify is already initialized in this project.");
        return Status::ok();
    }

    std::fs::create_dir(".templates").unwrap();
    std::fs::write(
        ".templates/README.md",
        crate::data::get_init_readme_content(),
    )
    .unwrap();

    // check if there is an internet connection
    if utils::check_internet_connection() && !command.get_bool_flag("offline") {
        println!("Loading example template from templify-vault...");
        utils::load_remote_template_dir(
            ".templates",
            "https://github.com/cophilot/templify-vault/tree/main/Example",
            true,
        );
    }
    println!("templify initialized successfully.");
    return Status::ok();
}

pub fn help(_command: &Command) -> Status {
    let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    println!("templify help center");
    println!("");
    println!("<...> - required");
    println!("[...] - optional");
    println!("");
    println!("Usage: {} <command>", command_name);
    println!("");
    println!("Commands:");

    let all_commands = crate::command_storage::get_all_commands();
    for command in all_commands {
        println!("{}", command.to_help_string());
    }

    return Status::ok();
}
