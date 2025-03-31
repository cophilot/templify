use crate::log;
use crate::utils;

use crate::types::template_meta::TemplateMeta;

#[derive(Clone, Debug)]
/// A snippet is a text/code snippet that can be added to s specific file when gerating from a template.
pub(crate) struct Snippet {
    pub id: String,
    pub file_name: String,
    pub content: String,
    pub before: bool,
}

impl Snippet {
    /// Get a new Snippet instance from a YAML object.
    pub fn from_yaml(yaml: &yaml_rust::yaml::Yaml) -> Snippet {
        let id = yaml["id"].as_str().unwrap_or("").to_string();
        let file_name = yaml["file"].as_str().unwrap_or("").to_string();
        let content = yaml["content"].as_str().unwrap_or("").to_string();
        let before = yaml["before"].as_bool().unwrap_or(false);
        Snippet {
            id,
            file_name,
            content,
            before,
        }
    }

    /// Generate the snippet and insert it into the file.
    pub fn generate(self) {
        if !std::path::Path::new(&self.file_name).exists() {
            log!(
                "Cannot create snippet '{}' because the file '{}' does not exist.",
                self.id,
                self.file_name
            );
            return;
        }

        let raw_id = format!("~~{}~~", self.id);

        let file_content = std::fs::read_to_string(&self.file_name).unwrap();
        let lines = file_content.lines().collect::<Vec<&str>>();

        let mut new_lines = Vec::new();

        for line in lines.iter() {
            let mut skip_insert = false;

            if line.contains(&raw_id) {
                if self.before {
                    self.clone().add_content(&mut new_lines);
                } else {
                    new_lines.push(line.to_string());
                    self.clone().add_content(&mut new_lines);
                    skip_insert = true;
                }
                log!(
                    "Snippet '{}' inserted into file '{}'.",
                    self.id,
                    self.file_name
                );
            }

            if !skip_insert {
                new_lines.push(line.to_string());
            }
        }

        let new_content = new_lines.join("\n");

        std::fs::write(&self.file_name, new_content).unwrap();
    }

    /// Add the content of the snippet to the given vector. Changes the vector in place.
    fn add_content(self, vec: &mut Vec<String>) {
        let content_lines = self.content.split("\\n").collect::<Vec<&str>>();
        for line in content_lines.iter() {
            vec.push(line.to_string());
        }
    }

    /// Parse the placeholders in the snippet. This should be done before generating the snippet.
    pub fn parse_placeholders(&mut self, name: &str, meta: TemplateMeta) {
        self.content =
            utils::formater::handle_placeholders(self.content.as_str(), name, meta.clone());
        self.file_name =
            utils::formater::handle_placeholders(self.file_name.as_str(), name, meta.clone());
    }
}
