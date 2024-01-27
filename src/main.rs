mod commands;
mod data;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: tpy <command>");
        println!("Run `tpy help` for more information.");
        return;
    }

    // match command
    match args[1].to_lowercase().as_str() {
        "help" => commands::help(),
        "h" => commands::help(),
        "version" => commands::version(),
        "v" => commands::version(),
        "init" => commands::init(),
        "i" => commands::init(),
        "new" => commands::new(args),
        "n" => commands::new(args),
        "list" => commands::list(),
        "ls" => commands::list(),
        "generate" => commands::generate(args),
        "g" => commands::generate(args),
        _ => println!("Unknown command: {}", args[1]),
    }
}
