mod command_storage;
mod commands;
mod data;
mod env;
mod executer;
mod types;
mod utils;
mod version_control;

fn main() {
    // ****************************
    // *** Welcome to templify! ***
    // ****************************

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

    if !executer::execute(args) {
        std::process::exit(1);
    }

    version_control::print_update_message();
}
