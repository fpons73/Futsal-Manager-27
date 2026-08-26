mod commands;
mod competition;
mod db;
mod engine;
mod world;

use commands::AppState;

pub fn run() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::get_app_info,
      commands::test_db,
      commands::seed_world_cmd
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
