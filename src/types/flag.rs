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

                    let mut j = i + 1;
                    let first_val = &args[j];

                    if first_val.starts_with('-') {
                        return Status::error(format!("Missing value for flag: -{}", name));
                    }
                    let mut v;
                    if first_val.starts_with('\'') || first_val.starts_with('"') {
                        let quote_char = first_val.chars().next();
                        v = first_val[1..].to_string(); // Skip opening quote
                        v.push(' ');
                        j += 1;
                        // Iterate until closing quote
                        while j < args.len() {
                            if args[j].ends_with(quote_char.unwrap()) {
                                v.push_str(&args[j][..args[j].len() - 1]); // Add without closing quote
                                break;
                            } else {
                                v.push_str(&args[j]);
                                v.push(' ');
                            }
                            j += 1;
                        }
                    } else {
                        v = args[j].clone();
                    }

                    if v.starts_with('/') {
                        v = v[1..].to_string();
                    }
                    self.value = v;

                    // remove the flag and its value from the arguments
                    let range = i..=j;
                    args.drain(range);
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
