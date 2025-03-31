use crate::log;
use crate::types::{command::Command, status::Status};

/// Returns all available commands
pub(crate) fn get_all_commands() -> Vec<Command> {
    let mut commands = Vec::new();

    for module in vec![
        crate::commands::list::definition(),
        crate::commands::help::definition(),
        crate::commands::version::definition(),
        crate::commands::update::definition(),
        crate::commands::init::definition(),
        crate::commands::new::definition(),
        crate::commands::placeholder::definition(),
        crate::commands::load::definition(),
        crate::commands::reload::definition(),
        crate::commands::generate::definition(),
    ] {
        commands.push(module);
    }

    commands
}

/// Returns the command that matches the given arguments
pub(crate) fn get_command(args: &[String]) -> Command {
    let all_commands = get_all_commands();
    for command in all_commands {
        if command.matches(args) {
            return command;
        }
    }

    let mut unknown_command_name = args[0].clone();
    let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    if args[0] == command_name {
        unknown_command_name = args[1].clone();
    }

    Command::new(vec![unknown_command_name], unknown_command, "".to_string())
}

/// Returns a command that prints an error message for when an unknown command is called
fn unknown_command(command: &Command) -> Status {
    let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    log!("Unknown command: {}", command.names[0]);
    log!("Run `{} help` for more information.", command_name);

    Status::ok()
}
