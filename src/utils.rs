use chrono::{self, Datelike};
use std::{io::Write, path::Path};

use crate::types::Status;

pub fn parse_template_name(name: &mut String, strict: bool) -> Status {
    let template_name_raw = name.clone().to_string();
    let parsed_template_name = name.clone().to_lowercase().to_string();

    let mut found = false;
    let paths = std::fs::read_dir(".templates").unwrap();

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
            // assign path_name to name so that it can be used from the caller
            *name = String::from(path_name.clone());
            found = true;
        } else if path.is_dir() && path_name == name.clone() && strict {
            found = true;
            *name = String::from(path_name.clone());
            break;
        }
    }

    if !found {
        return Status::error(format!("Template {} not found.", name));
    }
    return Status::ok();
}

pub fn load_remote_template_collection(path: &str, url: &str, force: bool) -> Status {
    let response = reqwest::blocking::get(url);
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response: serde_json::Value = response.unwrap();

    let items = response["payload"]["tree"]["items"].as_array().unwrap();

    for item in items {
        if item["contentType"] == "directory" {
            let st = load_remote_template(
                format!("{}/{}", path, item["name"])
                    .replace("\"", "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace("\"", "")
                    .as_str(),
                force,
            );
            if !st.is_ok {
                return st;
            }
        }
    }
    return Status::ok();
}

pub fn load_remote_template(path: &str, url: &str, force: bool) -> Status {
    if !force && Path::new(path).exists() {
        return Status::error(format!(
            "Template {} already exists...",
            path.replace(".templates/", "")
        ));
    }

    if !Path::new(path).exists() {
        std::fs::create_dir(path).unwrap();
    }

    let response = reqwest::blocking::get(url);
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response: serde_json::Value = response.unwrap();

    let items = response["payload"]["tree"]["items"].as_array().unwrap();

    for item in items {
        if item["contentType"] == "directory" {
            let st = load_remote_template_dir(
                format!("{}/{}", path, item["name"])
                    .replace("\"", "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace("\"", "")
                    .as_str(),
                force,
            );
            if !st.is_ok {
                return st;
            }
            continue;
        }

        let st = load_remote_template_file(
            format!("{}/{}", path, item["name"])
                .replace("\"", "")
                .as_str(),
            format!("{}/{}", url, item["name"])
                .replace("\"", "")
                .as_str(),
            force,
        );
        if !st.is_ok {
            return st;
        }
    }

    let temp_file = format!("{}/.templify", path);

    if !Path::new(temp_file.as_str()).exists() {
        // create .templify file
        std::fs::File::create(temp_file).unwrap();
    }

    // write to .templify file
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/.templify", path).as_str())
        .unwrap();

    // check if url already exists in .templify file
    let file_content = std::fs::read_to_string(format!("{}/.templify", path).as_str());
    if file_content.is_err() {
        return Status::ok();
    }
    let file_content = file_content.unwrap();
    if !file_content.contains(".source") {
        file.write_all(format!("\n\n.source:{}", url).as_bytes())
            .unwrap();
    }

    println!("Loaded template: {}", path.replace(".templates/", ""));
    return Status::ok();
}

fn load_remote_template_dir(path: &str, url: &str, force: bool) -> Status {
    if !force && Path::new(path).exists() {
        return Status::error(format!(
            "Directory {} already exists...",
            path.replace(".templates/", "")
        ));
    }

    if !Path::new(path).exists() {
        std::fs::create_dir(path).unwrap();
    }

    let response = reqwest::blocking::get(url);
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response: serde_json::Value = response.unwrap();
    let items = response["payload"]["tree"]["items"].as_array().unwrap();

    for item in items {
        if item["contentType"] == "directory" {
            let st = load_remote_template_dir(
                format!("{}/{}", path, item["name"])
                    .replace("\"", "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace("\"", "")
                    .as_str(),
                force,
            );
            if !st.is_ok {
                return st;
            }
            continue;
        }

        load_remote_template_file(
            format!("{}/{}", path, item["name"])
                .replace("\"", "")
                .as_str(),
            format!("{}/{}", url, item["name"])
                .replace("\"", "")
                .as_str(),
            force,
        );
    }
    return Status::ok();
}

