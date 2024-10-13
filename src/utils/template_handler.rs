use crate::commands::load::URLType;
use crate::log;
use crate::types::status::Status;
use crate::types::template_meta::TemplateMeta;
use crate::utils::formater;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::io::Write;
use std::path::Path;

use super::rest;

/// Parse the template name and check if it exists (template_name is modified in place)
pub(crate) fn parse_template_name(name: &mut String, strict: bool) -> Status {
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

        if path.is_dir() && parsed_path_name == parsed_template_name {
            *name = path_name.clone();
            found = true;
            break;
        }

        if path.is_dir() && parsed_path_name.starts_with(parsed_template_name.as_str()) && !strict {
            if found {
                return Status::error(format!(
                    "Template {} is not unique. Please use a more specific name.",
                    template_name_raw
                ));
            }
            // assign path_name to name so that it can be used from the caller
            *name = path_name.clone();
            found = true;
        }
    }

    if !found {
        return Status::error(format!("Template {} not found.", name));
    }
    Status::ok()
}

/// Reload a template from its source
pub(crate) fn reload_template(name: String, strict: bool, reset: bool) -> Status {
    let mut name = name;
    let st = parse_template_name(&mut name, strict);
    if !st.is_ok {
        return st;
    }
    let meta = TemplateMeta::parse(name.clone().to_string());
    if meta.get_source().is_empty() {
        return Status::error(format!("Template {} has no source", name));
    }
    log!(
        "Reloading template {} from {}...",
        meta.get_template_name(),
        meta.get_source()
    );

    let dir = format!(".templates/{}", name);
    let backup_dir = format!(".templates/{}---backup", name);

    if reset && std::path::Path::new(&dir).exists() {
        if std::path::Path::new(&backup_dir).exists() {
            std::fs::remove_dir_all(&backup_dir).unwrap();
        }
        std::fs::rename(&dir, &backup_dir).unwrap();
    }

    let url = meta.get_source();

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

    let st = load_remote_template(
        format!(".templates/{}", name).as_str(),
        url.as_str(),
        true,
        &url_type,
    );
    if !st.is_ok {
        if reset {
            std::fs::rename(&backup_dir, &dir).unwrap();
        }
        return st;
    }

    if reset {
        std::fs::remove_dir_all(&backup_dir).unwrap();
    }

    log!("Template {} reloaded successfully.", name);
    Status::ok()
}

/// Load a collection of templates from a remote repository
pub(crate) fn load_remote_template_collection(
    path: &str,
    url: &str,
    force: bool,
    url_type: &URLType,
) -> Status {
    let response = rest::json_call(url);
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: Request failed",
            url
        ));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: JSON parse error",
            url
        ));
    }
    let response: serde_json::Value = response.unwrap();

    let items = match url_type {
        URLType::GitHub => response["payload"]["tree"]["items"].as_array().unwrap(),
        URLType::GitLab => response.as_array().unwrap(),
    };

    for item in items {
        let check_collection = match url_type {
            URLType::GitHub => item["contentType"] == "directory",
            URLType::GitLab => item["type"] == "tree",
        };

        if check_collection {
            let st = load_remote_template(
                format!("{}/{}", path, item["name"])
                    .replace('"', "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace('"', "")
                    .as_str(),
                force,
                &url_type,
            );
            if !st.is_ok {
                return st;
            }
        }
    }

    Status::ok()
}

