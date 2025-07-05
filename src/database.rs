use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::args::Commands;
use crate::models::Task;

pub const CREATE_TASK_TABLE: &str = "CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0,
    birth TEXT NOT NULL
)";

pub const INSERT_TASK: &str = "INSERT INTO tasks (description, done, birth) VALUES (?1, 0, ?2)";
pub const SELECT_ALL_TASKS: &str = "SELECT id, description, done, birth FROM tasks";
pub const DELETE_TASK: &str = "DELETE FROM tasks WHERE id = ?1";
pub const UPDATE_TASK_DONE: &str = "UPDATE tasks SET done = 1 WHERE id = ?1 AND done = 0";

pub fn get_db_path() -> Result<PathBuf> {
    use crate::config;
    Ok(config::get_app_config_dir()?.join("tasks.db"))
}

pub fn check_db_exists(db_path: &Path) -> bool {
    db_path.exists()
}

pub fn create_database(db_path: &Path) -> Result<()> {
    File::create(db_path)
        .with_context(|| format!("Failed to create database at {}", db_path.display()))?;
    Ok(())
}

pub fn handle_db_operations(db_path: &Path, command: Commands) -> Result<()> {
    // Connect to database
    let conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open database at {}", db_path.display()))?;

    // Create table if it doesn't exist
    Task::create_default(&conn)
        .context("Failed to create tasks table")?;

    // Execute the command using associated functions
    match command {
        Commands::Add { description } => {
            let id = Task::add(&conn, description)
                .context("Failed to add task")?;
            println!("Task added successfully with id: {}", id);
        },
        Commands::List => {
            let tasks = Task::list(&conn)
                .context("Failed to list tasks")?;
            
            if tasks.is_empty() {
                println!("No tasks found");
            } else {
                println!(
                    "{:<8} | {:<8} | {:<19} | {}",
                    "ID", "DONE", "BIRTH", "DESCRIPTION"
                );
                println!("{}", "-".repeat(60));
                for task in tasks {
                    let done_display = if task.done { "true" } else { "false" };
                    println!(
                        "{:<8} | {:<8} | {:<19} | {}",
                        task.id, done_display, task.birth, task.description
                    );
                }
            }
        },
        Commands::Remove { id } => {
            let removed = Task::remove(&conn, id)
                .context("Failed to remove task")?;
            
            if removed {
                println!("Task {} removed!", id);
            } else {
                println!("No task found with id: {}", id);
            }
        }
        Commands::Done { id } => {
            let updated = Task::mark_done(&conn, id)
                .context("Failed to mark task as done")?;
            
            if updated {
                println!("Task {} marked as done!", id);
            } else {
                println!("Task {} already completed or doesn't exist.", id);
            }
        },
    }
    
    Ok(())
}