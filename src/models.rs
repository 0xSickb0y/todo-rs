use anyhow::Result;
use rusqlite::Connection;
use chrono::{Local, NaiveDateTime};

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub done: bool,
    pub birth: NaiveDateTime,
}

impl Task {
    pub fn create_default(conn: &Connection) -> Result<()> {
        conn.execute(crate::database::CREATE_TASK_TABLE, [])?;
        Ok(())
    }

    pub fn add(conn: &Connection, description: String) -> Result<i64> {
        let now = Local::now().naive_local();
        let birth_str = now.format("%Y-%m-%d %H:%M:%S").to_string();

        conn.execute(crate::database::INSERT_TASK, (&description, &birth_str))?;
        Ok(conn.last_insert_rowid())
    }

    pub fn list(conn: &Connection) -> Result<Vec<Task>> {
        let mut stmt = conn.prepare(crate::database::SELECT_ALL_TASKS)?;
        let task_iter = stmt.query_map([], |row| {
            let date_str: String = row.get(3)?;
            let parsed = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|_e| rusqlite::Error::InvalidColumnType(3, date_str.clone(), rusqlite::types::Type::Text))?;

            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
                done: row.get(2)?,
                birth: parsed,
            })
        })?;

        Ok(task_iter.filter_map(Result::ok).collect())
    }

    pub fn remove(conn: &Connection, id: i64) -> Result<bool> {
        let rows_affected = conn.execute(crate::database::DELETE_TASK, [&id])?;
        Ok(rows_affected > 0)
    }

    pub fn mark_done(conn: &Connection, id: i64) -> Result<bool> {
        let rows_affected = conn.execute(crate::database::UPDATE_TASK_DONE, [&id])?;
        Ok(rows_affected > 0)
    }
}