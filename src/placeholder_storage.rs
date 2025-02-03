use crate::{
    types::placeholder_definition::PlaceholderDefinition, utils::functions::get_git_name,
    utils::functions::get_month_string,
};

use chrono::Datelike;

/// Get all placeholders.
pub(crate) fn get_all_placeholders() -> Vec<PlaceholderDefinition> {
    vec![
        PlaceholderDefinition {
            name: "year".to_string(),
            description: "The current year".to_string(),
            get_value: || chrono::Local::now().year().to_string(),
        },
        PlaceholderDefinition {
            name: "month".to_string(),
            description: "The current month".to_string(),
            get_value: || chrono::Local::now().month().to_string(),
        },
        PlaceholderDefinition {
            name: "month-name".to_string(),
            description: "The current month as name".to_string(),
            get_value: get_month_string,
        },
        PlaceholderDefinition {
            name: "day".to_string(),
            description: "The current day".to_string(),
            get_value: || chrono::Local::now().day().to_string(),
        },
        PlaceholderDefinition {
            name: "git-name".to_string(),
            description: "The name of the git user".to_string(),
            get_value: get_git_name,
        },
    ]
}
