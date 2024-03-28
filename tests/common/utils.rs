#[allow(dead_code)]
pub mod utils {
    use templify::executer::execute;

    pub fn run_successfully(line: &str) {
        assert!(run(line, true));
    }

    pub fn run_failure(line: &str) {
        assert!(!run(line, false));
    }

    pub fn reset_dir() {
        cleanup_dir();
        init_dir();
    }

    pub fn init_tpy() {
        run("tpy init -b", true);
    }

    fn run(line: &str, success: bool) -> bool {
        println!("");
        if success {
            println!(" >>> {}", line);
        } else {
            println!(" !>> {}", line);
        }

        let mut args: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if args[0] == "tpy" {
            let _ = args.remove(0);
        }
        execute(args)
    }

    pub fn setup() {
        init_dir();
    }

    pub fn teardown() {
        cleanup_dir();
    }

    fn init_dir() {
        let current_dir = std::env::current_dir().unwrap();
        if current_dir.ends_with(".templify-test-dir") {
            return;
        }
        let new_dir = current_dir.join(".templify-test-dir");
        if !new_dir.exists() {
            std::fs::create_dir(&new_dir).unwrap();
        }
        std::env::set_current_dir(&new_dir).unwrap();
    }

    fn cleanup_dir() {
        let current_dir = std::env::current_dir().unwrap();
        if current_dir.ends_with(".templify-test-dir") {
            std::env::set_current_dir("..").unwrap();
            std::fs::remove_dir_all(&current_dir).unwrap();
        }
    }
}
