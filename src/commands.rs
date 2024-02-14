use crate::{
    types::{Command, Status},
    utils, version_control,
};
use chrono::{self, Datelike};
use std::fs::read_dir;

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
        return Status::error(format!(
            "Invalid url: {}\nOnly github templates are supported at the moment.",
            url
        ));
    }
    println!("Loading template from {}...", url);
    utils::load_remote_template_repo(".templates", url.as_str(), command.get_bool_flag("force"));
    return Status::ok();
}

pub fn generate(command: &Command) -> Status {
    let st = utils::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    let strict = command.get_bool_flag("strict");
    let dry_run = command.get_bool_flag("dry-run");

    let mut template_name = command.get_argument("template-name").value.clone();
    let parsed_template_name = template_name.clone().to_lowercase().to_string();
    let template_name_raw = template_name.clone().to_string();

    let given_name = command.get_argument("new-name").value.clone();

    let paths = std::fs::read_dir(".templates").unwrap();
    let mut found = false;
    for path in paths {
        let path = path.unwrap().path();

        let path_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .clone();

        let parsed_path_name = path_name.clone().to_lowercase().to_string();

        if path.is_dir() && parsed_path_name.starts_with(parsed_template_name.as_str()) && !strict {
            if found {
                return Status::error(format!(
                    "Template {} is not unique. Please use a more specific name.",
                    template_name_raw
                ));
            }
            template_name = path_name.clone();
            found = true;
        } else if path.is_dir() && path_name == template_name && strict {
            template_name = path_name.clone();
            found = true;
            break;
        }
    }

    if !found {
        return Status::error(format!("Template {} not found.", template_name));
    }

    println!("Generating new files from template {}...", template_name);

    let mut new_path =
        utils::parse_templify_file(&format!(".templates/{}/.templify", template_name))["path"]
            .clone();

    new_path = new_path.replace("$$name$$", given_name.as_str());
    new_path = new_path.replace("$$year$$", chrono::Local::now().year().to_string().as_str());
    new_path = new_path.replace("$$month$$", &chrono::Local::now().month().to_string());
    new_path = new_path.replace("$$day$$", &chrono::Local::now().day().to_string());
    new_path = new_path.replace("$$git-name$$", &utils::get_git_name());

    // create dir and all subdirs if they don't exist
    if !dry_run {
        std::fs::create_dir_all(&new_path).unwrap();
    }

    if utils::generate_template_dir(
        &format!(".templates/{}", template_name),
        &new_path,
        given_name.as_str(),
        dry_run,
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

pub fn update(command: &Command) -> Status {
    if !utils::check_internet_connection() {
        return Status::error("You need a internet connection for this command!".to_string());
    }

    let version = command.get_value_flag("version").clone();

    if !version_control::is_newer_version_available() && version == "" {
        println!("templify is already up to date.");
        return Status::ok();
    }

    if version != "" {
        println!("Updating templify to version {}...", version);
    } else {
        println!("Updating templify...");
    }

    let st = version_control::update(version);
    if st.is_err() {
        return Status::error(format!("{}", st.err().unwrap()));
    }

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
        return Status::error("templify is already initialized in this project.".to_string());
    }

    std::fs::create_dir(".templates").unwrap();

    if command.get_bool_flag("blank") {
        println!("templify initialized successfully.");
        return Status::ok();
    }
    std::fs::write(
        ".templates/README.md",
        crate::data::get_init_readme_content(),
    )
    .unwrap();

    // check if there is an internet connection
    if utils::check_internet_connection() && !command.get_bool_flag("offline") {
        println!("Loading example template from templify-vault...");
        utils::load_remote_template_repo(
            ".templates",
            "https://github.com/cophilot/templify-vault/tree/main/Example",
            true,
        );
    }
    println!("templify initialized successfully.");
    return Status::ok();
}

pub fn help(command: &Command) -> Status {
    let base_command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    if command.get_argument("command").is_set {
        let mut command_name = command.get_argument("command").value.clone();
        let all_commands = crate::command_storage::get_all_commands();
        for c in all_commands {
            if c.names.contains(&command_name) {
                command_name = c.names[0].clone();
                println!("templify help center");
                println!("");
                println!("<...> - required");
                println!("[...] - optional");
                println!("");
                println!("Usage: {} {}", base_command_name, command_name);
                println!("");
                println!("{}", c.to_help_string());
                println!(
                    "To get more information please visit: https://templify.philipp-bonin.com/#/command/{}", command_name
                );

                return Status::ok();
            }
        }
        return Status::error(format!("Command {} not found.", command_name));
    }

    println!("templify help center");
    println!("");
    println!("<...> - required");
    println!("[...] - optional");
    println!("");
    println!("Usage: {} <command>", base_command_name);
    println!("");
    println!("Commands:");

    let all_commands = crate::command_storage::get_all_commands();
    for command in all_commands {
        println!("{}", command.to_help_string());
    }

    println!("To get more information please visit: https://templify.philipp-bonin.com");

    return Status::ok();
}
