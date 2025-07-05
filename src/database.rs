//! Database operations and SQL query management module.
//!
//! This module contains all database-related functionality including:
//! - Database path resolution
//! - Database creation and validation
//! - SQL query constants
//! - Command execution orchestration
//!
//! The module acts as a bridge between the CLI commands and the database
//! operations, handling all the necessary setup and error handling.

use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::args::Commands;
use crate::models::Task;

/// SQL query to create the tasks table.
///
/// This query creates the main tasks table with the following schema:
/// - `id`: Primary key, auto-incrementing integer
/// - `description`: Task description, cannot be null
/// - `done`: Boolean completion status, defaults to false
/// - `birth`: Creation timestamp as text
pub const CREATE_TASK_TABLE: &str = "CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0,
    birth TEXT NOT NULL
)";

/// SQL query to insert a new task.
///
/// Parameters:
/// 1. `description` - The task description
/// 2. `birth` - The creation timestamp
pub const INSERT_TASK: &str = "INSERT INTO tasks (description, done, birth) VALUES (?1, 0, ?2)";

/// SQL query to select all tasks.
///
/// Returns all columns for all tasks in the database.
pub const SELECT_ALL_TASKS: &str = "SELECT id, description, done, birth FROM tasks";

/// SQL query to delete a task by ID.
///
/// Parameters:
/// 1. `id` - The task ID to delete
pub const DELETE_TASK: &str = "DELETE FROM tasks WHERE id = ?1";

/// SQL query to mark a task as done.
///
/// Only updates tasks that are not already completed.
/// Parameters:
/// 1. `id` - The task ID to mark as done
pub const UPDATE_TASK_DONE: &str = "UPDATE tasks SET done = 1 WHERE id = ?1 AND done = 0";

/// Get the full path to the database file.
///
/// This function combines the application config directory with the
/// database filename to create the full path where the SQLite database
/// should be stored.
///
/// # Returns
///
/// Returns a `PathBuf` pointing to the database file location.
///
/// # Errors
///
/// Returns an error if the config directory cannot be determined.
///
/// # Examples
///
/// ```
/// let db_path = get_db_path()?;
/// println!("Database will be stored at: {}", db_path.display());
/// ```
pub fn get_db_path() -> Result<PathBuf> {
    use crate::config;
    Ok(config::get_app_config_dir()?.join("tasks.db"))
}

/// Check if the database file exists.
///
/// This is a simple wrapper around `Path::exists()` to check if the
/// database file has been created.
///
/// # Arguments
///
/// * `db_path` - Path to the database file
///
/// # Returns
///
/// Returns `true` if the database file exists, `false` otherwise.
///
/// # Examples
///
/// ```
/// let db_path = get_db_path()?;
/// if check_db_exists(&db_path) {
///     println!("Database exists");
/// } else {
///     println!("Database needs to be created");
/// }
/// ```
pub fn check_db_exists(db_path: &Path) -> bool {
    db_path.exists()
}

/// Create a new database file.
///
/// This function creates an empty SQLite database file at the specified path.
/// The actual table structure is created later when the database is first opened.
///
/// # Arguments
///
/// * `db_path` - Path where the database file should be created
///
/// # Returns
///
/// Returns `Ok(())` if the database file was created successfully.
///
/// # Errors
///
/// Returns an error if file creation fails due to permissions or other IO issues.
///
/// # Examples
///
/// ```
/// let db_path = get_db_path()?;
/// create_database(&db_path)?;
/// println!("Database created at: {}", db_path.display());
/// ```
pub fn create_database(db_path: &Path) -> Result<()> {
    File::create(db_path)
        .with_context(|| format!("Failed to create database at {}", db_path.display()))?;
    Ok(())
}

/// Handle database operations based on the provided command.
///
/// This is the main orchestration function that:
/// 1. Opens a database connection
/// 2. Ensures the tasks table exists
/// 3. Executes the appropriate command
/// 4. Handles all output and error reporting
///
/// # Arguments
///
/// * `db_path` - Path to the database file
/// * `command` - The command to execute (Add, List, Remove, or Done)
///
/// # Returns
///
/// Returns `Ok(())` if the operation completed successfully.
///
/// # Errors
///
/// Returns an error if:
/// - Database connection fails
/// - Table creation fails
/// - The specific command operation fails
///
/// # Examples
///
/// ```
/// let db_path = get_db_path()?;
/// let command = Commands::Add { description: "Test task".to_string() };
/// handle_db_operations(&db_path, command)?;
/// ```
pub fn handle_db_operations(db_path: &Path, command: Commands) -> Result<()> {
    let conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open database at {}", db_path.display()))?;

    Task::create_default(&conn).context("Failed to create tasks table")?;

    match command {
        Commands::Add { description } => {
            let id = Task::add(&conn, description).context("Failed to add task")?;
            println!("Task added successfully with id: {}", id);
        }
        Commands::List => {
            let tasks = Task::list(&conn).context("Failed to list tasks")?;

            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                // Print header
                println!(
                    "{:<8} | {:<8} | {:<19} | DESCRIPTION",
                    "ID", "DONE", "BIRTH"
                );
                println!("{}", "-".repeat(60));

                // Print each task
                for task in tasks {
                    let done_display = if task.done { "true" } else { "false" };
                    println!(
                        "{:<8} | {:<8} | {:<19} | {}",
                        task.id, done_display, task.birth, task.description
                    );
                }
            }
        }
        Commands::Remove { id } => {
            let removed = Task::remove(&conn, id).context("Failed to remove task")?;

            if removed {
                println!("Task {} removed!", id);
            } else {
                println!("No task found with id: {}", id);
            }
        }
        Commands::Done { id } => {
            let updated = Task::mark_done(&conn, id).context("Failed to mark task as done")?;

            if updated {
                println!("Task {} marked as done!", id);
            } else {
                println!("Task {} already completed or doesn't exist.", id);
            }
        }
    }

    Ok(())
}
