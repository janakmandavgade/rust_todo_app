use rusqlite::{Connection, Result};
// use crate::db::crud_methods::insert_row;
use std::env;

pub struct DbState {
    pub path: String,
}

pub fn return_db_path() -> String{
    let relative_path = "./src/todos.db";
    let mut absolute_path = env::current_dir().unwrap();

    // check folder exists or not
    absolute_path.push(relative_path);
    let absolute_path_str = absolute_path.to_str().expect("Error in converting absolute path to string");

    return absolute_path_str.to_string();
}

pub fn create_table(db_path: &str) -> Result<()> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            todo_id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            todo_data TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    println!("Successfully created table todos");
    return Ok(());
}

pub fn table_exists(db_path: &str, table_name: &str) -> Result<bool> {

    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(
        "Select 1 from sqlite_master where type='table' and name=?1 limit 1;"
    )?;

    let table_exists = stmt.exists([table_name])?;

    Ok(table_exists)
}

// pub fn get_row(table_name: &str) -> Result<bool> {

//     let conn = Connection::open("todos.db");
//     let mut stmt = conn.prepare(
//         "Select 1 from sqlite_master where type='table' and name=?1 limit 1;"
//     );

//     let tableExists = stmt.exists([table_name])?;

//     Ok(exists)
// }

// fn main() -> Result<()> {
//     let conn = Connection::open("todos.db")?;

//     if(!table_exists("todos.db")){
//         create_table();
//     }

//     insert_row();
//     Ok(());
// }