use crate::{logger, types::status::Status};
use chrono::Datelike;
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::{io::Write, path::Path};

/// Check if templify is initialized in the current project
pub(crate) fn check_if_templify_initialized() -> Status {
    if !Path::new(".templates").exists() {
        let command_name = unsafe { crate::env::BASE_COMMAND_NAME.clone() };
        return Status::error(format!("templify is not initialized in this project.\nRun `{} init` to initialize templify in your project.",command_name));
    }
    Status::ok()
}

/// Check if a internet connection is available
pub fn check_internet_connection() -> bool {
    let response = reqwest::blocking::get("https://google.com");
    if response.is_err() {
        return false;
    }
    let response = response.unwrap();
    if response.status().is_success() {
        return true;
    }
    false
}

/// Get the git user name from the git configuration
pub fn get_git_name() -> String {
    let output = std::process::Command::new("git")
        .arg("config")
        .arg("user.name")
        .output();
    if output.is_err() {
        return "unknown".to_string();
    }
    let output = output.unwrap();
    let output = String::from_utf8_lossy(&output.stdout);
    let mut name = output.trim().to_string();
    if name.is_empty() {
        name = "unknown".to_string();
    }
    name
}

/// Returns the current month as a string.
pub fn get_month_string() -> String {
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

/// Prepare the dev mode
pub(crate) fn handle_dev_mode() -> Status {
    if !Path::new("dev").exists() {
        std::fs::create_dir("dev").unwrap();
    }
    std::env::set_current_dir("dev").unwrap();

    Status::ok()
}

/// Prepare the quiet mode
pub(crate) fn handle_quiet_mode() -> Status {
    logger::remove_logger_entity("stdout");
    Status::ok()
}

/// Save the logs to a file
pub(crate) fn handle_log_file(file: String) -> Status {
    if !Path::new(&file).exists() {
        let file = std::fs::File::create(&file);
        if file.is_err() {
            return Status::error(format!(
                "Failed to create the log file: {}",
                file.err().unwrap()
            ));
        }
    }

    let file_path = file.clone();

    let file_logger: Box<dyn Fn(&str)> = Box::new(move |message: &str| {
        let file = std::fs::OpenOptions::new().append(true).open(&file_path);
        if file.is_err() {
            return;
        }
        let mut file = file.unwrap();
        // get timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let message = format!("[{}] {}\n", timestamp, message);

        file.write_all(message.as_bytes()).unwrap();
    });

    let file_path = file.clone();

    let file_logger_error: Box<dyn Fn(&str)> = Box::new(move |message: &str| {
        let file = std::fs::OpenOptions::new().append(true).open(&file_path);
        if file.is_err() {
            return;
        }
        let mut file = file.unwrap();
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let message = format!("ERROR[{}] {}\n", timestamp, message);
        file.write_all(message.as_bytes()).unwrap();
    });

    logger::add_logger_entity_closure("file-log".to_string(), file_logger, file_logger_error);
    Status::ok()
}

/// Execute Command
pub fn execute_user_command(command: String) -> Result<String, std::io::Error> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd.exe", "/C")
    } else {
        ("sh", "-c")
    };

    let output = Command::new(shell).arg(flag).arg(command).output()?;

    if !output.status.success() {
        let exit_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(
            ErrorKind::Other,
            format!("Command failed with exit code {}: {}", exit_code, stderr),
        ));
    }

    String::from_utf8(output.stdout).map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
}
