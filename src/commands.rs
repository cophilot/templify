use std::fs::read_dir;

use crate::{env, utils, version_control};

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

pub fn generate(args: Vec<String>) {
    if args.len() < 3 {
        println!("Missing argument!");
        println!("Usage: tpy generate <template-name> <given-name>");
        return;
    }

    if !utils::check_if_templify_initialized() {
        return;
    }

    let template_name = &args[2];
    let given_name = &args[3];

    println!("Generating new file from template {}...", template_name);

    let paths = std::fs::read_dir(".templates").unwrap();
    let mut found = false;
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap() == template_name {
            found = true;
            break;
        }
    }
    if !found {
        println!("Template {} not found.", template_name);
        return;
    }

    let new_path = utils::parse_templify_file(&format!(".templates/{}/.templify", template_name))
        ["path"]
        .clone()
        .replace("$$name$$", given_name);

    // create dir and all subdirs if they don't exist
    std::fs::create_dir_all(&new_path).unwrap();
    //let files = std::fs::read_dir(&format!(".templates/{}/.templify", template_name)).unwrap();
    utils::generate_template_dir(
        &format!(".templates/{}", template_name),
        &new_path,
        given_name,
    );

    println!("Files generated successfully.");
}

pub fn new(args: Vec<String>) {
    // return if template name is not provided
    if args.len() < 3 {
        println!("Missing argument: template-name");
        println!("Usage: tpy new <template-name>");
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

    if !version_control::is_newer_version_available() {
        println!("Templify is already up to date.");
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
        println!("Templify is already initialized in this project.");
        return;
    }
    std::fs::create_dir(".templates").unwrap();
    std::fs::write(
        ".templates/README.md",
        crate::data::get_init_readme_content(),
    )
    .unwrap();
    std::fs::create_dir(".templates/Example").unwrap();
    std::fs::write(
        ".templates/Example/.templify",
        crate::data::get_init_example_templify_content(),
    )
    .unwrap();
    std::fs::write(
        ".templates/Example/index.html",
        crate::data::get_init_example_index_content(),
    )
    .unwrap();
}

pub fn help() {
    println!("templify help center");
    println!("");
    println!("Usage: tpy <command>");
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
    println!("  [ list | ls ]                                   List all templates");
    println!("  [ generate | g ] <template-name> <given-name>   Generate a new file from the given template");
}
