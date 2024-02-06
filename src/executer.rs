use crate::command_storage;

pub fn execute(args: Vec<String>) -> bool {
    let mut command = command_storage::get_command(&args);
    let parse_status = command.parse(&args);
    let base_command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    if !parse_status.is_ok {
        println!("Command parse error: {}", parse_status.message);
        println!("Run `{} help` for more information.", base_command_name);

        return false;
    }
    let execute_status = command.execute();
    if !execute_status.is_ok {
        println!("Command execution error: {}", execute_status.message);
        println!(
            "Run `{} help {}` for more information.",
            base_command_name, command.names[0]
        );

        return false;
    }
    return true;
}
