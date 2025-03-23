use crate::types::var_placeholder_collection::VarPlaceholderCollection;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

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
        map.insert("command".to_string(), "".to_string());

        let mut file_path = format!(".templates/{}/.templify.yaml", template_name);
        if !std::path::Path::new(&file_path).exists() {
            file_path = format!(".templates/{}/.templify.yml", template_name);
        }

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
        let yaml = match YamlLoader::load_from_str(&file_content) {
            Ok(yaml) => yaml,
            Err(_) => return meta,
        };

        if yaml.is_empty() {
            return meta;
        }

        let yaml = &yaml[0];
        if let Yaml::Hash(hash) = yaml {
            for (key, value) in hash {
                let k = key.as_str().unwrap().to_string();
                if k == "vars" || k == "variables" {
                    if value.is_array() {
                        for v in value.as_vec().unwrap() {
                            let v = v.as_str().unwrap().to_string();
                            meta.var_placeholder_collection
                                .add_from_conf_string(v.clone());
                        }
                        continue;
                    }
                    // when the value is not an array, it is a string
                    let v = value.as_str().unwrap().to_string();
                    meta.var_placeholder_collection
                        .add_from_conf_string(v.clone());
                    continue;
                }

                let mut v_opt = value.as_str();
                if v_opt.is_none() {
                    v_opt = Some("");
                }
                let v = v_opt.unwrap().to_string();

                meta.map.insert(k.clone(), v.clone());
            }
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

    /// Returns the command that is specified in the template meta information.
    pub fn get_command(&self) -> String {
        self.map["command"].clone()
    }
}
