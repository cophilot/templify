use std::{io::Write, path::Path};

pub fn parse_templify_file(file_path: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();

    map.insert("description".to_string(), "".to_string());
    map.insert("path".to_string(), ".".to_string());

    let file_content = std::fs::read_to_string(file_path).unwrap();

    for line in file_content.lines() {
        let line = line.trim();
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() < 2 {
            continue;
        }

        let key = parts[0].trim().to_string().to_lowercase();
        let value = parts[1].trim().to_string();

        map.insert(key, value);
    }

    return map;
}

pub fn load_remote_template_dir(path: &str, url: &str, first: bool) {
    if !first {
        if Path::new(path).exists() {
            println!("Directory {} already exists...", path);
            return;
        }

        std::fs::create_dir(path).unwrap();
    }

    let response = reqwest::blocking::get(url).unwrap();
    let response: serde_json::Value = response.json().unwrap();
    let items = response["payload"]["tree"]["items"].as_array().unwrap();

    for item in items {
        if item["contentType"] == "directory" {
            load_remote_template_dir(
                format!("{}/{}", path, item["name"])
                    .replace("\"", "")
                    .as_str(),
                format!("{}/{}", url, item["name"])
                    .replace("\"", "")
                    .as_str(),
                false,
            );
            continue;
        }

        if first {
            continue;
        }

        load_remote_template_file(
            format!("{}/{}", path, item["name"])
                .replace("\"", "")
                .as_str(),
            format!("{}/{}", url, item["name"])
                .replace("\"", "")
                .as_str(),
        );
    }
}

pub fn load_remote_template_file(path: &str, url: &str) {
    if Path::new(path).exists() {
        println!("File {} already exists.", path);
        return;
    }

    let response = reqwest::blocking::get(url).unwrap();
    let response: serde_json::Value = response.json().unwrap();

    let text = response["payload"]["blob"]["rawLines"].as_array().unwrap();
    let mut text = text
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect::<Vec<&str>>()
        .join("\n");

    text = text.replace("\\n", "\n");

    let mut new_file = std::fs::File::create(path).unwrap();
    new_file.write_all(text.as_bytes()).unwrap();

    println!("Created file {}", path);
}

pub fn generate_template_dir(path: &str, new_path: &str, given_name: &str) -> bool {
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name == ".templify" {
            continue;
        }

        let new_file_name = file_name.replace("$$name$$", given_name);
        let new_path = format!("{}/{}", new_path, new_file_name);

        // check if new_path already exists
        if Path::new(&new_path).exists() {
            println!("File {} already exists.", new_path);
            return false;
        }

        if path.is_dir() {
            std::fs::create_dir(&new_path).unwrap();
            if !generate_template_dir(&path.to_str().unwrap(), &new_path, given_name) {
                return false;
            }
        } else {
            if !generate_template_file(&path.to_str().unwrap(), &new_path, given_name) {
                return false;
            }
        }
    }
    return true;
}

pub fn generate_template_file(path: &str, new_path: &str, given_name: &str) -> bool {
    let file_content = std::fs::read_to_string(path).unwrap();
    let file_content = file_content.replace("$$name$$", given_name);

    if Path::new(new_path).exists() {
        println!("File {} already exists.", new_path);
        return false;
    }

    let mut new_file = std::fs::File::create(new_path).unwrap();
    new_file.write_all(file_content.as_bytes()).unwrap();

    println!("Created file {}", new_path);
    return true;
}

pub fn check_if_templify_initialized() -> bool {
    if !Path::new(".templates").exists() {
        println!("templify is not initialized in this project.");
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };
        println!(
            "Run `{} init` to initialize templify in your project.",
            command_name
        );
        return false;
    }
    return true;
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
