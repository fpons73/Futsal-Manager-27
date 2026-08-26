use serde::{Deserialize, Serialize};
use tauri::State;
use crate::commands::AppState;

#[derive(Serialize, Deserialize)]
pub struct SimpleRow { pub id: i64, pub name: String }

fn pool_opt(state: &State<'_, AppState>) -> Result<Option<sqlx::SqlitePool>, String> {
    Ok(state.pool.lock().map_err(|e| e.to_string())?.clone())
}

#[tauri::command]
pub async fn editor_init(state: State<'_, AppState>) -> Result<i64, String> {
    let existing = pool_opt(&state)?;
    if let Some(pool) = existing {
        let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs").fetch_one(&pool).await.map_err(|e| e.to_string()).unwrap_or((0,));
        if cnt > 0 { return Ok(cnt); }
    }
    let pool = crate::db::init_pool(None).await.map_err(|e| e.to_string())?;
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs").fetch_one(&pool).await.map_err(|e| e.to_string()).unwrap_or((0,));
    if cnt == 0 {
        crate::world::seed_world(&pool).await?;
        let (c,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs").fetch_one(&pool).await.map_err(|e| e.to_string())?;
        // Asegurar que game_state no exista para que new_game pueda reutilizar
        sqlx::query("DELETE FROM game_state WHERE id=1").execute(&pool).await.ok();
        *state.pool.lock().map_err(|e| e.to_string())? = Some(pool.clone());
        Ok(c)
    } else {
        Ok(cnt)
    }
}

#[tauri::command]
pub async fn editor_list_nations(state: State<'_, AppState>) -> Result<Vec<crate::editor::NationRow>, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::list_nations(&pool).await
}
#[tauri::command]
pub async fn editor_list_clubs(state: State<'_, AppState>) -> Result<Vec<crate::editor::ClubRow>, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::list_clubs(&pool).await
}
#[tauri::command]
pub async fn editor_list_players(state: State<'_, AppState>, limit: Option<i64>) -> Result<Vec<crate::editor::PlayerRow>, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::list_players(&pool, limit.unwrap_or(100)).await
}
#[tauri::command]
pub async fn editor_list_competitions(state: State<'_, AppState>) -> Result<Vec<crate::editor::CompetitionRow>, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::list_competitions(&pool).await
}
#[tauri::command]
pub async fn editor_list_confederations(state: State<'_, AppState>) -> Result<Vec<SimpleRow>, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    let rows: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM confederations ORDER BY name").fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name)| SimpleRow { id, name }).collect())
}
#[tauri::command]
pub async fn editor_list_cities(state: State<'_, AppState>) -> Result<Vec<(i64, String, String)>, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    let rows: Vec<(i64, String, String)> = sqlx::query_as("SELECT ci.id, ci.name, n.name FROM cities ci JOIN nations n ON n.id=ci.nation_id ORDER BY n.name, ci.name LIMIT 200").fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}
#[tauri::command]
pub async fn editor_create_nation(state: State<'_, AppState>, name: String, confederation_id: i64, reputation: i64, futsal_level: i64) -> Result<i64, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::create_nation(&pool, name, confederation_id, reputation, futsal_level).await
}
#[tauri::command]
pub async fn editor_update_nation(state: State<'_, AppState>, id: i64, name: String, reputation: i64, futsal_level: i64) -> Result<(), String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::update_nation(&pool, id, name, reputation, futsal_level).await
}
#[tauri::command]
pub async fn editor_delete_nation(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::delete_nation(&pool, id).await
}
#[tauri::command]
pub async fn editor_create_club(state: State<'_, AppState>, name: String, short_name: String, nation_id: i64, city: String, stadium: String, capacity: i64, reputation: i64, c1: String, c2: String) -> Result<i64, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::create_club(&pool, name, short_name, nation_id, city, stadium, capacity, reputation, c1, c2).await
}
#[tauri::command]
pub async fn editor_delete_club(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::delete_club(&pool, id).await
}
#[tauri::command]
pub async fn editor_create_player(state: State<'_, AppState>, first: String, last: String, nation_id: i64, club_id: Option<i64>, ca: i64, pa: i64, pos: String) -> Result<i64, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::create_player(&pool, first, last, nation_id, club_id, ca, pa, pos).await
}
#[tauri::command]
pub async fn editor_delete_player(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::delete_player(&pool, id).await
}
#[tauri::command]
pub async fn editor_create_competition(state: State<'_, AppState>, name: String, nation_id: Option<i64>, tier: Option<i64>, total_teams: i64, season: String) -> Result<i64, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::create_competition(&pool, name, nation_id, tier, total_teams, season).await
}
#[tauri::command]
pub async fn editor_delete_competition(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    crate::editor::delete_competition(&pool, id).await
}
#[tauri::command]
pub async fn editor_get_squad_count(state: State<'_, AppState>, club_id: i64) -> Result<i64, String> {
    let pool = state.pool.lock().map_err(|e| e.to_string())?.clone().ok_or("No hay partida")?;
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE club_id=? AND is_active=1").bind(club_id).fetch_one(&pool).await.map_err(|e| e.to_string())?;
    Ok(cnt)
}
