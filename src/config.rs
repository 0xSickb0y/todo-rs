use std::env;
use std::fs;
use std::process::exit;

pub fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|e| {
        eprintln!("Failed to get user home directory.\nReason: {}", e);
        exit(1);
    })
}

pub fn create_config_path(home_directory: &str) {
    let config_path = format!("{home_directory}/.config/");

    let metadata = fs::metadata(&config_path).unwrap_or_else(|e| {
        eprintln!("Failed to get metadata for ~/.config/\nReason: {}", e);
        exit(1);
    });

    if metadata.permissions().readonly() {
        eprintln!("~/.config/ is set to READ_ONLY, exiting...");
        exit(1);
    }

    let cli_todo_path = format!("{config_path}todo-rs");
    fs::create_dir_all(&cli_todo_path).unwrap_or_else(|e| {
        eprintln!(
            "Failed to create directory {}\nReason: {}",
            cli_todo_path, e
        );
        exit(1);
    });
}
