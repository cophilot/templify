use crate::{
    types::global_flag::GlobalFlag,
    utils::functions::{handle_dev_mode, handle_log_file, handle_quiet_mode},
};

/// Get all global flags.
pub(crate) fn get_all_global_flags() -> Vec<GlobalFlag> {
    vec![
        GlobalFlag::new_bool_flag(
            vec!["dev".to_string(), "d".to_string()],
            handle_dev_mode,
            "If enabled, the command will run in development mode.".to_string(),
        ),
        GlobalFlag::new_bool_flag(
            vec!["quiet".to_string(), "q".to_string()],
            handle_quiet_mode,
            "If enabled, the command will run in quiet mode.".to_string(),
        ),
        GlobalFlag::new_value_flag(
            vec!["log-file".to_string(), "lf".to_string()],
            handle_log_file,
            "Writes the log output to the specified file.".to_string(),
        ),
    ]
}
