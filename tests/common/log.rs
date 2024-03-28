#[allow(dead_code)]
pub mod log {

    pub fn init() {
        let log_file = "tpy-test.log";
        // Create a new file if it does not exist
        if !std::path::Path::new(log_file).exists() {
            std::fs::File::create(log_file).unwrap();
        }

        // save the path to the file
        let temp_path = std::fs::canonicalize(log_file).unwrap();
        set_log_file(temp_path.to_str().unwrap());

        templify::logger::add_logger_entity_fn("test-logger".to_string(), write_log, write_error);
        templify::logger::use_stdout();
    }

    pub fn clear() {
        // Clear the log file
        let log_file = get_log_file();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(false)
            .open(log_file.clone())
            .unwrap();

        std::io::Write::write_all(&mut file, "".as_bytes()).unwrap();
    }

    pub fn clean_up() {
        let log_file = get_log_file();
        std::fs::remove_file(log_file).unwrap();
    }

    pub fn write_log(message: &str) {
        let log_file = get_log_file();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(log_file)
            .unwrap();

        std::io::Write::write_all(&mut file, message.as_bytes()).unwrap();
        std::io::Write::write_all(&mut file, "\n".as_bytes()).unwrap();
    }

    pub fn write_error(message: &str) {
        let error_message = format!("ERROR: {}", message);
        write_log(&error_message);
    }

    pub fn contains_line(line: &str) {
        let log_file = get_log_file();

        let transformed_line = transform_string(line);
        let file = std::fs::read_to_string(log_file).unwrap();
        for l in file.lines() {
            if transform_string(l) == transformed_line {
                return;
            }
        }

        assert!(false, "🚨 Log file does not contain line: {}", line);
    }

    pub fn contains_string(s: &str) {
        let log_file = get_log_file();
        let file = std::fs::read_to_string(log_file).unwrap();
        let transformed_line = transform_string(s);
        let transformed_file = transform_string(&file);
        let result = transformed_file.contains(&transformed_line);
        assert!(result, "🚨 Log file does not contain string: {}", s);
    }

    fn transform_string(s: &str) -> String {
        let mut result = s.to_string();
        result.retain(|c| !c.is_whitespace() && c != '\n');
        result = result.to_lowercase();
        result
    }

    fn set_log_file(file: &str) {
        unsafe { crate::LOG_FILE = String::from(file) };
    }

    fn get_log_file() -> String {
        unsafe { crate::LOG_FILE.clone() }
    }
}
