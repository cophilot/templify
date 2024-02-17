// *** Flag ***

pub(crate) struct Command {
    pub names: Vec<String>,
    pub call: fn(&Command) -> Status,
    pub help: String,
    pub arguments: Vec<Argument>,
    pub flags: Vec<Flag>,
}

impl Command {
    pub fn new(names: Vec<String>, call: fn(&Command) -> Status, help: String) -> Command {
        Command {
            names: names,
            call: call,
            help: help,
            arguments: Vec::new(),
            flags: Vec::new(),
        }
    }

    pub fn add_argument(&mut self, argument: Argument) {
        self.arguments.push(argument);
        // sort arguments by index
        self.arguments.sort_by(|a, b| a.at_index.cmp(&b.at_index));
    }

    pub fn add_flag(&mut self, flag: Flag) {
        self.flags.push(flag);
    }

    pub fn is_called(&self, args: &Vec<String>) -> bool {
        let mut args = args.clone();
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

        if args[0] == command_name {
            args = args[1..].to_vec();
        }

        for name in &self.names {
            if args[0] == *name {
                return true;
            }
        }
        return false;
    }

    pub fn parse(&mut self, args: &Vec<String>) -> Status {
        let mut args = args.clone();
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

        if args[0] == command_name {
            args = args[1..].to_vec();
        }

        for name in &self.names {
            if args[0] == *name {
                args = args[1..].to_vec();
                break;
            }
        }

        for argument in &mut self.arguments {
            if !argument.parse(args.clone()) {
                return Status::error(format!("Missing argument: {}", argument.name));
            }
        }
        for flag in &mut self.flags {
            let st = flag.parse(args.clone());
            if !st.is_ok {
                return st;
            }
        }
        return Status::ok();
    }

    pub fn execute(&self) -> Status {
        return (self.call)(self);
    }

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

        if self.flags.len() > 0 {
            help_string.push_str("[flags] ");
        }

        help_string.push_str(&format!("- {}\n", self.help));

        for argument in &self.arguments {
            help_string.push_str(&format!("    {} - {}", argument.name, argument.help));
            help_string.push_str("\n");
        }
        if self.flags.len() > 0 {
            help_string.push_str("    Flags:\n");
        }

        for flag in &self.flags {
            help_string.push_str(format!("    {}\n", flag.to_help_string()).as_str());
        }
        return help_string;
    }

    pub fn get_argument(&self, name: &str) -> &Argument {
        for argument in &self.arguments {
            if argument.name == name {
                return argument;
            }
        }
        panic!("INTERNAL: Argument not found: {}", name);
    }

    pub fn get_value_flag(&self, name: &str) -> String {
        for flag in &self.flags {
            if flag.names.contains(&name.to_string()) && !flag.is_bool {
                return flag.value.clone();
            }
        }
        panic!("INTERNAL: Flag not found: {}", name);
    }

    pub fn get_bool_flag(&self, name: &str) -> bool {
        for flag in &self.flags {
            if flag.names.contains(&name.to_string()) && flag.is_bool {
                return flag.bool_value;
            }
        }
        panic!("INTERNAL: Flag not found: {}", name);
    }
}

// *** Flag ***

pub(crate) struct Flag {
    names: Vec<String>,
    value: String,
    is_bool: bool,
    bool_value: bool,
    help: String,
}

impl Flag {
    pub fn new_value_flag(names: Vec<String>, default_value: String, help: String) -> Flag {
        Flag {
            names: names,
            value: default_value,
            is_bool: false,
            bool_value: false,
            help: help,
        }
    }

    pub fn new_bool_flag(names: Vec<String>, help: String) -> Flag {
        Flag {
            names: names,
            value: String::from(""),
            is_bool: true,
            bool_value: false,
            help: help,
        }
    }

