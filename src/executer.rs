use crate::command_storage;

pub fn execute(args: Vec<String>) -> bool {
    let mut command = command_storage::get_command(&args);
    let parse_status = command.parse(&args);
    if !parse_status.is_ok {
        println!("Command parse error: {}", parse_status.message);
        return false;
    }
    let execute_status = command.execute();
    if !execute_status.is_ok {
        println!("Command execution error: {}", execute_status.message);
        return false;
    }
    return true;
}
