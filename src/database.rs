use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::exit;

use chrono::{Local, NaiveDateTime};
use rusqlite::{Connection, Result};

use crate::args::Commands;

pub fn get_db_path(home_directory: &str) -> PathBuf {
    PathBuf::from(format!(
        "{home_directory}/.config/todo-rs/tasks.db"
    ))
}

pub fn check_db_exists(db_path: &Path) -> bool {
    db_path.exists()
}

pub fn create_database(db_path: &Path) {
    File::create(db_path).unwrap_or_else(|e| {
        eprintln!("Failed to create database.\nReason: {}", e);
        exit(1);
    });
}

pub fn handle_db_operations(db_path: &Path, command: Commands) {
    // Connect to database
    let conn = Connection::open(db_path).unwrap_or_else(|e| {
        eprintln!("Failed to open database: {}", e);
        exit(1);
    });

    if let Err(e) = Task::create_default(&conn) {
        eprintln!("Failed to create tasks table\nReason: {}", e);
        exit(1);
    }

    // Execute the command using associated functions
    match command {
        Commands::Add { description } => match Task::add(&conn, description) {
            Ok(id) => println!("Task added successfully with id: {}", id),
            Err(e) => {
                eprintln!("Failed to add task\nReason: {}", e);
                exit(1);
            }
        },
        Commands::List => match Task::list(&conn) {
            Ok(tasks) => {
                if tasks.is_empty() {
                    println!("No tasks found")
                } else {
                    println!(
                        "{:<8} | {:<8} | {:<19} | {}",
                        "ID", "DONE", "BIRTH", "DESCRIPTION"
                    );
                    println!("{}", "-".repeat(60));
                    for task in tasks {
                        println!(
                            "{:<8} | {:<8} | {:<19} | {}",
                            task.id, task.done, task.birth, task.description
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to list tasks\nReason: {}", e);
                exit(1);
            }
        },
        Commands::Remove { id } => {
            if let Err(e) = Task::remove(&conn, id) {
                eprintln!("Failed to remove task\nReason: {}", e);
                exit(1)
            } else {
                println!("Task {} removed!", id)
            }
        }
        Commands::Done { id } => match Task::mark_done(&conn, id) {
            Ok(updated) => {
                if updated {
                    println!("Task {} marked as done!", id);
                } else {
                    println!("Task {} already completed or doesn't exist.", id);
                }
            }
            Err(e) => {
                eprintln!("Failed to mark task as 'done'\nReason: {}", e);
                exit(1);
            }
        },
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub done: bool,
    pub birth: NaiveDateTime,
}

impl Task {
    pub fn create_default(conn: &Connection) -> Result<()> {
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT 0,
                birth TEXT NOT NULL
            )",
            [],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn add(conn: &Connection, description: String) -> Result<i64> {
        let now = Local::now().naive_local();
        let birth_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

        conn.execute(
            "INSERT INTO tasks (description, done, birth) VALUES (?1, 0, ?2)",
            (&description, &birth_str),
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare("SELECT id, description, done, birth FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            let date_str: String = row.get(3)?;
            let parsed = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
                .expect("Corrupted birth timestamp");

            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                done: row.get(2)?,
                birth: parsed,
            })
        })?;

        Ok(task_iter.filter_map(Result::ok).collect())
    }

    pub fn remove(conn: &Connection, id: i64) -> Result<()> {
        let rows_affected = conn.execute("DELETE FROM tasks WHERE id = ?1", [&id])?;

        if rows_affected == 0 {
            eprintln!("No task found with id: {}", id);
        }

        Ok(())
    }

    pub fn mark_done(conn: &Connection, id: i64) -> Result<bool> {
        let rows_affected = conn.execute(
            "UPDATE tasks SET done = 1 WHERE id = ?1 and DONE = 0",
            [&id],
        )?;

        Ok(rows_affected > 0)
    }
}
