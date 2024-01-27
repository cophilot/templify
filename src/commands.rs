pub fn init() {
    println!("Initializing templify...");
    // check if .templates folder exists
    if std::path::Path::new(".templates").exists() {
        println!("Templify is already initialized in this project.");
        return;
    }
    std::fs::create_dir(".templates").unwrap();
    std::fs::write(".templates/README.md", crate::data::get_readme_content()).unwrap();
}

pub fn help() {
    println!("help...");
}
