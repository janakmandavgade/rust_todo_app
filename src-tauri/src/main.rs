// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub struct DbState {
    path: String,
}

fn main() {
    todo_lib::run()
}

// Code for testing DB functions!


// use rusqlite::{Result};
// // use directories::BaseDirs;
// use std::env;
// // use std::path::PathBuf;
// mod db;
// use db::connection::{table_exists, create_table};
// use db::crud_methods::{insert_row, get_all_rows, get_completed_todos, get_pending_todos};
// mod utils_todo;
// use crate::db::crud_methods::set_as_completed;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // let conn = Connection::open("todos.db")?;
    
//     let relative_path = "./src/todos.db";
//     let mut absolute_path = env::current_dir().unwrap();

//     //check folder exists or not

//     absolute_path.push(relative_path);
//     let absolute_path_str = absolute_path.to_str().expect("Error in converting absolute path to string");

//     println!("Constructed path: {:?}", absolute_path);
//     // println!("Abs path", absolute_path);

//     if !table_exists(absolute_path_str, "todos")? {
//         create_table(absolute_path_str)?;
//     }

//     insert_row(absolute_path_str, "Work Hard, Play Harder V2.0")?;
//     let vec = get_all_rows(absolute_path_str)?;

//     for row in vec{
//         println!(
//             "todoId: {}, user_id: {}, todo_data: {}, completed: {}, created_at: {}", 
//             row.todo_id,
//             row.user_id,
//             row.todo_data,
//             row.completed,
//             row.created_at,
//         )
//     }
//     let todo_id = "86242b84-c348-4a73-bd84-5c083e2e349e";
//     set_as_completed(absolute_path_str, todo_id)?;

//     let completed_todo = get_completed_todos(absolute_path_str)?;

//     for row in completed_todo{
//         println!(
//             "Completed Todo: todoId: {}, user_id: {}, todo_data: {}, completed: {}, created_at: {}", 
//             row.todo_id,
//             row.user_id,
//             row.todo_data,
//             row.completed,
//             row.created_at,
//         )
//     }
//     let pending_todo = get_pending_todos(absolute_path_str)?;

//     for row in pending_todo{
//         println!(
//             "Pending Todos: todoId: {}, user_id: {}, todo_data: {}, completed: {}, created_at: {}", 
//             row.todo_id,
//             row.user_id,
//             row.todo_data,
//             row.completed,
//             row.created_at,
//         )
//     }

//     return Ok(());
// }
