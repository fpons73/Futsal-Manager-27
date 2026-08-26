mod commands;

use commands::AppState;

pub fn run() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::get_app_info
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
