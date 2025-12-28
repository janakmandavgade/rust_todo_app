use rusqlite::{Connection, Result, params};
use serde::Serialize;
use uuid::{Uuid};

use crate::utils_todo::date_time_formatter::get_current_date_time;

#[derive(Debug, Serialize)]
pub struct SingleTodo {
    pub todo_id: String,
    pub user_id: String,
    pub todo_data: String,
    pub completed: bool,
    pub created_at: String,
}

pub fn insert_row(db_path: &str, todo_text: &str) -> Result<()> {

    let conn = Connection::open(db_path)?;

    let todo_id = Uuid::new_v4().to_string();
    let user_id = Uuid::new_v4().to_string();
    let created_at = get_current_date_time();
    // let todo_text = "Work Hard, Play Harder!";
    conn.execute(
        "INSERT INTO todos (
            todo_id,
            user_id,
            todo_data,
            completed,
            created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            todo_id,
            user_id,
            todo_text,
            false,
            created_at
        ],
    )?;

    Ok(())
}

pub fn get_all_rows(db_path: &str) -> Result<Vec<SingleTodo>>{
    let conn = Connection::open(db_path)?;

    let mut vec: Vec<SingleTodo> = Vec::new();
    
    let mut stmt = conn.prepare("SELECT todo_id, user_id, todo_data, completed, created_at FROM todos")?;

    let todo_iter = stmt.query_map([], |row| {
        Ok(SingleTodo{
            todo_id: row.get(0)?,
            user_id: row.get(1)?,
            todo_data: row.get(2)?,
            completed: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;

    for todo in todo_iter{
        vec.push(todo?);
    }

    Ok(vec)
}

pub fn get_completed_todos(db_path: &str) -> Result<Vec<SingleTodo>>{
    
    let conn =Connection::open(db_path)?; 

    let mut vec: Vec<SingleTodo> = Vec::new();
    
    let mut stmt = conn.prepare("select todo_id, user_id, todo_data, completed, created_at from todos where completed=?1")?;

    let todo_complete_iter = stmt.query_map([true], |row| {
        Ok(SingleTodo{
            todo_id: row.get(0)?,
            user_id: row.get(1)?,
            todo_data: row.get(2)?,
            completed: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;

    for todo in todo_complete_iter{
        vec.push(todo?);
    }

    return Ok(vec);
}

pub fn get_pending_todos(db_path: &str) -> Result<Vec<SingleTodo>>{
    
    let conn =Connection::open(db_path)?; 

    let mut vec: Vec<SingleTodo> = Vec::new();
    
    let mut stmt = conn.prepare("select todo_id, user_id, todo_data, completed, created_at from todos where completed=?1")?;

    let todo_complete_iter = stmt.query_map([false], |row| {
        Ok(SingleTodo{
            todo_id: row.get(0)?,
            user_id: row.get(1)?,
            todo_data: row.get(2)?,
            completed: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;

    for todo in todo_complete_iter{
        vec.push(todo?);
    }

    return Ok(vec);
}

pub fn set_as_completed(db_path: &str, todo_id: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;

    // SQL UPDATE statement
    let mut stmt = conn.prepare(
        "UPDATE todos SET completed = ?1 WHERE todo_id = ?2"
    )?;

    // Execute the update
    // We pass true (1 in SQLite) and the specific UUID
    let rows_affected = stmt.execute(params![true, todo_id])?;

    if rows_affected == 0 {
        println!("Warning: No todo found with ID: {}", todo_id);
    } else {
        println!("Successfully updated todo!");
    }

    return Ok(());
}