/// A struct that represents an argument of a command
pub struct Argument {
    pub name: String,
    pub at_index: usize, // the index is relative to the base command (so the first argument is 0, the second is 1, etc.)
    pub value: String,
    pub is_set: bool,
    pub required: bool,
    pub help: String,
}

impl Argument {
    /// Creates a new argument
    pub fn new(name: String, at_index: usize, required: bool, help: String) -> Argument {
        Argument {
            name,
            at_index,
            value: String::from(""),
            required,
            is_set: false,
            help,
        }
    }

    /// Parses the argument from the given arguments
    pub fn parse(&mut self, args: &mut Vec<String>, offset: usize) -> bool {
        // calculate the index of the argument
        let index = self.at_index - offset;

        if args.len() <= index {
            return !self.required;
        }

        let mut v = args[index].clone();
        if v.starts_with('-') {
            return !self.required;
        }
        if v.starts_with('/') {
            v = v[1..].to_string();
        }
        self.value = v;
        self.is_set = true;

        // remove the argument from the arguments
        args.remove(index);

        true
    }
}
