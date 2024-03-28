use crate::global_flag_storage::get_all_global_flags;
use crate::log;
use crate::types::argument::Argument;
use crate::types::command::Command;
use crate::types::status::Status;

/// The definition of the help command.
pub(crate) fn definition() -> Command {
    let mut help_commmand = Command::new(
        vec!["help".to_string(), "h".to_string()],
        help,
        "Show this help message.".to_string(),
    );

    help_commmand.add_argument(Argument::new(
        "command".to_string(),
        0,
        false,
        "The command to show help for.".to_string(),
    ));

    help_commmand
}

/// The help command is used to show help messages for all commands.
pub(crate) fn help(command: &Command) -> Status {
    let base_command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    if command.get_argument("command").is_set {
        let mut command_name = command.get_argument("command").value.clone();
        let all_commands = crate::commands::command_storage::get_all_commands();
        for c in all_commands {
            if c.names.contains(&command_name) {
                command_name = c.names[0].clone();
                log!("templify help center");
                log!(" ");
                log!("<...> - required");
                log!("[...] - optional");
                log!(" ");
                log!("Usage: {} {}", base_command_name, command_name);
                log!(" ");
                log!("{}", c.to_help_string());
                log!(
                    "To get more information please visit: https://templify.philipp-bonin.com/#/command/{}", command_name
                );

                return Status::ok();
            }
        }
        return Status::error(format!("Command {} not found.", command_name));
    }

    log!("templify help center");
    log!(" ");
    log!("<...> - required");
    log!("[...] - optional");
    log!(" ");
    log!("Usage: {} <command> [global-flag]", base_command_name);
    log!(" ");
    log!("Global Flags (use with every command):");
    log!(" ");

    let all_flags = get_all_global_flags();
    for flag in all_flags {
        log!("  {}", flag.to_help_string());
    }

    log!(" ");
    log!("Commands:");

    let all_commands = crate::commands::command_storage::get_all_commands();
    for command in all_commands {
        log!("{}", command.to_help_string());
    }

    log!("To get more information please visit: https://templify.philipp-bonin.com");

    Status::ok()
}
