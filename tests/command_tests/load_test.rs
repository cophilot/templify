include!("../common/utils.rs");
include!("../common/fs.rs");
include!("../common/log.rs");

pub fn test() {
    utils::init_tpy();

    utils::run_successfully("tpy load https://github.com/cophilot/templify-vault/tree/main/Test");
    check_test_structure();
    utils::run_failure("tpy load https://github.com/cophilot/templify-vault/tree/main/Test");

    // check -force flag
    utils::run_successfully(
        "tpy load https://github.com/cophilot/templify-vault/tree/main/Test -f",
    );
    check_test_structure();

    utils::reset_dir();
    utils::init_tpy();

    // check -template flag
    utils::run_successfully(
        "tpy load https://github.com/cophilot/templify-vault/tree/main/Test/Test1 -t",
    );
    check_test_1_structure();
    fs::templates_dir().dir("Test2").check_not_exists();

    utils::reset_dir();
    utils::init_tpy();

    utils::run_successfully(
        "tpy load https://github.com/cophilot/templify-vault/tree/main/Test/Test2 -t",
    );
    check_test_2_structure();
    fs::templates_dir().dir("Test1").check_not_exists();
}

pub fn check_test_structure() {
    check_test_1_structure();
    check_test_2_structure();
    check_my_test_structure();
}

pub fn check_test_1_structure() {
    fs::templates_dir()
        .dir("Test1")
        .file("Test1$$name$$.txt")
        .contains_string("$$name$$")
        .contains_string(
            "Paddington loves to eat marmalade sandwiches and he is a very polite bear.",
        );
    fs::templates_dir()
        .dir("Test1")
        .file(".templify")
        .contains_string("description:This is used to test templify")
        .contains_string("path:src")
        .contains_string(".source:https://github.com/cophilot/templify-vault/tree/main/Test/Test1");
}

pub fn check_test_2_structure() {
    let mut base = fs::templates_dir().dir("Test2");

    base.file(".templify")
        .contains_string("description:This is used to test templify")
        .contains_string("path:src")
        .contains_string(".source:https://github.com/cophilot/templify-vault/tree/main/Test/Test2");
    base.file("file.txt")
        .contains_string("A elephant can eat 300 pounds of food in a day.");
    base.dir("subdir")
        .file("file.txt")
        .contains_string("Apollo 11 started its journey to the moon on July 16, 1969.");
    base.dir("subdir")
        .dir("subdir")
        .file(".tpykeep")
        .check_all_exists();
}

pub fn check_my_test_structure() {
    fs::templates_dir()
        .dir("MyTest")
        .file("file.txt")
        .contains_string("Nebraska has the largest indoor rainforest in the world.");
    fs::templates_dir()
        .dir("MyTest")
        .file(".templify")
        .contains_string("description:This is used to test templify")
        .contains_string("path:src")
        .contains_string(
            ".source:https://github.com/cophilot/templify-vault/tree/main/Test/MyTest",
        );
}
