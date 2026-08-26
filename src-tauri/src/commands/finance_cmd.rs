use tauri::State;
use crate::commands::AppState;

#[tauri::command]
pub async fn get_finance(state: State<'_, AppState>) -> Result<crate::finance::FinanceRow, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::finance::get_finance(&pool, uc).await
}

#[tauri::command]
pub async fn get_injuries(state: State<'_, AppState>) -> Result<Vec<(i64, String, String, String, String)>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    let rows: Vec<(i64, String, String, String, String)> = sqlx::query_as(
        "SELECT i.id, p.common_name, i.injury_type, i.expected_return_date, i.injury_date FROM injuries i JOIN players p ON p.id=i.player_id JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 WHERE i.is_active=1 ORDER BY i.injury_date DESC"
    ).bind(uc).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}
