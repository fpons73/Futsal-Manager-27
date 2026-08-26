use tauri::State;
use crate::commands::AppState;

#[tauri::command]
pub async fn get_training_schedule(state: State<'_, AppState>) -> Result<Vec<crate::training::TrainingRow>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::training::ensure_default_schedule(&pool, uc).await?;
    crate::training::get_schedule(&pool, uc).await
}

#[tauri::command]
pub async fn set_training_schedule(state: State<'_, AppState>, schedule: Vec<(i64, i64, i64)>) -> Result<String, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::training::set_schedule(&pool, uc, schedule).await?;
    Ok("Calendario actualizado".into())
}

#[tauri::command]
pub async fn get_training_progress(state: State<'_, AppState>) -> Result<Vec<crate::training::ProgressRow>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::training::get_progress(&pool, uc).await
}

#[tauri::command]
pub async fn get_training_types(state: State<'_, AppState>) -> Result<Vec<(i64, String, String, i64)>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let rows: Vec<(i64, String, String, i64)> = sqlx::query_as("SELECT id, name, category, intensity FROM training_types ORDER BY id").fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}
