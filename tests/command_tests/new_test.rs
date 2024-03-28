include!("../common/utils.rs");
include!("../common/fs.rs");

pub fn test() {
    utils::init_tpy();

    utils::run_failure("tpy new");

    utils::run_successfully("tpy new Test1");

    fs::templates_dir()
        .dir("Test1")
        .file(".templify")
        .check_all_exists();

    utils::run_failure("tpy new Test1");

    // test -description flag
    utils::run_successfully("tpy new Test2 -description this_is_a_description");

    fs::templates_dir()
        .dir("Test2")
        .file(".templify")
        .check_all_exists()
        .contains_string("description:this_is_a_description");

    utils::run_failure("tpy new Test2");

    // test -path flag
    utils::run_successfully("tpy new Test3 -path this/is/a/path");

    fs::templates_dir()
        .dir("Test3")
        .file(".templify")
        .check_all_exists()
        .contains_string("path:this/is/a/path");

    utils::run_failure("tpy new Test3");

    // test -description and -path flags together
    utils::run_successfully(
        "tpy new Test4 -path this/is/a/path -description this_is_a_description",
    );

    fs::templates_dir()
        .dir("Test4")
        .file(".templify")
        .check_all_exists()
        .contains_string("description:this_is_a_description")
        .contains_string("path:this/is/a/path");

    utils::run_failure("tpy new Test4");
}
