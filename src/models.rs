//! Data models module.
//!
//! This module defines the core data structures and provides methods for
//! interacting with the SQLite database.

use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use rusqlite::Connection;

/// Represents a task in the todo application.
///
/// Each task has a unique identifier, description, completion status,
/// and creation timestamp. Tasks are stored in a SQLite database and
/// can be manipulated through the associated methods.
///
/// # Fields
///
/// * `id` - Unique identifier for the task (auto-generated)
/// * `description` - Human-readable description of the task
/// * `done` - Boolean indicating if the task is completed
/// * `birth` - Timestamp when the task was created
#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub done: bool,
    pub birth: NaiveDateTime,
}

impl Task {
    /// Create the tasks table in the database if it doesn't exist.
    ///
    /// This method sets up the initial database schema. It's designed to be
    /// idempotent - calling it multiple times won't cause errors.
    ///
    /// # Arguments
    ///
    /// * `conn` - SQLite database connection
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the table was created or already exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the SQL execution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = Connection::open("tasks.db")?;
    /// Task::create_default(&conn)?;
    /// ```
    pub fn create_default(conn: &Connection) -> Result<()> {
        conn.execute(crate::database::CREATE_TASK_TABLE, [])?;
        Ok(())
    }

    /// Add a new task to the database.
    ///
    /// Creates a new task with the given description and the current timestamp.
    /// The task is initially marked as not done.
    ///
    /// # Arguments
    ///
    /// * `conn` - SQLite database connection
    /// * `description` - The task description
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly created task.
    ///
    /// # Errors
    ///
    /// Returns an error if the database insertion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = Connection::open("tasks.db")?;
    /// let task_id = Task::add(&conn, "Buy groceries".to_string())?;
    /// println!("Created task with ID: {}", task_id);
    /// ```
    pub fn add(conn: &Connection, description: String) -> Result<i64> {
        let now = Local::now().naive_local();
        let birth_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

        conn.execute(crate::database::INSERT_TASK, (&description, &birth_str))?;
        Ok(conn.last_insert_rowid())
    }

    /// Retrieve all tasks from the database.
    ///
    /// Fetches all tasks regardless of their completion status and returns
    /// them as a vector of `Task` structs.
    ///
    /// # Arguments
    ///
    /// * `conn` - SQLite database connection
    ///
    /// # Returns
    ///
    /// Returns a vector of all tasks in the database.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The database query fails
    /// - Task data is corrupted (e.g., invalid timestamp format)
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = Connection::open("tasks.db")?;
    /// let tasks = Task::list(&conn)?;
    /// for task in tasks {
    ///     println!("{}: {}", task.id, task.description);
    /// }
    /// ```
    pub fn list(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare(crate::database::SELECT_ALL_TASKS)?;
        let task_iter = stmt.query_map([], |row| {
            let date_str: String = row.get(3)?;
            let parsed =
                NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").map_err(|_e| {
                    rusqlite::Error::InvalidColumnType(
                        3,
                        date_str.clone(),
                        rusqlite::types::Type::Text,
                    )
                })?;

            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                done: row.get(2)?,
                birth: parsed,
            })
        })?;

        Ok(task_iter.filter_map(Result::ok).collect())
    }

    /// Remove a task from the database.
    ///
    /// Deletes the task with the specified ID from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - SQLite database connection
    /// * `id` - The ID of the task to remove
    ///
    /// # Returns
    ///
    /// Returns `true` if a task was deleted, `false` if no task with the given ID exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = Connection::open("tasks.db")?;
    /// let removed = Task::remove(&conn, 1)?;
    /// if removed {
    ///     println!("Task removed successfully");
    /// } else {
    ///     println!("No task found with that ID");
    /// }
    /// ```
    pub fn remove(conn: &Connection, id: i64) -> Result<bool> {
        let rows_affected = conn.execute(crate::database::DELETE_TASK, [&id])?;
        Ok(rows_affected > 0)
    }

    /// Mark a task as completed.
    ///
    /// Updates the task's status to completed (done = true) if it exists
    /// and is not already completed.
    ///
    /// # Arguments
    ///
    /// * `conn` - SQLite database connection
    /// * `id` - The ID of the task to mark as done
    ///
    /// # Returns
    ///
    /// Returns `true` if the task was updated, `false` if no incomplete task
    /// with the given ID exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the database operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = Connection::open("tasks.db")?;
    /// let updated = Task::mark_done(&conn, 1)?;
    /// if updated {
    ///     println!("Task marked as done");
    /// } else {
    ///     println!("Task already completed or doesn't exist");
    /// }
    /// ```
    pub fn mark_done(conn: &Connection, id: i64) -> Result<bool> {
        let rows_affected = conn.execute(crate::database::UPDATE_TASK_DONE, [&id])?;
        Ok(rows_affected > 0)
    }
}
