use super::load_test;

include!("../common/utils.rs");
include!("../common/log.rs");
include!("../common/fs.rs");

pub fn test() {
    utils::init_tpy();
    utils::run_successfully("tpy load https://github.com/cophilot/templify-vault/tree/main/Test");

    let mut base = fs::templates_dir().dir("Test1");
    let mut file = base.file("Test1$$name$$.txt");
    file.remove();
    file.check_not_exists();
    let mut new_file = base.file("this_is_a_new_file.txt");
    new_file.create();
    new_file.check_all_exists();

    utils::run_successfully("tpy reload Test1");
    load_test::check_test_1_structure();
    new_file.check_all_exists();

    // check -reset flag
    file.remove();
    utils::run_successfully("tpy reload Test1 -reset");
    load_test::check_test_1_structure();
    new_file.check_not_exists();

    // check naming convention
    utils::run_failure("tpy reload test");
    log::clear();
    utils::run_successfully("tpy reload m");
    log::contains_line("Template MyTest reloaded successfully.");
    utils::run_failure("tpy reload m -strict");
    utils::run_successfully("tpy reload MyTest -strict");
    log::contains_line("Template MyTest reloaded successfully.");

    // check reloading all templates
    log::clear();
    utils::run_successfully("tpy reload");
    log::contains_line("Template MyTest reloaded successfully.");
    log::contains_line("Template Test1 reloaded successfully.");
    log::contains_line("Template Test2 reloaded successfully.");

    // check reloading template without source
    utils::run_successfully("tpy new NewTemplate");
    utils::run_failure("tpy reload NewTemplate");
}