    fn parse(&mut self, args: Vec<String>) -> Status {
        for (i, arg) in args.iter().enumerate() {
            if !arg.starts_with("-") {
                continue;
            }

            for name in &self.names {
                if String::from(arg) == format!("-{}", name) {
                    if self.is_bool {
                        self.bool_value = true;
                    } else {
                        if args.len() <= i + 1 {
                            return Status::error(format!("Missing value for flag: -{}", name));
                        }
                        let mut v = args[i + 1].clone();
                        if v.starts_with("-") {
                            return Status::error(format!("Missing value for flag: -{}", name));
                        }
                        if v.starts_with("/") {
                            v = v[1..].to_string();
                        }
                        self.value = v;
                    }
                    return Status::ok();
                }
            }
        }
        return Status::ok();
    }

    pub fn to_help_string(&self) -> String {
        let mut help_string = String::new();
        if self.names.len() > 1 {
            help_string.push_str("[");
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

        return help_string;
    }
}

// *** Argument ***

pub(crate) struct Argument {
    name: String,
    at_index: usize, // the index is relative to the base command (so the first argument is 0, the second is 1, etc.)
    pub value: String,
    pub is_set: bool,
    required: bool,
    help: String,
}

impl Argument {
    pub fn new(name: String, at_index: usize, required: bool, help: String) -> Argument {
        Argument {
            name: name,
            at_index: at_index,
            value: String::from(""),
            required: required,
            is_set: false,
            help: help,
        }
    }

    fn parse(&mut self, args: Vec<String>) -> bool {
        if args.len() <= self.at_index {
            return !self.required;
        }

        let mut v = args[self.at_index].clone();
        if v.starts_with("-") {
            return !self.required;
        }
        if v.starts_with("/") {
            v = v[1..].to_string();
        }
        self.value = v;
        self.is_set = true;
        return true;
    }
}

// *** Status ***

pub(crate) struct Status {
    pub is_ok: bool,
    pub message: String,
}

impl Status {
    pub fn ok() -> Status {
        Status {
            is_ok: true,
            message: String::from(""),
        }
    }

    pub fn error(message: String) -> Status {
        Status {
            is_ok: false,
            message: message,
        }
    }
}

// ***TemplateMeta***

pub(crate) struct TemplateMeta {
    template_name: String,
    file_path: String,
    map: std::collections::HashMap<String, String>,
}

impl TemplateMeta {
    pub fn new(template_name: String) -> TemplateMeta {
        let mut map = std::collections::HashMap::new();

        map.insert("description".to_string(), "".to_string());
        map.insert("path".to_string(), ".".to_string());
        map.insert(".source".to_string(), "".to_string());

        let file_path = format!(".templates/{}/.templify", template_name);

        let meta = TemplateMeta {
            template_name: template_name.clone(),
            file_path: file_path.clone(),
            map: map,
        };
        return meta;
    }

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

        let first_line = first_line.unwrap().replace(" ", "");
        if first_line.starts_with("#!") {
            let new_divider = first_line.clone().replace("#!", "");

            divider = new_divider.to_string();
        }

        for line in file_content.lines() {
            let line = line.trim();
            if line.starts_with("#") || line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(divider.as_str()).collect();
            if parts.len() < 2 {
                continue;
            }
            let mut second_part = parts[1].to_string();
            if parts.len() > 2 {
                for i in 2..parts.len() {
                    second_part.push_str(format!("{}{}", divider, parts[i]).as_str());
                }
            }

            let key = parts[0].trim().to_string().to_lowercase();
            let value = second_part.trim().to_string();

            meta.map.insert(key, value);
        }

        return meta;
    }

    pub fn get_template_name(&self) -> String {
        return self.template_name.clone();
    }

    pub fn get_description(&self) -> String {
        return self.map["description"].clone();
    }

    pub fn get_path(&self) -> String {
        return self.map["path"].clone();
    }

    pub fn get_source(&self) -> String {
        return self.map[".source"].clone();
    }
}
