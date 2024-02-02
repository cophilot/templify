mod commands;
mod data;
mod env;
mod utils;
mod version_control;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    unsafe { env::BASE_COMMAND_NAME = args[0].clone() };

    if args.len() < 2 {
        println!("Welcome to templify!");
        println!("");
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

        println!("Usage: {} <command>", command_name);
        println!("Run `{} help` for more information.", command_name);
        println!("");
        println!("by Philipp B.");
        println!("Have a nice day :)");
        version_control::print_update_message();
        return;
    }

    // match command
    match args[1].to_lowercase().as_str() {
        "help" => commands::help(),
        "-h" => commands::help(),
        "version" => commands::version(),
        "-v" => commands::version(),
        "init" => commands::init(),
        "i" => commands::init(),
        "new" => commands::new(args),
        "n" => commands::new(args),
        "list" => commands::list(),
        "ls" => commands::list(),
        "load" => commands::load(args),
        "l" => commands::load(args),
        "generate" => commands::generate(args),
        "g" => commands::generate(args),
        "update" => commands::update(),
        _ => println!("Unknown command: {}", args[1]),
    }

    version_control::print_update_message();
}
