use crate::types::status::Status;

/// A flag that can be used with a command.
pub struct Flag {
    pub names: Vec<String>,
    pub value: String,
    pub is_bool: bool,
    pub bool_value: bool,
    pub help: String,
}

impl Flag {
    /// Creates a new value flag
    pub fn new_value_flag(names: Vec<String>, default_value: String, help: String) -> Flag {
        Flag {
            names,
            value: default_value,
            is_bool: false,
            bool_value: false,
            help,
        }
    }

    /// Creates a new boolean flag
    pub fn new_bool_flag(names: Vec<String>, help: String) -> Flag {
        Flag {
            names,
            value: String::from(""),
            is_bool: true,
            bool_value: false,
            help,
        }
    }

    /// Parses the flag from the given arguments
    pub fn parse(&mut self, args: &mut Vec<String>) -> Status {
        for (i, arg) in args.iter().enumerate() {
            // skip non-flag arguments
            if !arg.starts_with('-') {
                continue;
            }

            for name in &self.names {
                if *arg != format!("-{}", name) {
                    continue;
                }

                if self.is_bool {
                    self.bool_value = true;

                    // remove the flag from the arguments
                    args.remove(i);
                } else {
                    if args.len() <= i + 1 {
                        return Status::error(format!("Missing value for flag: -{}", name));
                    }

                    let mut v = args[i + 1].clone();
                    if v.starts_with('-') {
                        return Status::error(format!("Missing value for flag: -{}", name));
                    }

                    if v.starts_with('/') {
                        v = v[1..].to_string();
                    }
                    self.value = v;

                    // remove the flag and its value from the arguments
                    args.remove(i);
                    args.remove(i);
                }
                return Status::ok();
            }
        }
        Status::ok()
    }

    /// Get the help string for the flag.
    pub fn to_help_string(&self) -> String {
        let mut help_string = String::new();
        if self.names.len() > 1 {
            help_string.push('[');
            for name in &self.names {
                help_string.push_str(&format!("-{}|", name));
            }
            help_string.pop();
            help_string.push_str("] ");
        } else {
            help_string.push_str(&format!("-{} ", self.names[0]));
        }

        if self.is_bool {
            help_string.push_str(&format!("- {}", self.help));
        } else {
            help_string.push_str(&format!("<value> - {}", self.help));
        }

        help_string
    }
}
