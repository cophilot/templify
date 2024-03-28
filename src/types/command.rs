use crate::types::argument::Argument;
use crate::types::flag::Flag;
use crate::types::status::Status;

/// A command that can be executed by the user.
pub(crate) struct Command {
    pub names: Vec<String>,
    pub call: fn(&Command) -> Status,
    pub help: String,
    pub arguments: Vec<Argument>,
    pub flags: Vec<Flag>,
}

impl Command {
    /// Create a new command.
    pub fn new(names: Vec<String>, call: fn(&Command) -> Status, help: String) -> Command {
        Command {
            names,
            call,
            help,
            arguments: Vec::new(),
            flags: Vec::new(),
        }
    }

    /// Add an argument to the command.
    pub fn add_argument(&mut self, argument: Argument) {
        self.arguments.push(argument);
        // sort arguments by index
        self.arguments.sort_by(|a, b| a.at_index.cmp(&b.at_index));
    }

    /// Add a flag to the command.
    pub fn add_flag(&mut self, flag: Flag) {
        self.flags.push(flag);
    }

    /// Check if the command matches the given arguments.
    pub fn matches(&self, args: &[String]) -> bool {
        let mut args = args.to_owned();
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

        if args[0] == command_name {
            args = args[1..].to_vec();
        }

        for name in &self.names {
            if args[0] == *name {
                return true;
            }
        }
        false
    }

    /// Parse the arguments and flags of the command.
    pub fn parse(&mut self, args: &[String]) -> Status {
        let mut args = args.to_owned();
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

        // Remove base command name if present
        if args[0] == command_name {
            args = args[1..].to_vec();
        }

        // Remove the own command name from the arguments
        for name in &self.names {
            if args[0] == *name {
                args = args[1..].to_vec();
                break;
            }
        }

        // Parse the flags
        for flag in &mut self.flags {
            let st = flag.parse(&mut args);
            if !st.is_ok {
                return st;
            }
        }

        // Parse the arguments
        for (offset, argument) in self.arguments.iter_mut().enumerate() {
            if !argument.parse(&mut args, offset) {
                return Status::error(format!("Missing argument: {}", argument.name));
            }
        }

        if !args.is_empty() {
            return Status::error(format!("Unknown argument: {}", args[0]));
        }

        Status::ok()
    }

    /// Execute the command.
    pub fn execute(&self) -> Status {
        (self.call)(self)
    }

    /// Get the help string for the command.
    pub fn to_help_string(&self) -> String {
        let mut help_string = String::new();
        help_string.push_str("  ");

        let names = format!("<{}> ", self.names.join("|"));
        help_string.push_str(names.as_str());

        for argument in &self.arguments {
            if argument.required {
                help_string.push_str(&format!("<{}> ", argument.name));
            } else {
                help_string.push_str(&format!("[{}] ", argument.name));
            }
        }

        if !self.flags.is_empty() {
            help_string.push_str("[flags] ");
        }

        help_string.push_str(&format!("- {}\n", self.help));

        for argument in &self.arguments {
            help_string.push_str(&format!("    {} - {}", argument.name, argument.help));
            help_string.push('\n');
        }
        if !self.flags.is_empty() {
            help_string.push_str("    Flags:\n");
        }

        for flag in &self.flags {
            help_string.push_str(format!("    {}\n", flag.to_help_string()).as_str());
        }
        help_string
    }

    /// Get the argument with the given name.
    pub fn get_argument(&self, name: &str) -> &Argument {
        for argument in &self.arguments {
            if argument.name == name {
                return argument;
            }
        }
        panic!("INTERNAL: Argument not found: {}", name);
    }

    /// Get the value of the value flag with the given name.
    pub fn get_value_flag(&self, name: &str) -> String {
        for flag in &self.flags {
            if flag.names.contains(&name.to_string()) && !flag.is_bool {
                return flag.value.clone();
            }
        }
        panic!("INTERNAL: Flag not found: {}", name);
    }

    /// Get the value of the bool flag with the given name.
    pub fn get_bool_flag(&self, name: &str) -> bool {
        for flag in &self.flags {
            if flag.names.contains(&name.to_string()) && flag.is_bool {
                return flag.bool_value;
            }
        }
        panic!("INTERNAL: Flag not found: {}", name);
    }
}