/// Load a template from a gitlab repository
pub(crate) fn load_gitlab_template(
    response: serde_json::Value,
    path: &str,
    url: &str,
    force: bool,
) -> Status {
    let items = response.as_array().unwrap();

    for item in items {
        if item["type"] == "tree" {
            let st = load_remote_gitlab_template_dir(
                format!("{}/{}", path, item["name"])
                    .replace('"', "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace('"', "")
                    .as_str(),
                force,
            );
            if !st.is_ok {
                return st;
            }
            continue;
        }

        let base_url = url.split("/tree").next().unwrap_or("");

        if base_url.is_empty() {
            return Status::error(format!("Invalid url: {}\n", url));
        }

        let st = load_remote_gitlab_template_file(
            format!("{}/{}", path, item["name"])
                .replace('"', "")
                .as_str(),
            format!("{}/blobs/{}", base_url, item["id"])
                .replace('"', "")
                .as_str(),
            force,
        );
        if !st.is_ok {
            return st;
        }
    }

    return Status::ok();
}

/// Load a template from a github repository
pub(crate) fn load_github_template(
    response: serde_json::Value,
    path: &str,
    url: &str,
    force: bool,
) -> Status {
    let items = response["payload"]["tree"]["items"].as_array().unwrap();

    for item in items {
        if item["contentType"] == "directory" {
            let st = load_remote_template_dir(
                format!("{}/{}", path, item["name"])
                    .replace('"', "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace('"', "")
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
                .replace('"', "")
                .as_str(),
            format!("{}/{}", url, item["name"])
                .replace('"', "")
                .as_str(),
            force,
        );
        if !st.is_ok {
            return st;
        }
    }

    return Status::ok();
}

/// Load a template from a remote repository
pub(crate) fn load_remote_template(
    path: &str,
    url: &str,
    force: bool,
    url_type: &URLType,
) -> Status {
    if !force && Path::new(path).exists() {
        return Status::error(format!(
            "Template {} already exists...",
            path.replace(".templates/", "")
        ));
    }

    if !Path::new(path).exists() {
        std::fs::create_dir(path).unwrap();
    }

    let response = rest::json_call(url);
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: Request failed",
            url
        ));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: JSON parse error",
            url
        ));
    }
    let response: serde_json::Value = response.unwrap();

    let status = match url_type {
        URLType::GitHub => load_github_template(response, path, url, force),
        URLType::GitLab => load_gitlab_template(response, path, url, force),
    };

    if !status.is_ok {
        return status;
    }

    let temp_file = format!("{}/.templify", path);

    if !Path::new(temp_file.as_str()).exists() {
        // create .templify file
        std::fs::File::create(temp_file).unwrap();
    }

    // write to .templify file
    let mut file = std::fs::OpenOptions::new()
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

    log!("Loaded template: {}", path.replace(".templates/", ""));
    Status::ok()
}

/// Load a directory from a remote repository
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

    let response = rest::json_call(url);
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: : Request failed",
            url
        ));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: JSON parse error",
            url
        ));
    }
    let response: serde_json::Value = response.unwrap();
    let items = response["payload"]["tree"]["items"].as_array().unwrap();

    for item in items {
        if item["contentType"] == "directory" {
            let st = load_remote_template_dir(
                format!("{}/{}", path, item["name"])
                    .replace('"', "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace('"', "")
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
                .replace('"', "")
                .as_str(),
            format!("{}/{}", url, item["name"])
                .replace('"', "")
                .as_str(),
            force,
        );
    }
    Status::ok()
}

fn load_remote_gitlab_template_dir(path: &str, url: &str, force: bool) -> Status {
    if !force && Path::new(path).exists() {
        return Status::error(format!(
            "Directory {} already exists...",
            path.replace(".templates/", "")
        ));
    }

    if !Path::new(path).exists() {
        std::fs::create_dir(path).unwrap();
    }

    let response = rest::json_call(url);
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: : Request failed",
            url
        ));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: JSON parse error",
            url
        ));
    }
    let response: serde_json::Value = response.unwrap();
    let items = response.as_array().unwrap();

    for item in items {
        if item["type"] == "tree" {
            let st = load_remote_gitlab_template_dir(
                format!("{}/{}", path, item["name"])
                    .replace('"', "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace('"', "")
                    .as_str(),
                force,
            );
            if !st.is_ok {
                return st;
            }
            continue;
        }

        let base_url = url.split("/tree").next().unwrap_or("");

        if base_url.is_empty() {
            return Status::error(format!("Invalid url: {}\n", url));
        }

        load_remote_gitlab_template_file(
            format!("{}/{}", path, item["name"])
                .replace('"', "")
                .as_str(),
            format!("{}/blobs/{}", base_url, item["id"])
                .replace('"', "")
                .as_str(),
            force,
        );
    }
    Status::ok()
}

