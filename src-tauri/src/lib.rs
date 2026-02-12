use chrono::{NaiveDate, Timelike};
use dirs::data_local_dir;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle,
};
use tauri_plugin_notification::NotificationExt;

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

fn calc_days_until(dday_str: &str) -> Option<i64> {
    let dday = NaiveDate::parse_from_str(dday_str, "%Y-%m-%d").ok()?;
    let today = chrono::Local::now().date_naive();
    Some((dday - today).num_days())
}

// 알림 체크 함수
fn check_notifications(app: &AppHandle) {
    let todos = read_todos();
    let now = chrono::Local::now();
    let current_time = format!("{:02}:{:02}", now.hour(), now.minute());

    for todo in todos {
        if todo.completed {
            continue;
        }

        let Some(days_until) = calc_days_until(&todo.dday) else {
            continue;
        };

        // 알림 시간과 현재 시간이 일치하는지 확인
        if todo.notification_time != current_time {
            continue;
        }

        // notify_days에 해당하는 날인지 확인
        if todo.notify_days.contains(&days_until) {
            let body = if days_until == 0 {
                format!("오늘이 D-Day입니다! - {}", todo.title)
            } else {
                format!("D-{} - {}", days_until, todo.title)
            };

            let _ = app
                .notification()
                .builder()
                .title("📅 D-Day Todo 알림")
                .body(&body)
                .show();
        }
    }
}

// 백그라운드 스케줄러
fn start_scheduler(app: AppHandle) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(60));
            check_notifications(&app);
        }
    });
}

// CRUD Commands
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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // 시스템 트레이 메뉴 생성
            let open = MenuItem::with_id(app, "open", "열기", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "종료", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open, &quit])?;

            // 트레이 아이콘 생성
            TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("D-Day Todo")
                .on_tray_icon_event(|tray, event| {
                    // 트레이 아이콘 클릭 시 창 표시
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // 백그라운드 스케줄러 시작
            start_scheduler(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            todo_get_all,
            todo_add,
            todo_update,
            todo_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}