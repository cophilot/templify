use crate::{
    types::{self, Command, Status},
    utils, version_control,
};
use chrono::{self, Datelike};
use std::fs::read_dir;

pub fn list(command: &Command) -> Status {
    let st = utils::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    // get all folders in .templates
    let paths = read_dir(".templates").unwrap();

    let print_path = command.get_bool_flag("path");
    let only_name = command.get_bool_flag("name");

    println!("Available templates:");
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let template_name = path.file_name().unwrap().to_str().unwrap();

            let meta = types::TemplateMeta::parse(template_name.to_string());

            let mut print_string = template_name.to_string();

            if !meta.get_description().is_empty() && !only_name {
                print_string = format!("{} - {}", print_string, meta.get_description());
            }
            if print_path {
                print_string = format!("{} [{}]", print_string, meta.get_path());
            }
            println!("  {}", print_string);
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
            "Invalid url: {}\nOnly templates from GitHub are supported at the moment.",
            url
        ));
    }

    let load_template = command.get_bool_flag("template");
    if load_template {
        println!("Loading template from {}...", url);
        let name = url.split("/").last().unwrap();
        let st = utils::load_remote_template(
            format!(".templates/{}", name).as_str(),
            url.as_str(),
            command.get_bool_flag("force"),
        );
        if !st.is_ok {
            return st;
        }
    } else {
        println!("Loading template collection from {}...", url);
        let st = utils::load_remote_template_collection(
            ".templates",
            url.as_str(),
            command.get_bool_flag("force"),
        );
        if !st.is_ok {
            return st;
        }
    }
    return Status::ok();
}

pub fn reload(command: &Command) -> Status {
    if !utils::check_internet_connection() {
        println!("You need a internet connection for this command!");
        return Status::error("You need a internet connection for this command!".to_string());
    }

    let st = utils::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }
    let strict = command.get_bool_flag("strict");
    let mut name = command.get_argument("template-name").value.clone();
    if name != "" {
        let st = utils::parse_template_name(&mut name, strict);
        if !st.is_ok {
            return st;
        }
        let meta = types::TemplateMeta::parse(name.clone().to_string());
        if meta.get_source().is_empty() {
            return Status::error(format!("Template {} has no source.", name));
        }
        println!(
            "Reloading template {} from {}...",
            meta.get_template_name(),
            meta.get_source()
        );
        let st = utils::load_remote_template(
            format!(".templates/{}", name).as_str(),
            meta.get_source().as_str(),
            true,
        );
        if !st.is_ok {
            return st;
        }
        println!("Template {} reloaded successfully.", name);
        return Status::ok();
    }

    let paths: std::fs::ReadDir = read_dir(".templates").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if !path.is_dir() {
            continue;
        }
        let template_name = path.file_name().unwrap().to_str().unwrap();
        let meta = types::TemplateMeta::parse(template_name.to_string());
        if meta.get_source().is_empty() {
            continue;
        }
        println!(
            "Reloading template {} from {}...",
            meta.get_template_name(),
            meta.get_source()
        );
        let st = utils::load_remote_template(
            format!(".templates/{}", template_name).as_str(),
            meta.get_source().as_str(),
            true,
        );
        if !st.is_ok {
            println!("Error: Template {} could not be reloaded!", template_name);
            println!("");
            continue;
        }
        println!("Template {} reloaded successfully.", template_name);
        println!("");
    }

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
    let given_name = command.get_argument("new-name").value.clone();

    let st = utils::parse_template_name(&mut template_name, strict);
    if !st.is_ok {
        return st;
    }

    println!(
        "Generating new files from template {}...",
        template_name.clone()
    );
    let meta = types::TemplateMeta::parse(template_name.clone().to_string());
    let mut new_path = meta.get_path();

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
        utils::load_remote_template_collection(
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