/// Load a file from a remote repository
fn load_remote_template_file(path: &str, url: &str, force: bool) -> Status {
    if Path::new(path).exists() && !force {
        return Status::error(format!(
            "File {} already exists...",
            path.replace(".templates/", "")
        ));
    }

    let response = rest::json_call(url);
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: Request failed",
            url
        ));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: JSON parse error",
            url
        ));
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
    let path_dir = path.split('/').collect::<Vec<&str>>();
    let path_dir = path_dir[..path_dir.len() - 1].join("/");
    std::fs::create_dir_all(path_dir.clone()).unwrap();

    let mut new_file = std::fs::File::create(path).unwrap();
    new_file.write_all(text.as_bytes()).unwrap();

    log!("Created file {}", path);
    Status::ok()
}

/// Load a file from gitlab remote repository
fn load_remote_gitlab_template_file(path: &str, url: &str, force: bool) -> Status {
    if Path::new(path).exists() && !force {
        return Status::error(format!(
            "File {} already exists...",
            path.replace(".templates/", "")
        ));
    }

    let response = rest::json_call(url);
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: Request failed",
            url
        ));
    }
    let response = response.unwrap().json();
    if response.is_err() {
        return Status::error(format!(
            "Failed to get template from {}: JSON parse error",
            url
        ));
    }
    let response: serde_json::Value = response.unwrap();

    let content = response["content"].as_str();
    let encoding = response["encoding"].as_str();

    if encoding.unwrap_or("") != "base64" || content.is_none() {
        return Status::error(format!(
            "Failed to get template from {}: Decoding Error",
            url
        ));
    }

    let mut text = match STANDARD.decode(content.unwrap()) {
        Ok(decoded) => match String::from_utf8(decoded) {
            Ok(message) => message,
            Err(_e) => {
                return Status::error(format!(
                    "Failed to get template from {}: Decoding Error",
                    url
                ))
            }
        },
        Err(_e) => {
            return Status::error(format!(
                "Failed to get template from {}: Decoding Error",
                url
            ))
        }
    };

    text = text.replace("\\n", "\n");

    // create all subdirs if they don't exist
    let path_dir = path.split('/').collect::<Vec<&str>>();
    let path_dir = path_dir[..path_dir.len() - 1].join("/");
    std::fs::create_dir_all(path_dir.clone()).unwrap();

    let mut new_file = std::fs::File::create(path).unwrap();
    new_file.write_all(text.as_bytes()).unwrap();

    log!("Created file {}", path);
    Status::ok()
}

/// Generate a template from a template
pub(crate) fn generate_template_dir(
    path: &str,
    new_path: &str,
    given_name: &str,
    dry_run: bool,
    meta: TemplateMeta,
    force: bool,
) -> bool {
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name == ".templify" || file_name == ".tpykeep" || file_name == ".templifykeep" {
            continue;
        }

        let meta = meta.clone();
        let new_file_name = formater::handle_placeholders(file_name, given_name, meta.clone());
        let new_path = format!("{}/{}", new_path, new_file_name);

        if path.is_dir() {
            if !dry_run {
                std::fs::create_dir_all(&new_path).unwrap();
            }
            if !generate_template_dir(
                path.to_str().unwrap(),
                &new_path,
                given_name,
                dry_run,
                meta,
                force,
            ) {
                return false;
            }
        } else if !generate_template_file(
            path.to_str().unwrap(),
            &new_path,
            given_name,
            dry_run,
            meta,
            force,
        ) {
            return false;
        }
    }
    true
}

/// Generate a file from a template
pub(crate) fn generate_template_file(
    path: &str,
    new_path: &str,
    given_name: &str,
    dry_run: bool,
    meta: TemplateMeta,
    force: bool,
) -> bool {
    let file_content = std::fs::read_to_string(path).unwrap();
    let file_content = formater::handle_placeholders(&file_content, given_name, meta);

    if Path::new(new_path).exists() {
        if force {
            if !dry_run {
                std::fs::remove_file(new_path).unwrap();
            }
        } else {
            log!("File {} already exists.", new_path);
            return false;
        }
    }

    if dry_run {
        log!("Would create file {}", new_path);
        return true;
    }

    let mut new_file = std::fs::File::create(new_path).unwrap();
    new_file.write_all(file_content.as_bytes()).unwrap();

    let abs_path = std::fs::canonicalize(new_path).unwrap();

    log!("Created file {}", abs_path.to_str().unwrap());
    true
}
