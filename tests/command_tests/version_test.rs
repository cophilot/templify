include!("../common/utils.rs");
include!("../common/log.rs");

pub fn test() {
    utils::run_successfully("tpy version");

    log::contains_string(env!("CARGO_PKG_VERSION"));
}
