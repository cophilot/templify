use chrono::Datelike;

pub fn handle_placeholders(s: &str, name: &str) -> String {
    let mut s = s.to_string();

    s = s.replace("$$name$$", name);
    s = s.replace("$$name.lower$$", name.to_lowercase().as_str());
    s = s.replace("$$name.upper$$", name.to_uppercase().as_str());

    let tokens = tokenize_string(name);
    s = s.replace("$$name.camel$$", &to_camel_case(tokens.clone()));
    s = s.replace("$$name.snake$$", &to_snake_case(tokens.clone()));
    s = s.replace("$$name.kebab$$", &to_kebab_case(tokens.clone()));
    s = s.replace("$$name.pascal$$", &to_pascal_case(tokens.clone()));
    s = s.replace("$$name.macro$$", &to_macro_case(tokens.clone()));
    s = s.replace("$$name.train$$", &to_train_case(tokens.clone()));

    s = s.replace("$$year$$", chrono::Local::now().year().to_string().as_str());
    s = s.replace("$$month$$", &chrono::Local::now().month().to_string());
    let month_str = match chrono::Local::now().month() {
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
    };
    s = s.replace("$$month-name$$", month_str);
    s = s.replace("$$day$$", &chrono::Local::now().day().to_string());
    s = s.replace("$$git-name$$", &crate::utils::get_git_name());
    return s;
}

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
    return result;
}

fn to_macro_case(tokens: Vec<String>) -> String {
    return tokens.join("_").to_uppercase();
}

fn to_pascal_case(tokens: Vec<String>) -> String {
    let mut result = String::new();
    for token in tokens {
        let newtoken = format!("{}{}", &token[0..1].to_uppercase(), &token[1..]);
        result.push_str(&newtoken);
    }
    return result;
}

fn to_kebab_case(tokens: Vec<String>) -> String {
    return tokens.join("-");
}

fn to_snake_case(tokens: Vec<String>) -> String {
    return tokens.join("_");
}

fn to_camel_case(tokens: Vec<String>) -> String {
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i == 0 {
            result.push_str(&token);
        } else {
            let newtoken = format!("{}{}", &token[0..1].to_uppercase(), &token[1..]);
            result.push_str(&newtoken);
        }
    }
    return result;
}

fn tokenize_string(input: &str) -> Vec<String> {
    let mut result = input.replace('-', " ").replace('_', " ").to_string();
    result = result.replace("_", " ");

    // replace all uppercase letters with lowercase and a space before
    result = result
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                format!(" {}", c.to_lowercase().to_string())
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
