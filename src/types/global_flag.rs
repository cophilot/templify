use crate::types::status::Status;

/// A flag that can be used with any command.
pub(crate) struct GlobalFlag {
    pub names: Vec<String>,
    pub value: String,
    pub is_set: bool,
    pub is_bool: bool,
    pub bool_value: bool,
    pub help: String,
    pub callback_bool: fn() -> Status,
    pub callback_value: fn(arg: String) -> Status,
}

impl GlobalFlag {
    #[allow(dead_code)]
    /// Creates a new value global flag
    pub fn new_value_flag(
        names: Vec<String>,
        call: fn(arg: String) -> Status,
        help: String,
    ) -> GlobalFlag {
        GlobalFlag {
            names,
            value: String::from(""),
            is_set: false,
            is_bool: false,
            bool_value: false,
            help,
            callback_bool: Status::ok,
            callback_value: call,
        }
    }

    /// Creates a new boolean global flag
    pub fn new_bool_flag(names: Vec<String>, call: fn() -> Status, help: String) -> GlobalFlag {
        GlobalFlag {
            names,
            value: String::from(""),
            is_set: false,
            is_bool: true,
            bool_value: false,
            help,
            callback_bool: call,
            callback_value: |_| Status::ok(),
        }
    }

    /// Calls the flag's callback
    pub fn call(&self) -> Status {
        if self.is_bool {
            return (self.callback_bool)();
        }
        (self.callback_value)(self.value.clone())
    }

    /// Parses the flag from the given arguments
    pub fn parse(&mut self, args: &mut Vec<String>) -> Status {
        for (i, arg) in args.iter().enumerate() {
            // skip non-flag arguments
            if !arg.starts_with("--") {
                continue;
            }

            for name in &self.names {
                if *arg != format!("--{}", name) {
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
                self.is_set = true;
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
                help_string.push_str(&format!("--{}|", name));
            }
            help_string.pop();
            help_string.push_str("] ");
        } else {
            help_string.push_str(&format!("--{} ", self.names[0]));
        }

        if self.is_bool {
            help_string.push_str(&format!("- {}", self.help));
        } else {
            help_string.push_str(&format!("<value> - {}", self.help));
        }

        help_string
    }
}
