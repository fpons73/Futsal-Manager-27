use tauri::State;
use crate::commands::AppState;

#[tauri::command]
pub async fn get_market(state: State<'_, AppState>) -> Result<Vec<crate::transfer::MarketPlayer>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club usuario")?;
    crate::transfer::get_market(&pool, uc).await
}

#[tauri::command]
pub async fn get_offers(state: State<'_, AppState>) -> Result<Vec<crate::transfer::OfferRow>, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::transfer::get_offers(&pool, uc).await
}

#[tauri::command]
pub async fn make_offer(state: State<'_, AppState>, player_id: i64, fee: f64) -> Result<String, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    let (user_club,): (Option<i64>,) = sqlx::query_as("SELECT user_club_id FROM game_state WHERE id=1").fetch_one(&pool).await.map_err(|e| e.to_string())?;
    let uc = user_club.ok_or("Sin club")?;
    crate::transfer::make_offer(&pool, player_id, uc, fee).await
}

#[tauri::command]
pub async fn respond_offer(state: State<'_, AppState>, offer_id: i64, accept: bool) -> Result<String, String> {
    let pool = {
        let g = state.pool.lock().map_err(|e| e.to_string())?;
        g.clone().ok_or("No hay partida")?
    };
    crate::transfer::respond_offer(&pool, offer_id, accept).await
}
