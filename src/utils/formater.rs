use crate::types::template_meta::TemplateMeta;
use chrono::Datelike;

/// This function is used to handle the placeholders in a template string.
pub(crate) fn handle_placeholders(s: &str, name: &str, meta: TemplateMeta) -> String {
    let mut s = s.to_string();

    s = s.replace("$$name$$", name);
    s = handle_case_conversion("name", name, s.as_str());

    s = s.replace("$$year$$", chrono::Local::now().year().to_string().as_str());
    s = s.replace("$$month$$", &chrono::Local::now().month().to_string());
    s = s.replace("$$month-name$$", get_month_string().as_str());
    s = s.replace("$$day$$", &chrono::Local::now().day().to_string());
    s = s.replace("$$git-name$$", &crate::utils::functions::get_git_name());
    s = handle_variable_placeholders(s.as_str(), meta);

    s
}

/// This function is used to handle the variable placeholders in a template string.
fn handle_variable_placeholders(s: &str, meta: TemplateMeta) -> String {
    let mut s = s.to_string();
    for (_, p) in meta.var_placeholder_collection.placeholders {
        s = s.replace(format!("$${}$$", p.name).as_str(), p.value.as_str());
        s = handle_case_conversion(p.name.as_str(), p.value.as_str(), s.as_str());
    }
    s
}

/// This function is used to handle the case conversion for placeholders.
fn handle_case_conversion(placeholder_name: &str, value: &str, s: &str) -> String {
    let mut s = s.to_string();
    let tokens = tokenize_string(value);

    let lower = to_total_lower_case(tokens.clone());
    s = s.replace(format!("$${}.lower$$", placeholder_name).as_str(), &lower);
    s = s.replace(format!("$${}.l$$", placeholder_name).as_str(), &lower);

    let upper = to_total_upper_case(tokens.clone());
    s = s.replace(format!("$${}.upper$$", placeholder_name).as_str(), &upper);
    s = s.replace(format!("$${}.u$$", placeholder_name).as_str(), &upper);

    let camel = to_camel_case(tokens.clone());
    s = s.replace(format!("$${}.camel$$", placeholder_name).as_str(), &camel);
    s = s.replace(format!("$${}.c$$", placeholder_name).as_str(), &camel);

    let snake = to_snake_case(tokens.clone());
    s = s.replace(format!("$${}.snake$$", placeholder_name).as_str(), &snake);
    s = s.replace(format!("$${}.s$$", placeholder_name).as_str(), &snake);

    let kebab = to_kebab_case(tokens.clone());
    s = s.replace(format!("$${}.kebab$$", placeholder_name).as_str(), &kebab);
    s = s.replace(format!("$${}.k$$", placeholder_name).as_str(), &kebab);

    let pascal = to_pascal_case(tokens.clone());
    s = s.replace(format!("$${}.pascal$$", placeholder_name).as_str(), &pascal);
    s = s.replace(format!("$${}.p$$", placeholder_name).as_str(), &pascal);

    let macro_s = to_macro_case(tokens.clone());
    s = s.replace(format!("$${}.macro$$", placeholder_name).as_str(), &macro_s);
    s = s.replace(format!("$${}.m$$", placeholder_name).as_str(), &macro_s);

    let train = to_train_case(tokens.clone());
    s = s.replace(format!("$${}.train$$", placeholder_name).as_str(), &train);
    s = s.replace(format!("$${}.t$$", placeholder_name).as_str(), &train);

    s
}

/// Transforms the given tokens to a lower case string.
fn to_total_lower_case(tokens: Vec<String>) -> String {
    tokens.join("").to_lowercase()
}

/// Transforms the given tokens to an upper case string.
fn to_total_upper_case(tokens: Vec<String>) -> String {
    tokens.join("").to_uppercase()
}

/// Transforms the given tokens to a train case string.
fn to_train_case(tokens: Vec<String>) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        let newtoken = format!("{}{}", &token[0..1].to_uppercase(), &token[1..]);
        if i == 0 {
            result.push_str(&newtoken);
        } else {
            result.push_str(&format!("-{}", newtoken));
        }
    }
    result
}

/// Transforms the given tokens to a macro case string.
fn to_macro_case(tokens: Vec<String>) -> String {
    tokens.join("_").to_uppercase()
}

/// Transforms the given tokens to a pascal case string.
fn to_pascal_case(tokens: Vec<String>) -> String {
    let mut result = String::new();
    for token in tokens {
        let newtoken = format!("{}{}", &token[0..1].to_uppercase(), &token[1..]);
        result.push_str(&newtoken);
    }
    result
}

/// Transforms the given tokens to a kebab case string.
fn to_kebab_case(tokens: Vec<String>) -> String {
    tokens.join("-")
}

/// Transforms the given tokens to a snake case string.
fn to_snake_case(tokens: Vec<String>) -> String {
    tokens.join("_")
}

/// Transforms the given tokens to a camel case string.
fn to_camel_case(tokens: Vec<String>) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i == 0 {
            result.push_str(token);
        } else {
            let newtoken = format!("{}{}", &token[0..1].to_uppercase(), &token[1..]);
            result.push_str(&newtoken);
        }
    }
    result
}

/// Parses the given string into tokens.
fn tokenize_string(input: &str) -> Vec<String> {
    let mut result = input.replace(['-', '_'], " ").to_string();
    result = result.replace('_', " ");

    // replace all uppercase letters with lowercase and a space before
    result = result
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                format!(" {}", c.to_lowercase())
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("");
    return result
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
}

/// Returns the current month as a string.
fn get_month_string() -> String {
    let month = chrono::Local::now().month();
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Unknown",
    }
    .to_string()
}
