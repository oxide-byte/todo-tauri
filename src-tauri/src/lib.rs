use std::sync::Mutex;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;

#[tauri::command]
fn get_todo_list(storage: State<Storage>) -> Vec<Todo> {
    println!("Backend:get_todo_list");
    let s = storage.store.lock().unwrap();
    s.to_vec()
}

#[tauri::command]
fn add_todo(todo: Todo, storage: State<Storage>) {
    println!("Backend:add_todo: {:?}", todo);
    let mut s = storage.store.lock().unwrap();
    s.push(todo);
}

#[tauri::command]
fn edit_todo(todo: Todo, storage: State<Storage>) {
    println!("Backend:edit_todo");
    let mut s = storage.store.lock().unwrap();
    s.retain(|x| x.id != todo.id);
    s.push(todo);
}

#[tauri::command]
fn delete_todo(todo: Todo, storage: State<Storage>) {
    println!("Backend:delete_todo");
    let mut s = storage.store.lock().unwrap();
    s.retain(|x| x.id != todo.id);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>
}

#[derive(Debug)]
struct Storage {
    store: Mutex<Vec<Todo>>,
}
// To evaluate in the future
// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Storage { store: Default::default() })
        .invoke_handler(tauri::generate_handler![
            get_todo_list,
            add_todo,
            edit_todo,
            delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}