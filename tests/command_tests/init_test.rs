include!("../common/utils.rs");
include!("../common/fs.rs");

pub fn test() {
    utils::run_successfully("tpy init");

    fs::templates_dir().file("README.md").check_all_exists();
    fs::templates_dir()
        .dir("Example")
        .check_that_dir_is_not_empty();

    utils::run_failure("tpy init");

    utils::reset_dir();

    // test -offline flag
    utils::run_successfully("tpy init -offline");

    fs::templates_dir().file("README.md").check_all_exists();
    fs::templates_dir().dir("Example").check_not_exists();

    utils::run_failure("tpy init -offline");

    utils::reset_dir();

    // test -blank flag
    utils::run_successfully("tpy init -blank");

    fs::templates_dir().check_all_exists();
    fs::templates_dir().file("README.md").check_not_exists();
    fs::templates_dir().dir("Example").check_not_exists();

    utils::run_failure("tpy init -blank");
}
