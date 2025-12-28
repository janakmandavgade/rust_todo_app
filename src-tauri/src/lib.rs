// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{Result};
use tauri::Manager;
// use tauri::State;
// use std::sync::Mutex;
use std::env;
use std::fs;

mod db;
use db::connection::{table_exists, create_table, DbState};
use db::crud_methods::{insert_row, get_all_rows, get_completed_todos, get_pending_todos, set_as_completed, SingleTodo};
// use crate::db::crud_methods::{};

mod utils_todo;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_current_user() -> String {
    whoami::username()
}

#[tauri::command]
fn get_all_todos_tauri(state: tauri::State<DbState>) -> Result<Vec<SingleTodo>, String>{
    let db_path = &state.path;
    return get_all_rows(db_path).map_err(|e| e.to_string());
}

#[tauri::command]
fn get_all_completed_todos_tauri(state: tauri::State<DbState>) -> Result<Vec<SingleTodo>, String>{
    let db_path = &state.path;
    return get_completed_todos(db_path).map_err(|e| e.to_string());
}

#[tauri::command]
fn get_all_pending_todos_tauri(state: tauri::State<DbState>) -> Result<Vec<SingleTodo>, String>{
    let db_path = &state.path;
    return get_pending_todos(db_path).map_err(|e| e.to_string());
}

#[tauri::command]
fn set_todo_as_completed_tauri(state: tauri::State<DbState>, todo_id: String) -> Result<(), String>{
    let db_path = &state.path;
    return set_as_completed(db_path, &todo_id).map_err(|e| e.to_string());
}

#[tauri::command]
fn insert_todo_tauri(state: tauri::State<DbState>, todo_text: String) -> Result<(), String>{
    let db_path = &state.path;
    return insert_row(db_path, &todo_text).map_err(|e| e.to_string());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            
            // 1. Get the official App Data directory for your app
            // This will be something like: C:\Users\Name\AppData\Roaming\your-app-name
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");

            // 2. Create the folder if it doesn't exist yet
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir).expect("Failed to create storage directory");
            }

            // 3. Define the full path to the database file
            let db_path = app_data_dir.join("todos.db").to_string_lossy().to_string();
            
            println!("The current db_path is: {:?}", db_path);
            // 4. Store this path in Managed State so your commands can use it
            app.manage(DbState { path: db_path.clone() });

            // Ok(());
            
            // 1. Get the path
            // let db_path = return_db_path(); 
            
            // 2. Check and create table
            // We pass &db_path because your functions likely expect &str
            if !table_exists(&db_path, "todos").map_err(|e| e.to_string())? {
                create_table(&db_path).map_err(|e| e.to_string())?;
            }

            // 3. Return Ok to tell Tauri setup is done
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_current_user,
            insert_todo_tauri,
            set_todo_as_completed_tauri,
            get_all_pending_todos_tauri,
            get_all_completed_todos_tauri,
            get_all_todos_tauri
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
