use templify::env;
use templify::executer;
use templify::log;
use templify::logger;
use templify::utils;

/// The main function of the templify application.
fn main() {
    // ****************************
    // *** Welcome to templify! ***
    // ****************************

    logger::use_stdout();

    let mut args: Vec<String> = std::env::args().collect();

    unsafe { env::BASE_COMMAND_NAME = args[0].clone() };

    exit_when_error(executer::process_global_flags(&mut args));

    if args.len() < 2 {
        log!("Welcome to templify!");
        log!(" ");
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };

        log!("Usage: {} <command>", command_name);
        log!("Run `{} help` for more information.", command_name);
        log!(" ");
        log!("by Philipp B.");
        log!("Have a nice day :)");
        utils::version_control::print_update_message();
        return;
    }

    exit_when_error(executer::execute(args));

    utils::version_control::print_update_message();
}

/// Exit the application when an error occurred.
fn exit_when_error(is_success: bool) {
    if is_success {
        return;
    }
    std::process::exit(1);
}
