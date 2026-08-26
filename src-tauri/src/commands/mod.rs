use serde::Serialize;
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
  #[allow(dead_code)]
  pub current_date: Mutex<String>,
}

#[derive(Serialize)]
pub struct AppInfo {
  name: String,
  version: String,
}

#[tauri::command]
pub fn ping() -> String {
  "pong".to_string()
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
  AppInfo {
    name: "Futsal Manager 27".to_string(),
    version: "0.1.0".to_string(),
  }
}
