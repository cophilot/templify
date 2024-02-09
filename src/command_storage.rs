use crate::commands::{self};
use crate::types::{Argument, Command, Flag, Status};

pub fn get_all_commands() -> Vec<Command> {
    let mut commands = Vec::new();

    // *** help ***

    let mut help_com = Command::new(
        vec!["help".to_string(), "h".to_string()],
        commands::help,
        "Show this help message.".to_string(),
    );

    help_com.add_argument(Argument::new(
        "command".to_string(),
        0,
        false,
        "The command to show help for.".to_string(),
    ));

    commands.push(help_com);

    // *** version ***

    let version_com = Command::new(
        vec!["version".to_string(), "v".to_string()],
        commands::version,
        "Print the current version of templify.".to_string(),
    );

    commands.push(version_com);

    // *** update ***

    let mut update_com = Command::new(
        vec!["update".to_string()],
        commands::update,
        "Update templify to the latest version.".to_string(),
    );

    update_com.add_flag(Flag::new_value_flag(
        vec!["version".to_string(), "v".to_string()],
        "".to_string(),
        "Update to a specific version.".to_string(),
    ));

    commands.push(update_com);

    // *** init ***

    let mut init_com = Command::new(
        vec!["init".to_string(), "i".to_string()],
        commands::init,
        "Initialize templify in the current directory.".to_string(),
    );

    init_com.add_flag(Flag::new_bool_flag(
        vec!["offline".to_string(), "o".to_string()],
        "Do not fetch the example template from the internet.".to_string(),
    ));

    commands.push(init_com);

    // *** new ***

    let mut new_com = Command::new(
        vec!["new".to_string(), "n".to_string()],
        commands::new,
        "Create a new template with the given name.".to_string(),
    );

    new_com.add_argument(Argument::new(
        "template-name".to_string(),
        0,
        true,
        "The name of the new template.".to_string(),
    ));

    new_com.add_flag(Flag::new_value_flag(
        vec!["description".to_string(), "d".to_string()],
        "".to_string(),
        "Provide a description for the new template.".to_string(),
    ));

    new_com.add_flag(Flag::new_value_flag(
        vec!["path".to_string(), "p".to_string()],
        ".".to_string(),
        "Provide a path for the new template.".to_string(),
    ));

    commands.push(new_com);

    // *** list ***

    let list_com = Command::new(
        vec!["list".to_string(), "ls".to_string()],
        commands::list,
        "List all available templates in the current project.".to_string(),
    );

    commands.push(list_com);

    // *** load ***

    let mut load_com = Command::new(
        vec!["load".to_string(), "l".to_string()],
        commands::load,
        "Load templates from a github repository.".to_string(),
    );

    load_com.add_argument(Argument::new(
        "url".to_string(),
        0,
        true,
        "The url of the github repository.".to_string(),
    ));

    load_com.add_flag(Flag::new_bool_flag(
        vec!["force".to_string(), "f".to_string()],
        "Force the load, even if the folder already exists.".to_string(),
    ));

    commands.push(load_com);

    // *** generate ***

    let mut generate_com = Command::new(
        vec!["generate".to_string(), "g".to_string()],
        commands::generate,
        "Create a new file from the given template.".to_string(),
    );

    generate_com.add_argument(Argument::new(
        "template-name".to_string(),
        0,
        true,
        "The name of the template to use.".to_string(),
    ));

    generate_com.add_argument(Argument::new(
        "new-name".to_string(),
        1,
        true,
        "The name of the new file.".to_string(),
    ));

    generate_com.add_flag(Flag::new_bool_flag(
        vec!["strict".to_string()],
        "If enabled the template name must match exactly.".to_string(),
    ));

    generate_com.add_flag(Flag::new_bool_flag(
        vec!["dry-run".to_string(), "dr".to_string()],
        "If enabled the file will not be created and the output will be printed.".to_string(),
    ));

    commands.push(generate_com);

    return commands;
}

pub fn get_command(args: &Vec<String>) -> Command {
    let all_commands = get_all_commands();
    for command in all_commands {
        if command.is_called(&args) {
            return command;
        }
    }

    let mut unknown_command_name = args[0].clone();
    let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    if args[0] == command_name {
        unknown_command_name = args[1].clone();
    }

    return Command::new(vec![unknown_command_name], uknown_command, "".to_string());
}

pub fn uknown_command(command: &Command) -> Status {
    let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

    println!("Unknown command: {}", command.names[0]);
    println!("Run `{} help` for more information.", command_name);

    return Status::ok();
}
