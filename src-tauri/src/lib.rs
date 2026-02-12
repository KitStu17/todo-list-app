use chrono::NaiveDate;
use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Todo 구조체 정의
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub dday: String,
    pub notification_time: String,
    pub notify_days: Vec<i64>,
    pub completed: bool,
    pub created_at: String,
}

// 데이터 저장 경로
fn get_data_path() -> PathBuf {
    let mut path = data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("todo-list-app");
    fs::create_dir_all(&path).unwrap();
    path.push("todos.json");
    path
}

// 파일에서 읽기
fn read_todos() -> Vec<Todo> {
    let path = get_data_path();
    if path.exists() {
        let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

// 파일에 쓰기
fn write_todos(todos: &Vec<Todo>) -> Result<(), String> {
    let path = get_data_path();
    let json = serde_json::to_string_pretty(todos).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

// 남은 일수 계산
pub fn calc_days_until(dday_str: &str) -> Option<i64> {
    let dday = NaiveDate::parse_from_str(dday_str, "%Y-%m-%d").ok()?;
    let today = chrono::Local::now().date_naive();
    Some((dday - today).num_days())
}

// CRUD 함수들
#[tauri::command]
fn todo_get_all() -> Result<Vec<Todo>, String> {
    Ok(read_todos())
}

#[tauri::command]
fn todo_add(todo: Todo) -> Result<Todo, String> {
    let mut todos = read_todos();
    todos.push(todo.clone());
    write_todos(&todos)?;
    Ok(todo)
}

#[tauri::command]
fn todo_update(id: String, updated: Todo) -> Result<Todo, String> {
    let mut todos = read_todos();
    let pos = todos
        .iter()
        .position(|t| t.id == id)
        .ok_or_else(|| "Todo를 찾을 수 없습니다.".to_string())?;
    todos[pos] = updated.clone();
    write_todos(&todos)?;
    Ok(updated)
}

#[tauri::command]
fn todo_delete(id: String) -> Result<(), String> {
    let mut todos = read_todos();
    todos.retain(|t| t.id != id);
    write_todos(&todos)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            todo_get_all,
            todo_add,
            todo_update,
            todo_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
