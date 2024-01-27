pub fn init() {
    println!("Initializing templify...");
    // check if .templates folder exists
    if std::path::Path::new(".templates").exists() {
        println!("Templify is already initialized in this project.");
        return;
    }
    std::fs::create_dir(".templates").unwrap();
    std::fs::write(
        ".templates/README.md",
        crate::data::get_init_readme_content(),
    )
    .unwrap();
    std::fs::create_dir(".templates/ExampleTemplate").unwrap();
    std::fs::write(
        ".templates/ExampleTemplate/.templify",
        crate::data::get_init_example_templify_content(),
    )
    .unwrap();
    std::fs::write(
        ".templates/ExampleTemplate/index.html",
        crate::data::get_init_example_index_content(),
    )
    .unwrap();
}

pub fn help() {
    println!("templify help");
    println!("");
    println!("Usage: tmy <command>");
    println!("");
    println!("Commands:");
    println!("  [ help | h ]              Show this help message");
    println!("  [ init | i ]              Initialize Templify in your project");
    println!("  [ new | n ] <name>        Create a new template with the given name");
    println!("  [ list | l ]              List all templates");
    println!("  [ generate | g ] <name>   Generate a new file from the given template");
}
