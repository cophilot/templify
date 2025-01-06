use self::fs::FSItem;

include!("../common/utils.rs");
include!("../common/log.rs");
include!("../common/fs.rs");

pub fn test() {
    utils::init_tpy();
    utils::run_failure("tpy generate");

    // test name matching and -strict flag
    utils::run_successfully("tpy new Component -path src/$$name$$/subdir");
    utils::run_successfully("tpy new Command -path src/commands/subdir");
    utils::run_failure("tpy generate comp"); // Missing name
    utils::run_failure("tpy generate com test"); // not unique

    log::clear();
    utils::run_successfully("tpy generate comp test");
    log::contains_line("Generating new files from template Component...");

    log::clear();
    utils::run_successfully("tpy generate comm test");
    log::contains_line("Generating new files from template Command...");

    utils::run_failure("tpy generate comm test -strict ");
    utils::run_successfully("tpy generate Command test -strict");

    // test default behavior
    let mut base_path = fs::templates_dir().dir("Component");
    base_path
        .file("Component$$name$$.rs")
        .create()
        .check_all_exists();
    base_path
        .dir("dir$$name$$")
        .create()
        .file("file$$name$$.js")
        .create_file("My Name is: file$$name$$.js")
        .check_all_exists();
    base_path
        .dir("empty_dir")
        .create()
        .file(".tpykeep")
        .create()
        .check_all_exists();

    utils::run_successfully("tpy generate comp foo");

    fs::dir("src")
        .dir("foo")
        .dir("subdir")
        .file("Componentfoo.rs")
        .check_all_exists();
    fs::dir("src")
        .dir("foo")
        .dir("subdir")
        .dir("dirfoo")
        .file("filefoo.js")
        .contains_string("My Name is: filefoo.js")
        .check_all_exists();
    fs::dir("src")
        .dir("foo")
        .dir("subdir")
        .dir("empty_dir")
        .check_all_exists()
        .file(".tpykeep")
        .check_not_exists();

    utils::run_failure("tpy generate comp foo");

    utils::run_successfully("tpy generate comp bar");

    fs::dir("src")
        .dir("bar")
        .dir("subdir")
        .file("Componentbar.rs")
        .check_all_exists();
    fs::dir("src")
        .dir("bar")
        .dir("subdir")
        .dir("dirbar")
        .file("filebar.js")
        .contains_string("My Name is: filebar.js")
        .check_all_exists();
    fs::dir("src")
        .dir("bar")
        .dir("subdir")
        .dir("empty_dir")
        .check_all_exists()
        .file(".tpykeep")
        .check_not_exists();

    utils::run_failure("tpy generate comp bar");

    // test -dry-run flag
    fs::templates_dir()
        .dir("Command")
        .file("TestCommand.txt")
        .create_file("$$name$$")
        .check_all_exists();
    log::clear();
    utils::run_successfully("tpy generate comm test -dry-run");
    fs::dir("src")
        .dir("commands")
        .dir("subdir")
        .file("TestCommand.txt")
        .check_not_exists();
    log::contains_line("Would create file src/commands/subdir/TestCommand.txt");

    // test -force flag
    utils::run_successfully("tpy generate comm FOO");
    fs::dir("src")
        .dir("commands")
        .dir("subdir")
        .file("TestCommand.txt")
        .contains_string("FOO")
        .check_all_exists();
    utils::run_failure("tpy generate comm BAR");
    utils::run_successfully("tpy generate comm BAR -force");
    fs::dir("src")
        .dir("commands")
        .dir("subdir")
        .file("TestCommand.txt")
        .not_contains_string("FOO")
        .contains_string("BAR")
        .check_all_exists();

    // check case conversion
    fs::templates_dir()
        .dir("Command")
        .file("TestCommand.txt")
        .remove()
        .create_file("$$name$$")
        .append_line("$$name.lower$$")
        .append_line("$$name.upper$$")
        .append_line("$$name.camel$$")
        .append_line("$$name.snake$$")
        .append_line("$$name.kebab$$")
        .append_line("$$name.pascal$$")
        .append_line("$$name.macro$$")
        .append_line("$$name.train$$");
    let mut file = fs::dir("src")
        .dir("commands")
        .dir("subdir")
        .file("TestCommand.txt");
    utils::run_successfully("tpy generate comm myFirstComponent -force");
    check_case_conversion_generated_file(&mut file);
    utils::run_successfully("tpy generate comm my_first_component -force");
    check_case_conversion_generated_file(&mut file);
    utils::run_successfully("tpy generate comm My-First-Component -force");
    check_case_conversion_generated_file(&mut file);

    // test variable placeholders
    fs::templates_dir()
        .dir("Command")
        .file(".templify.yml")
        .append_line("vars:")
        .append_line(" - test1")
        .append_line(" - test2(default_value)")
        .append_line(" - test3[opt1,opt2,opt3]");
    fs::templates_dir()
        .dir("Command")
        .file("TestCommand.txt")
        .append_line("$$test1$$")
        .append_line("$$test2$$")
        .append_line("$$test3$$");
    utils::run_successfully("tpy generate comm test -force -default-var");
    fs::dir("src")
        .dir("commands")
        .dir("subdir")
        .file("TestCommand.txt")
        .contains_string("unknown")
        .contains_string("default_value")
        .contains_string("opt1");
    utils::run_successfully("tpy generate comm test -force -var test1=foo,test2=bar,test3=opt3");
    fs::dir("src")
        .dir("commands")
        .dir("subdir")
        .file("TestCommand.txt")
        .contains_string("foo")
        .contains_string("bar")
        .contains_string("opt3");
    utils::run_failure("tpy generate comm test -force -var test1=foo,test2=bar,test3=opt4");

    // test -reload flag
    utils::run_successfully(
        "tpy load https://github.com/cophilot/templify-vault/tree/main/Test/MyTest -t",
    );
    fs::templates_dir()
        .dir("MyTest")
        .file("file.txt")
        .remove()
        .check_not_exists();
    utils::run_successfully("tpy generate MyTest test -reload");
    fs::dir("src").file("file.txt").check_all_exists();
}

fn check_case_conversion_generated_file(file: &mut FSItem) {
    file.check_all_exists()
        .contains_string("myfirstcomponent")
        .contains_string("MYFIRSTCOMPONENT")
        .contains_string("myFirstComponent")
        .contains_string("my_first_component")
        .contains_string("my-first-component")
        .contains_string("MyFirstComponent")
        .contains_string("MY_FIRST_COMPONENT")
        .contains_string("My-First-Component");
}
