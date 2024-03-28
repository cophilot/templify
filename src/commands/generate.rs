use crate::log;
use crate::types::argument::Argument;
use crate::types::command::Command;
use crate::types::flag::Flag;
use crate::types::status::Status;
use crate::{types, utils};
use std::io::Write;

/// The definition of the generate command.
pub(crate) fn definition() -> Command {
    let mut generate_command = Command::new(
        vec!["generate".to_string(), "g".to_string()],
        generate,
        "Create a new file from the given template.".to_string(),
    );

    generate_command.add_argument(Argument::new(
        "template-name".to_string(),
        0,
        true,
        "The name of the template to use.".to_string(),
    ));

    generate_command.add_argument(Argument::new(
        "new-name".to_string(),
        1,
        true,
        "The name of the new file.".to_string(),
    ));

    generate_command.add_flag(Flag::new_value_flag(
        vec!["var".to_string(), "v".to_string()],
        "".to_string(),
        "Provide values for the variables in the template, seperated by commas (e.g. -var var1=foo,var2=bar).".to_string(),
    ));

    generate_command.add_flag(Flag::new_bool_flag(
        vec!["default-var".to_string(), "D".to_string()],
        "If enabled use the default value for all variables.".to_string(),
    ));

    generate_command.add_flag(Flag::new_bool_flag(
        vec!["reload".to_string(), "rl".to_string()],
        "If enabled the template will be reloaded before generating.".to_string(),
    ));

    generate_command.add_flag(Flag::new_bool_flag(
        vec!["dry-run".to_string(), "dr".to_string()],
        "If enabled the file will not be created and the output will be printed.".to_string(),
    ));

    generate_command.add_flag(Flag::new_bool_flag(
        vec!["force".to_string(), "f".to_string()],
        "If enabled files will be overwritten if they already exist.".to_string(),
    ));

    generate_command.add_flag(Flag::new_bool_flag(
        vec!["strict".to_string()],
        "If enabled the template name must match exactly.".to_string(),
    ));

    generate_command
}

/// The generate command is used to create a new file from a given template.
pub(crate) fn generate(command: &Command) -> Status {
    let st = utils::functions::check_if_templify_initialized();
    if !st.is_ok {
        return st;
    }

    let strict = command.get_bool_flag("strict");
    let dry_run = command.get_bool_flag("dry-run");
    let force = command.get_bool_flag("force");

    let mut template_name = command.get_argument("template-name").value.clone();
    let given_name = command.get_argument("new-name").value.clone();

    let st = utils::template_handler::parse_template_name(&mut template_name, strict);
    if !st.is_ok {
        return st;
    }

    // reload template if flag is set
    if command.get_bool_flag("reload") {
        utils::template_handler::reload_template(template_name.clone(), strict, false);
    }

    let mut meta = types::template_meta::TemplateMeta::parse(template_name.clone().to_string());
    let mut manual_vars = vec![];
    let st = meta
        .var_placeholder_collection
        .parse_from_input_string(command.get_value_flag("var").clone(), &mut manual_vars);

    if !st.is_ok {
        return st;
    }

    let use_default_vars = command.get_bool_flag("default-var");

    if use_default_vars {
        log!("Using default values for all variables.");
    } else if !meta
        .var_placeholder_collection
        .get_all_placeholders()
        .is_empty()
    {
        log!("Please provide a value for the following variable placeholders:");
    }

    for placeholder in meta.var_placeholder_collection.get_all_placeholders() {
        if manual_vars.contains(&placeholder.name) {
            continue;
        }

        if use_default_vars {
            if placeholder.is_set {
                continue;
            }

            if placeholder.has_options() {
                placeholder.set_value(placeholder.options[0].clone());
            } else {
                placeholder.set_value("unknown".to_string());
            }
            continue;
        }

        let mut input = String::new();

        if placeholder.has_options() {
            loop {
                log!(" {} ", placeholder);
                for i in 0..placeholder.options.len() {
                    log!("  [{}] {}", i + 1, placeholder.options[i]);
                }
                print!("  > ");

                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();

                // break if input is a number and in range
                if input.parse::<usize>().is_ok()
                    && input.parse::<usize>().unwrap() <= placeholder.options.len()
                {
                    break;
                } else {
                    log!(" Invalid input: {}", input);
                    log!(" Please try again.");
                    input = String::new();
                }
            }
            log!(" ");
            placeholder.set_value(placeholder.options[input.parse::<usize>().unwrap() - 1].clone());
            continue;
        }

        print!(" {}: ", placeholder);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if !input.is_empty() {
            placeholder.set_value(input);
        }
    }
    let st = meta.var_placeholder_collection.are_all_set();
    if !st.is_ok {
        return st;
    }

    log!(
        "Generating new files from template {}...",
        template_name.clone()
    );

    let mut new_path = meta.get_path();
    new_path = utils::formater::handle_placeholders(&new_path, &given_name, meta.clone());

    // create dir and all subdirs if they don't exist
    if !dry_run {
        std::fs::create_dir_all(&new_path).unwrap();
    }

    if utils::template_handler::generate_template_dir(
        &format!(".templates/{}", template_name),
        &new_path,
        given_name.as_str(),
        dry_run,
        meta.clone(),
        force,
    ) {
        log!("Files generated successfully.");
        Status::ok()
    } else {
        Status::error("Files could not be generated.".to_string())
    }
}
