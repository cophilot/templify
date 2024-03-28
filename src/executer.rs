use crate::{error, global_flag_storage::get_all_global_flags};

/// This module is responsible for executing a command.
pub fn execute(args: Vec<String>) -> bool {
    let mut command = crate::commands::command_storage::get_command(&args);
    let parse_status = command.parse(&args);
    let base_command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    if !parse_status.is_ok {
        error!("Command parse error: {}", parse_status.message);
        error!("Run `{} help` for more information.", base_command_name);

        return false;
    }
    let execute_status = command.execute();
    if !execute_status.is_ok {
        error!("Command execution error: {}", execute_status.message);
        error!(
            "Run `{} help {}` for more information.",
            base_command_name, command.names[0]
        );

        return false;
    }
    true
}

/// Handle the global flags
pub fn process_global_flags(args: &mut Vec<String>) -> bool {
    let base_command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    // Parse all global flags
    let mut flags = get_all_global_flags();
    for flag in &mut flags {
        let status = flag.parse(args);
        if !status.is_ok {
            error!("Global Flag parse error: {}", status.message);
            error!("Run `{} help` for more information.", base_command_name);

            return false;
        }
    }

    // Execute callback
    for flag in &mut flags {
        if !flag.is_set {
            continue;
        }
        let status = flag.call();
        if !status.is_ok {
            error!("Global Flag execution error: {}", status.message);
            error!("Run `{} help` for more information.", base_command_name);

            return false;
        }
    }

    true
}
