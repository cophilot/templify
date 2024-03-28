include!("../common/utils.rs");
include!("../common/log.rs");

pub fn test() {
    utils::run_successfully("tpy help");

    log::contains_string("generate");
    log::contains_string("help");
    log::contains_string("init");
    log::contains_string("list");
    log::contains_string("load");
    log::contains_string("new");
    log::contains_string("reload");
    log::contains_string("update");
    log::contains_string("version");
}
