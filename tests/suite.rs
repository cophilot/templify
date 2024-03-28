use std::env;
use std::panic;
use std::vec;
mod command_tests;
include!("./common/utils.rs");
include!("./common/log.rs");

pub static mut LOG_FILE: String = String::new();

fn get_tests() -> Vec<CommandTest> {
    vec![
        CommandTest {
            name: "init_test".to_string(),
            test: command_tests::init_test::test,
            exit_on_failure: true,
        },
        CommandTest {
            name: "new_test".to_string(),
            test: command_tests::new_test::test,
            exit_on_failure: true,
        },
        CommandTest {
            name: "list_test".to_string(),
            test: command_tests::list_test::test,
            exit_on_failure: false,
        },
        CommandTest {
            name: "version_test".to_string(),
            test: command_tests::version_test::test,
            exit_on_failure: false,
        },
        CommandTest {
            name: "help_test".to_string(),
            test: command_tests::help_test::test,
            exit_on_failure: false,
        },
        CommandTest {
            name: "load_test".to_string(),
            test: command_tests::load_test::test,
            exit_on_failure: true,
        },
        CommandTest {
            name: "reload_test".to_string(),
            test: command_tests::reload_test::test,
            exit_on_failure: false,
        },
        CommandTest {
            name: "generate_test".to_string(),
            test: command_tests::generate_test::test,
            exit_on_failure: false,
        },
    ]
}

#[test]
fn run() {
    before_all();

    let tests = get_tests();

    let mut result: Vec<String> = vec![];
    let mut success_test = 0;
    let mut failed_test = 0;

    for test in tests {
        if !run_test(test.clone()) {
            result.push(format!("🚨 Test failed: {}", test.name));
            failed_test += 1;
            if test.exit_on_failure {
                break;
            }
        } else {
            result.push(format!("✅ Test passed: {}", test.name));
            success_test += 1;
        }
    }

    after_all();

    print_result(result, success_test, failed_test);

    assert!(failed_test == 0);
}

fn before_all() {
    unsafe { templify::env::BASE_COMMAND_NAME = "tpy".to_string() };
    log::init();
    env::set_var("RUST_BACKTRACE", "0");
}

fn after_all() {
    env::remove_var("RUST_BACKTRACE");
    log::clean_up();
}

fn before_each(command_test: CommandTest) {
    println!("Running test: {}", command_test.name);
    utils::setup();
}

fn after_each(command_test: CommandTest, success: bool) {
    if !success {
        println!("🚨🚨🚨 Test failed: {}", command_test.name);
    } else {
        println!("✅✅✅ Test passed: {}", command_test.name);
    }
    println!("");
    utils::teardown();
    log::clear();
}

fn run_test(command_test: CommandTest) -> bool
where
    fn(): FnOnce() -> () + panic::UnwindSafe,
{
    before_each(command_test.clone());

    let test = command_test.clone().test;

    let result = panic::catch_unwind(|| test());

    let success = result.is_ok();

    after_each(command_test, success);
    success
}

fn print_result(result: Vec<String>, success_test: i32, failed_test: i32) {
    println!("");
    println!("Test result:");
    println!(
        "✅ Success: {}/{}",
        success_test,
        success_test + failed_test
    );
    if failed_test > 0 {
        println!("🚨 Failed: {}/{}", failed_test, success_test + failed_test);
    }
    println!("");
    for r in result {
        println!("{}", r);
    }
    println!("");
}

#[derive(Clone)]
struct CommandTest {
    name: String,
    test: fn(),
    exit_on_failure: bool,
}
