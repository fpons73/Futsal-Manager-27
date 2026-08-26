use tauri::State;
use crate::commands::AppState;

#[tauri::command]
pub async fn check_season_finished(state: State<'_, AppState>) -> Result<bool, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    crate::season::is_season_finished(&pool).await
}

#[tauri::command]
pub async fn rollover_season_cmd(state: State<'_, AppState>) -> Result<String, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    crate::season::rollover_season(&pool).await
}
