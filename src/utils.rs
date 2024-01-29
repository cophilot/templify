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
        println!("Run `tpy init` to initialize templify in your project.");
        return false;
    }
    return true;
}
