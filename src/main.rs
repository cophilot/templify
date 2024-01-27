mod commands;
mod data;

fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: tmy <command>");
        println!("Run `tmy help` for more information.");
        return;
    }

    // match command
    match args[1].to_lowercase().as_str() {
        "init" => commands::init(),
        "i" => commands::init(),
        "help" => commands::help(),
        _ => println!("Unknown command: {}", args[1]),
    }
}
