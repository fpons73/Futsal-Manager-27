use tauri::State;

use crate::commands::AppState;
use crate::engine::{MatchEngine, MatchSnapshot};

async fn build_engine_for_match(pool: &sqlx::SqlitePool, match_id: i64) -> Result<MatchEngine, String> {
    let (home, away): (i64, i64) = sqlx::query_as("SELECT home_club_id, away_club_id FROM matches WHERE id=?")
        .bind(match_id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    let home_row: (String, String) = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(home)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;
    let away_row: (String, String) = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(away)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

    let rows_home = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(
        "SELECT p.id, pa.passing, pa.finishing, pa.dribbling, pa.tackling, pa.vision, pa.anticipation, pa.positioning, pa.stamina, pa.acceleration, pa.pace, pa.composure, pa.technique FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_attributes pa ON pa.player_id=p.id LIMIT 12"
    ).bind(home).fetch_all(pool).await.map_err(|e| e.to_string())?;

    let rows_away = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(
        "SELECT p.id, pa.passing, pa.finishing, pa.dribbling, pa.tackling, pa.vision, pa.anticipation, pa.positioning, pa.stamina, pa.acceleration, pa.pace, pa.composure, pa.technique FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_attributes pa ON pa.player_id=p.id LIMIT 12"
    ).bind(away).fetch_all(pool).await.map_err(|e| e.to_string())?;

    let mut r1 = Vec::new();
    for (pid, passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique) in rows_home {
        let role = if r1.len() < 2 { crate::engine::Role::POR } else if r1.len() < 4 { crate::engine::Role::CIE } else if r1.len() < 8 { crate::engine::Role::ALA } else if r1.len() < 10 { crate::engine::Role::PIV } else { crate::engine::Role::UNI };
        let attrs = crate::engine::PlayerAttrs::from_ints(passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique, 10);
        r1.push((pid as u32, (r1.len() + 1) as u8, role, attrs));
    }
    let mut r2 = Vec::new();
    for (pid, passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique) in rows_away {
        let role = if r2.len() < 2 { crate::engine::Role::POR } else if r2.len() < 4 { crate::engine::Role::CIE } else if r2.len() < 8 { crate::engine::Role::ALA } else if r2.len() < 10 { crate::engine::Role::PIV } else { crate::engine::Role::UNI };
        let attrs = crate::engine::PlayerAttrs::from_ints(passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique, 10);
        r2.push((pid as u32, (r2.len() + 1) as u8, role, attrs));
    }

    let mut eng = MatchEngine::new([(0, home_row.0, home_row.1), (1, away_row.0, away_row.1)], [r1, r2]);
    eng.start();
    Ok(eng)
}

#[tauri::command]
pub async fn start_live_match(state: State<'_, AppState>, match_id: i64) -> Result<MatchSnapshot, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let eng = build_engine_for_match(&pool, match_id).await?;
    let snap = eng.snapshot();
    {
        let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
        *guard = Some(eng);
    }
    Ok(snap)
}

#[tauri::command]
pub async fn tick_live(state: State<'_, AppState>, ticks: Option<u32>) -> Result<MatchSnapshot, String> {
    let n = ticks.unwrap_or(1) as usize;
    let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
    let eng = guard.as_mut().ok_or("No hay partido en vivo")?;
    for _ in 0..n {
        eng.tick();
        if eng.state == crate::engine::MatchState::Finished { break; }
    }
    Ok(eng.snapshot())
}

#[tauri::command]
pub async fn get_live_snapshot(state: State<'_, AppState>) -> Result<MatchSnapshot, String> {
    let guard = state.live_match.lock().map_err(|e| e.to_string())?;
    let eng = guard.as_ref().ok_or("No hay partido en vivo")?;
    Ok(eng.snapshot())
}
