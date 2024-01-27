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

pub fn check_if_templify_initialized() -> bool {
    if !std::path::Path::new(".templates").exists() {
        println!("Templify is not initialized in this project.");
        println!("Run `tpy init` to initialize Templify in your project.");
        return false;
    }
    return true;
}
