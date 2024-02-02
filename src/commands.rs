use std::fs::read_dir;

use crate::{utils, version_control};

pub fn list() {
    if !utils::check_if_templify_initialized() {
        return;
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
}

pub fn load(args: Vec<String>) {
    if !utils::check_internet_connection() {
        println!("You need a internet connection for this command!");
        return;
    }
    if args.len() < 3 {
        println!("Missing argument!");
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };
        println!("Usage: {} load <url>", command_name);
        return;
    }
    if !utils::check_if_templify_initialized() {
        return;
    }

    let url = &args[2];
    if !url.starts_with("https://github.com") {
        println!("Could not load template: {}", url);
        println!("Only github templates are supported at the moment.");
        return;
    }
    println!("Loading template from {}...", url);
    utils::load_remote_template_dir(".templates", url, true)
}

pub fn generate(args: Vec<String>) {
    if args.len() < 4 {
        println!("Missing argument!");
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

        println!(
            "Usage: {} generate <template-name> <given-name>",
            command_name
        );
        return;
    }

    if !utils::check_if_templify_initialized() {
        return;
    }

    let template_name = &args[2].to_string();
    let given_name = &args[3];

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
        println!("Template {} not found.", template_name);
        return;
    }

    println!("Generating new files from template {}...", template_name);

    let new_path = utils::parse_templify_file(&format!(".templates/{}/.templify", template_name))
        ["path"]
        .clone()
        .replace("$$name$$", given_name);

    // create dir and all subdirs if they don't exist
    std::fs::create_dir_all(&new_path).unwrap();

    if utils::generate_template_dir(
        &format!(".templates/{}", template_name),
        &new_path,
        given_name,
    ) {
        println!("Files generated successfully.");
    } else {
        println!("Files could not be generated.");
    }
}

pub fn new(args: Vec<String>) {
    // return if template name is not provided
    if args.len() < 3 {
        println!("Missing argument: template-name");
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };
        println!("Usage: {} new <template-name>", command_name);
        return;
    }

    if !utils::check_if_templify_initialized() {
        return;
    }

    let template_name = &args[2];

    println!("Creating new template: {}", template_name);

    let template_path = format!(".templates/{}", template_name);
    if std::path::Path::new(&template_path).exists() {
        println!("Template already exists.");
        return;
    }

    std::fs::create_dir(&template_path).unwrap();

    std::fs::write(
        format!("{}/.templify", template_path),
        crate::data::templify_file_blank(),
    )
    .unwrap();

    println!("Template {} created successfully.", template_name);
}

pub fn update() {
    /* if !env::is_linux() {
        println!("Updating templify is currently only supported on Linux.");
        println!("Please visit https://github.com/cophilot/templify to download the latest version and update manually.");
        return;
    } */
    if !utils::check_internet_connection() {
        println!("You need a internet connection for this command!");
        return;
    }

    if !version_control::is_newer_version_available() {
        println!("templify is already up to date.");
        return;
    }

    println!("Updating templify...");

    version_control::update().unwrap();

    println!("templify updated successfully.");
    std::process::exit(0);
}

pub fn version() {
    println!("templify version {}", env!("CARGO_PKG_VERSION"));
}

pub fn init() {
    println!("Initializing templify...");
    // check if .templates folder exists
    if std::path::Path::new(".templates").exists() {
        println!("templify is already initialized in this project.");
        return;
    }
    std::fs::create_dir(".templates").unwrap();
    std::fs::write(
        ".templates/README.md",
        crate::data::get_init_readme_content(),
    )
    .unwrap();

    // check if there is an internet connection
    if utils::check_internet_connection() {
        load(vec![
            "tpy".to_string(),
            "load".to_string(),
            "https://github.com/cophilot/templify-vault/tree/main/Example".to_string(),
        ]);
    }
}

pub fn help() {
    let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    println!("templify help center");
    println!("");
    println!("Usage: {} <command>", command_name);
    println!("");
    println!("Commands:");
    println!("  [ help | -h ]                                   Show this help message");
    println!(
        "  [ version | -v ]                                Print the current version of templify",
    );
    println!(
        "  [ update ]                                      Update templify to the latest version",
    );
    println!(
        "  [ init | i ]                                    Initialize Templify in your project",
    );
    println!("  [ new | n ] <template-name>                     Create a new template with the given name");
    println!("  [ load | l ] <url>                              Load templates from a github repository (provide a url that points to an folder in a github repository)");
    println!("  [ list | ls ]                                   List all templates");
    println!("  [ generate | g ] <template-name> <given-name>   Generate a new file from the given template");
}