fn load_remote_template_file(path: &str, url: &str, force: bool) -> Status {
    if Path::new(path).exists() && !force {
        return Status::error(format!(
            "File {} already exists...",
            path.replace(".templates/", "")
        ));
    }

    let response = reqwest::blocking::get(url);
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!("Failed to get template from {}", url));
    }
    let response: serde_json::Value = response.unwrap();

    let text = response["payload"]["blob"]["rawLines"].as_array().unwrap();
    let mut text = text
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect::<Vec<&str>>()
        .join("\n");

    text = text.replace("\\n", "\n");

    // create all subdirs if they don't exist
    let path_dir = path.split("/").collect::<Vec<&str>>();
    let path_dir = path_dir[..path_dir.len() - 1].join("/");
    std::fs::create_dir_all(path_dir.clone()).unwrap();

    let mut new_file = std::fs::File::create(path).unwrap();
    new_file.write_all(text.as_bytes()).unwrap();

    println!("Created file {}", path);
    return Status::ok();
}

pub fn generate_template_dir(path: &str, new_path: &str, given_name: &str, dry_run: bool) -> bool {
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name == ".templify" || file_name == ".tpykeep" || file_name == ".templifykeep" {
            continue;
        }

        let mut new_file_name = file_name.replace("$$name$$", given_name);
        new_file_name =
            new_file_name.replace("$$year$$", chrono::Local::now().year().to_string().as_str());
        new_file_name =
            new_file_name.replace("$$month$$", &chrono::Local::now().month().to_string());
        new_file_name = new_file_name.replace("$$day$$", &chrono::Local::now().day().to_string());
        new_file_name = new_file_name.replace("$$git-name$$", &crate::utils::get_git_name());
        let new_path = format!("{}/{}", new_path, new_file_name);

        // check if new_path already exists
        if Path::new(&new_path).exists() {
            println!("File {} already exists.", new_path);
            return false;
        }

        if path.is_dir() {
            std::fs::create_dir(&new_path).unwrap();
            if !generate_template_dir(&path.to_str().unwrap(), &new_path, given_name, dry_run) {
                return false;
            }
        } else {
            if !generate_template_file(&path.to_str().unwrap(), &new_path, given_name, dry_run) {
                return false;
            }
        }
    }
    return true;
}

pub fn generate_template_file(path: &str, new_path: &str, given_name: &str, dry_run: bool) -> bool {
    let file_content = std::fs::read_to_string(path).unwrap();
    let mut file_content = file_content.replace("$$name$$", given_name);
    file_content =
        file_content.replace("$$year$$", chrono::Local::now().year().to_string().as_str());
    file_content = file_content.replace("$$month$$", &chrono::Local::now().month().to_string());
    file_content = file_content.replace("$$day$$", &chrono::Local::now().day().to_string());
    file_content = file_content.replace("$$git-name$$", &crate::utils::get_git_name());

    if Path::new(new_path).exists() {
        println!("File {} already exists.", new_path);
        return false;
    }

    if dry_run {
        println!("Would create file {}", new_path);
        return true;
    }

    let mut new_file = std::fs::File::create(new_path).unwrap();
    new_file.write_all(file_content.as_bytes()).unwrap();

    println!("Created file {}", new_path);
    return true;
}

pub fn check_if_templify_initialized() -> crate::types::Status {
    if !Path::new(".templates").exists() {
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };
        return Status::error(format!("templify is not initialized in this project.\nRun `{} init` to initialize templify in your project.",command_name));
    }
    return Status::ok();
}

pub fn check_internet_connection() -> bool {
    let response = reqwest::blocking::get("https://google.com");
    if response.is_err() {
        return false;
    }
    let response = response.unwrap();
    if response.status().is_success() {
        return true;
    }
    return false;
}

pub fn get_git_name() -> String {
    let output = std::process::Command::new("git")
        .arg("config")
        .arg("user.name")
        .output();
    if output.is_err() {
        return "unknown".to_string();
    }
    let output = output.unwrap();
    let output = String::from_utf8_lossy(&output.stdout);
    let mut name = output.trim().to_string();
    if name.is_empty() {
        name = "unknown".to_string();
    }
    return name;
}

pub fn handle_dev_mode(args: &Vec<String>) {
    if args.contains(&"-dev".to_string()) {
        if !Path::new("dev").exists() {
            std::fs::create_dir("dev").unwrap();
        }
        std::env::set_current_dir("dev").unwrap();
    }
}
