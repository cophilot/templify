use std::fmt;

#[derive(Clone)]
/// A variable placeholder.
pub(crate) struct VarPlaceholder {
    pub name: String,
    pub value: String,
    pub options: Vec<String>,
    pub is_set: bool,
}

impl VarPlaceholder {
    /// Create a new variable placeholder from a name.
    pub fn new(name: String) -> VarPlaceholder {
        VarPlaceholder {
            name,
            value: String::from(""),
            options: Vec::new(),
            is_set: false,
        }
    }

    /// Create a new optional variable placeholder from a name and a default value.
    pub fn new_optional(name: String, default_value: String) -> VarPlaceholder {
        VarPlaceholder {
            name,
            value: default_value,
            options: Vec::new(),
            is_set: true,
        }
    }

    /// Create a new variable placeholder from a configuration string.
    pub fn from_conf_string(conf_string: String) -> VarPlaceholder {
        if conf_string.contains('(') && conf_string.contains(')') {
            return VarPlaceholder::from_conf_with_default(conf_string);
        }
        if conf_string.contains('[') && conf_string.contains(']') {
            return VarPlaceholder::from_conf_with_options(conf_string);
        }
        VarPlaceholder::new(conf_string)
    }

    /// Create a new variable placeholder from a configuration string with a default value.
    fn from_conf_with_default(conf_string: String) -> VarPlaceholder {
        let name = conf_string.split('(').collect::<Vec<&str>>()[0]
            .trim()
            .to_string();
        let default_value = conf_string.split('(').collect::<Vec<&str>>()[1]
            .split(')')
            .collect::<Vec<&str>>()[0]
            .trim()
            .to_string();
        VarPlaceholder::new_optional(name, default_value)
    }

    /// Create a new variable placeholder from a configuration string with options.
    fn from_conf_with_options(conf_string: String) -> VarPlaceholder {
        let name = conf_string.split('[').collect::<Vec<&str>>()[0]
            .trim()
            .to_string();
        let options = conf_string.split('[').collect::<Vec<&str>>()[1]
            .split(']')
            .collect::<Vec<&str>>()[0]
            .trim()
            .to_string();

        let mut var = VarPlaceholder::new(name);

        for o in options.split(',') {
            var.add_option(o.trim().to_string());
        }

        var
    }

    /// Add an option to the variable placeholder.
    fn add_option(&mut self, options: String) {
        self.options.push(options);
    }

    /// Check if the variable placeholder has options.
    pub fn has_options(&self) -> bool {
        !self.options.is_empty()
    }

    /// Set the value of the variable placeholder.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.is_set = true;
    }
}

impl fmt::Display for VarPlaceholder {
    /// The string representation of a variable placeholder.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_set {
            return write!(f, "{}", self.name);
        }
        write!(f, "{} ({})", self.name, self.value)
    }
}
