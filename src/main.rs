mod commands;
mod data;
mod env;
mod utils;
mod version_control;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Welcome to templify!");
        println!("");
        println!("Usage: tpy <command>");
        println!("Run `tpy help` for more information.");
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
        "generate" => commands::generate(args),
        "g" => commands::generate(args),
        "update" => commands::update(),
        _ => println!("Unknown command: {}", args[1]),
    }

    version_control::print_update_message();
}
