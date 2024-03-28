include!("../common/utils.rs");
include!("../common/fs.rs");
include!("../common/log.rs");

pub fn test() {
    utils::init_tpy();

    utils::run_successfully("tpy new Test1");
    std::fs::write(".templates/Test1/.templify", "").unwrap();

    utils::run_successfully("tpy new Test2");
    std::fs::write(
        ".templates/Test2/.templify",
        "description:test2_description",
    )
    .unwrap();

    utils::run_successfully("tpy new Test3 -path test3/path");

    utils::run_successfully("tpy new Test4 -description test4_description -path test4/path");

    log::clear();
    utils::run_successfully("tpy list");
    log::contains_line("Test1");

    log::clear();
    utils::run_successfully("tpy list -name");
    log::contains_line("Test1");

    log::clear();
    utils::run_successfully("tpy list -path");
    log::contains_line("Test1");

    log::clear();
    utils::run_successfully("tpy list");
    log::contains_line("Test2 - test2_description");

    log::clear();
    utils::run_successfully("tpy list -name");
    log::contains_line("Test2");

    log::clear();
    utils::run_successfully("tpy list -path");
    log::contains_line("Test2");

    log::clear();
    utils::run_successfully("tpy list");
    log::contains_line("Test3");

    log::clear();
    utils::run_successfully("tpy list -name");
    log::contains_line("Test3");

    log::clear();
    utils::run_successfully("tpy list -path");
    log::contains_line("Test3 [test3/path]");

    log::clear();
    utils::run_successfully("tpy list");
    log::contains_line("Test4 - test4_description");

    log::clear();
    utils::run_successfully("tpy list -name");
    log::contains_line("Test4");

    log::clear();
    utils::run_successfully("tpy list -path");
    log::contains_line("Test4 - test4_description [test4/path]");
}
