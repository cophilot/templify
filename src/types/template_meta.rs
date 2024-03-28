use crate::types::var_placeholder_collection::VarPlaceholderCollection;

#[derive(Clone)]
/// The meta information of a template.
pub(crate) struct TemplateMeta {
    template_name: String,
    file_path: String,
    map: std::collections::HashMap<String, String>,
    pub var_placeholder_collection: VarPlaceholderCollection,
}

impl TemplateMeta {
    /// Create a new TemplateMeta instance for the given template name.
    pub fn new(template_name: String) -> TemplateMeta {
        let mut map = std::collections::HashMap::new();

        map.insert("description".to_string(), "".to_string());
        map.insert("path".to_string(), ".".to_string());
        map.insert(".source".to_string(), "".to_string());

        let file_path = format!(".templates/{}/.templify", template_name);

        TemplateMeta {
            template_name: template_name.clone(),
            file_path: file_path.clone(),
            map,
            var_placeholder_collection: VarPlaceholderCollection::new(),
        }
    }

    /// Parse the template meta information from the file system.
    pub fn parse(template_name: String) -> TemplateMeta {
        let mut meta = TemplateMeta::new(template_name.clone());

        let file_content = std::fs::read_to_string(meta.file_path.clone());
        if file_content.is_err() {
            return meta;
        }

        let file_content = file_content.unwrap();

        let mut divider = ":".to_string();

        let first_line = file_content.lines().next();
        if first_line.is_none() {
            return meta;
        }

        let first_line = first_line.unwrap().replace(' ', "");
        if first_line.starts_with("#?") {
            let new_divider = first_line.clone().replace("#?", "");

            divider = new_divider.to_string();
        }

        for line in file_content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(divider.as_str()).collect();
            if parts.len() < 2 {
                continue;
            }
            let mut second_part = parts[1].to_string();
            if parts.len() > 2 {
                for part in parts.iter().skip(2) {
                    second_part.push_str(format!("{}{}", divider, part).as_str());
                }
            }

            let key = parts[0].trim().to_string().to_lowercase();
            let value = second_part.trim().to_string();

            if key == "var" || key == "variable" {
                meta.var_placeholder_collection.add_from_conf_string(value);
                continue;
            }

            meta.map.insert(key, value);
        }

        meta
    }

    /// Returns the template name.
    pub fn get_template_name(&self) -> String {
        self.template_name.clone()
    }

    /// Returns the description that is specified in the template meta information.
    pub fn get_description(&self) -> String {
        self.map["description"].clone()
    }

    /// Returns the path that is specified in the template meta information.
    pub fn get_path(&self) -> String {
        self.map["path"].clone()
    }

    /// Returns the source that is specified in the template meta information.
    pub fn get_source(&self) -> String {
        self.map[".source"].clone()
    }
}
