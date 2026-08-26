mod commands;
mod competition;
mod db;
mod editor;
mod engine;
mod finance;
mod season;
mod simulation;
mod training;
mod transfer;
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
      commands::game::get_next_fixture,
      commands::match_live::start_live_match,
      commands::match_live::tick_live,
      commands::match_live::get_live_snapshot,
      commands::transfer_cmd::get_market,
      commands::transfer_cmd::get_offers,
      commands::transfer_cmd::make_offer,
      commands::transfer_cmd::respond_offer,
      commands::training_cmd::get_training_schedule,
      commands::training_cmd::set_training_schedule,
      commands::training_cmd::get_training_progress,
      commands::training_cmd::get_training_types,
      commands::finance_cmd::get_finance,
      commands::finance_cmd::get_injuries,
      commands::season_cmd::check_season_finished,
      commands::season_cmd::rollover_season_cmd,
      commands::inbox_cmd::get_inbox,
      commands::inbox_cmd::mark_read,
      commands::inbox_cmd::mark_all_read,
      commands::editor_cmd::editor_init,
      commands::editor_cmd::editor_list_nations,
      commands::editor_cmd::editor_list_clubs,
      commands::editor_cmd::editor_list_players,
      commands::editor_cmd::editor_list_players_by_club,
      commands::editor_cmd::editor_assign_player,
      commands::editor_cmd::editor_release_player,
      commands::editor_cmd::editor_list_competitions,
      commands::editor_cmd::editor_list_confederations,
      commands::editor_cmd::editor_list_cities,
      commands::editor_cmd::editor_create_nation,
      commands::editor_cmd::editor_update_nation,
      commands::editor_cmd::editor_delete_nation,
      commands::editor_cmd::editor_create_club,
      commands::editor_cmd::editor_delete_club,
      commands::editor_cmd::editor_update_club,
      commands::editor_cmd::editor_create_player,
      commands::editor_cmd::editor_delete_player,
      commands::editor_cmd::editor_update_player,
      commands::editor_cmd::editor_create_competition,
      commands::editor_cmd::editor_delete_competition,
      commands::editor_cmd::editor_update_competition,
      commands::editor_cmd::editor_get_squad_count,
      commands::editor_cmd::editor_list_staff,
      commands::editor_cmd::editor_list_coaches,
      commands::editor_cmd::editor_create_staff,
      commands::editor_cmd::editor_update_staff,
      commands::editor_cmd::editor_delete_staff,
      commands::editor_cmd::editor_set_coach,
      commands::editor_cmd::editor_set_crest
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
