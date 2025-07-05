//! # todo-rs
//!
//! A simple and efficient command-line TODO application written in Rust.
//!
//! This application provides basic task management functionality including:
//! - Adding new tasks
//! - Listing all tasks with their status
//! - Marking tasks as complete
//! - Removing tasks
//!
//! Tasks are stored in a SQLite database located in the user's config directory,
//! following XDG Base Directory specification when available.

use anyhow::Result;
use std::process::exit;

mod args;
mod config;
mod database;
mod io_utils;
mod models;

/// Main entry point for the todo-rs application.
///
/// This function handles the overall application flow and error handling.
/// If any error occurs during execution, it will be printed to stderr
/// and the program will exit with status code 1.
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

/// Core application logic.
///
/// This function orchestrates the main application flow:
/// 1. Parse command line arguments
/// 2. Ensure config directory exists and is writable  
/// 3. Check if database exists, create if needed with user confirmation
/// 4. Execute the requested database operation
///
/// # Returns
///
/// Returns `Ok(())` on successful execution, or an error if any step fails.
///
/// # Errors
///
/// This function can return errors for:
/// - Invalid command line arguments
/// - Config directory creation/permission issues
/// - Database creation/access problems
/// - Task operation failures
fn run() -> Result<()> {
    let command = args::parse_args();

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
