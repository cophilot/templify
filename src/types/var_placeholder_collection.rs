use crate::types::status::Status;
use crate::types::var_placeholder::VarPlaceholder;
use indexmap::IndexMap;

#[derive(Clone)]
/// A collection of variable placeholders.
pub(crate) struct VarPlaceholderCollection {
    pub placeholders: IndexMap<String, VarPlaceholder>,
}

impl VarPlaceholderCollection {
    /// Create a new empty VarPlaceholderCollection.
    pub fn new() -> VarPlaceholderCollection {
        VarPlaceholderCollection {
            placeholders: IndexMap::new(),
        }
    }

    /// Add a new placeholder from a configuration string.
    pub fn add_from_conf_string(&mut self, conf_string: String) {
        let placeholder = VarPlaceholder::from_conf_string(conf_string.trim().to_string());
        self.placeholders
            .insert(placeholder.name.clone(), placeholder);
    }

    /// Parse the placeholders values from the given input string.
    pub fn parse_from_input_string(
        &mut self,
        input_string: String,
        names: &mut Vec<String>,
    ) -> Status {
        if input_string.is_empty() {
            return Status::ok();
        }

        let parts = input_string.split(',');
        for part in parts {
            let mut parts = part.split('=');

            let name = parts.next();
            if name.is_none() {
                return Status::error(format!("Invalid input: {}", input_string));
            }
            let name = name.unwrap().trim().to_string();

            let value = parts.next();
            if value.is_none() {
                return Status::error(format!("No value found for: {}", name));
            }
            let value = value.unwrap().trim().to_string();

            if self.placeholders.contains_key(&name) {
                let placeholder = self.placeholders.get_mut(&name).unwrap();
                if placeholder.has_options() && !placeholder.options.contains(&value) {
                    return Status::error(format!("Invalid value for {}: {}", name, value));
                }
                self.placeholders.get_mut(&name).unwrap().set_value(value);
                names.push(name);
            }
        }
        Status::ok()
    }

    /// Check if all placeholders have a value set.
    pub fn are_all_set(&mut self) -> Status {
        let mut message = "Missing value for: ".to_string();
        let mut is_ok = true;

        for placeholder in self.placeholders.values_mut() {
            if !placeholder.is_set {
                message.push_str(format!("{} ", placeholder.name).as_str());
                is_ok = false;
            }
        }

        if is_ok {
            return Status::ok();
        }
        Status::error(message)
    }

    /// Get all placeholders.
    pub fn get_all_placeholders(&mut self) -> Vec<&mut VarPlaceholder> {
        let mut result = Vec::new();
        for placeholder in self.placeholders.values_mut() {
            result.push(placeholder);
        }
        result
    }
}
