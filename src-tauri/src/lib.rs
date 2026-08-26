mod commands;
mod competition;
mod db;
mod engine;
mod simulation;
mod world;

use commands::AppState;

pub fn run() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::get_app_info,
      commands::test_db,
      commands::seed_world_cmd,
      commands::game::new_game,
      commands::game::get_game_state,
      commands::game::advance_day_cmd,
      commands::game::advance_week_cmd,
      commands::game::get_standings,
      commands::game::get_fixtures,
      commands::game::get_squad,
      commands::game::get_competitions,
      commands::game::get_next_fixture
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
