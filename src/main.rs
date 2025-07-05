use std::process::exit;
use anyhow::Result;

mod args;
mod config;
mod database;
mod io_utils;
mod models;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

fn run() -> Result<()> {
    let command = args::parse_args();

    // Ensure config directory exists and is writable
    config::ensure_config_dir()?;
    config::check_config_dir_writable()?;

    let db_path = database::get_db_path()?;
    
    if !database::check_db_exists(&db_path) {
        println!("Database not found at {}", db_path.display());

        if io_utils::ask_user_confirmation("Do you want to create it? (Y/N): ") {
            database::create_database(&db_path)?;
            println!("Database created at {}", db_path.display());
        } else {
            println!("Goodbye!");
            return Ok(());
        }
    }

    database::handle_db_operations(&db_path, command)?;
    Ok(())
}