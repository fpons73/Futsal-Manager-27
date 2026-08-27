use serde::Serialize;
use tauri::State;

use crate::commands::AppState;
use crate::engine::{EngineTactics, MatchEngine, MatchSnapshot, PlayerAttrs, Role};

#[derive(Serialize)]
pub struct PreMatch {
    pub home_name: String,
    pub home_color: String,
    pub away_name: String,
    pub away_color: String,
    pub tactics: TacticsRow,
    pub squad: Vec<PreMatchPlayer>,
}

#[derive(Serialize)]
pub struct TacticsRow {
    pub formation: String,
    pub tempo: i64,
    pub pressing: i64,
    pub defensive_line: i64,
    pub width: i64,
    pub powerplay_enabled: bool,
}

#[derive(Serialize)]
pub struct PreMatchPlayer {
    pub id: i64,
    pub name: String,
    pub position: String,
    pub ca: i64,
}

async fn match_info(pool: &sqlx::SqlitePool, match_id: i64) -> Result<(i64, i64, String, String, String, String), String> {
    let (home, away): (i64, i64) = sqlx::query_as("SELECT home_club_id, away_club_id FROM matches WHERE id=?")
        .bind(match_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (hn, hc): (String, String) = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(home).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (an, ac): (String, String) = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(away).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok((home, away, hn, hc, an, ac))
}

async fn load_roster_raw(pool: &sqlx::SqlitePool, club_id: i64) -> Result<Vec<(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>, String> {
    let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(
        "SELECT p.id, pa.passing, pa.finishing, pa.dribbling, pa.tackling, pa.vision, pa.anticipation, pa.positioning, pa.stamina, pa.acceleration, pa.pace, pa.composure, pa.technique FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_attributes pa ON pa.player_id=p.id LIMIT 12"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}

fn attrs_from(r: &(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)) -> PlayerAttrs {
    PlayerAttrs::from_ints(r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9, r.10, r.11, r.12, 50)
}

async fn natural_role(pool: &sqlx::SqlitePool, pid: i64) -> Role {
    let (por, cie, ala, piv, uni): (i64, i64, i64, i64, i64) = sqlx::query_as(
        "SELECT por_natural, cie_natural, ala_natural, piv_natural, uni_natural FROM player_positions WHERE player_id=?"
    ).bind(pid).fetch_one(pool).await.unwrap_or((0, 0, 0, 0, 0));
    let best = [(por, Role::POR), (cie, Role::CIE), (ala, Role::ALA), (piv, Role::PIV), (uni, Role::UNI)]
        .into_iter().max_by_key(|(v, _)| *v).map(|(_, r)| r).unwrap_or(Role::UNI);
    best
}

fn role_for_index(idx: usize) -> Role {
    if idx < 2 { Role::POR } else if idx < 4 { Role::CIE } else if idx < 8 { Role::ALA } else if idx < 10 { Role::PIV } else { Role::UNI }
}

fn formation_code(f: &str) -> u8 {
    match f { "4-0" => 1, "2-2" => 2, "5-0" => 3, _ => 0 }
}

#[tauri::command]
pub async fn get_pre_match(state: State<'_, AppState>, match_id: i64) -> Result<PreMatch, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let (home, _away, hn, hc, an, ac) = match_info(&pool, match_id).await?;

    let tactics: (String, i64, i64, i64, i64, i64) = sqlx::query_as(
        "SELECT formation, tempo, pressing, defensive_line, width, powerplay_enabled FROM tactics WHERE club_id=?"
    ).bind(home).fetch_one(&pool).await.unwrap_or(("3-1".into(), 50, 50, 50, 50, 1));
    let tac = TacticsRow { formation: tactics.0.clone(), tempo: tactics.1, pressing: tactics.2, defensive_line: tactics.3, width: tactics.4, powerplay_enabled: tactics.5 == 1 };

    let rows = sqlx::query_as::<_, (i64, String, String, i64)>(
        "SELECT p.id, p.common_name, COALESCE(pp.pos,'UNI'), ps.current_ability FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_states ps ON ps.player_id=p.id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id ORDER BY ps.current_ability DESC"
    ).bind(home).fetch_all(&pool).await.map_err(|e| e.to_string())?;
    let squad = rows.into_iter().map(|(id, name, position, ca)| PreMatchPlayer { id, name, position, ca }).collect();

    Ok(PreMatch { home_name: hn, home_color: hc, away_name: an, away_color: ac, tactics: tac, squad })
}

#[tauri::command]
pub async fn start_live_match_tactics(
    state: State<'_, AppState>,
    match_id: i64,
    formation: String,
    tempo: i64,
    pressing: i64,
    defensive_line: i64,
    width: i64,
    powerplay_enabled: bool,
    lineup: Vec<i64>,
) -> Result<MatchSnapshot, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let (home, away, hn, hc, an, ac) = match_info(&pool, match_id).await?;

    let rows_home = load_roster_raw(&pool, home).await?;
    let rows_away = load_roster_raw(&pool, away).await?;

    // Equipo local: colocar primero el quintero elegido (con rol natural)
    let mut r1: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for pid in &lineup {
        let role = natural_role(&pool, *pid).await;
        if let Some(r) = rows_home.iter().find(|r| r.0 == *pid) {
            r1.push((r.0 as u32, (r1.len() + 1) as u8, role, attrs_from(r)));
        }
    }
    // resto de la plantilla (lesión/banquillo) en su rol natural
    for r in &rows_home {
        if !lineup.contains(&r.0) {
            let role = natural_role(&pool, r.0).await;
            r1.push((r.0 as u32, (r1.len() + 1) as u8, role, attrs_from(r)));
        }
    }

    // Equipo rival: rol por índice (descanso por el quinto inicial)
    let mut r2: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for r in &rows_away {
        let role = role_for_index(r2.len());
        r2.push((r.0 as u32, (r2.len() + 1) as u8, role, attrs_from(r)));
    }

    let mut eng = MatchEngine::new([(0, hn, hc), (1, an, ac)], [r1, r2]);
    // Tacticas del equipo local
    let t = EngineTactics {
        formation: formation_code(&formation),
        tempo: tempo as f32,
        pressing: pressing as f32,
        defensive_line: defensive_line as f32,
        width: width as f32,
    };
    eng.set_tactics(0, t);
    eng.set_allow_powerplay(0, powerplay_enabled);
    // Tacticas del rival desde BD (o default)
    let away_tac: Option<(String, i64, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT formation, tempo, pressing, defensive_line, width, powerplay_enabled FROM tactics WHERE club_id=?"
    ).bind(away).fetch_optional(&pool).await.map_err(|e| e.to_string())?;
    if let Some((af, at, ap, ad, aw, _)) = away_tac {
        eng.set_tactics(1, EngineTactics { formation: formation_code(&af), tempo: at as f32, pressing: ap as f32, defensive_line: ad as f32, width: aw as f32 });
    }
    eng.start();
    let snap = eng.snapshot();
    {
        let mut guard = state.live_match.lock().map_err(|e| e.to_string())?;
        *guard = Some(eng);
    }
    Ok(snap)
}

#[tauri::command]
pub async fn start_live_match(state: State<'_, AppState>, match_id: i64) -> Result<MatchSnapshot, String> {
    let pool = {
        let guard = state.pool.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("No hay partida activa")?
    };
    let (home, away, hn, hc, an, ac) = match_info(&pool, match_id).await?;
    let rows_home = load_roster_raw(&pool, home).await?;
    let rows_away = load_roster_raw(&pool, away).await?;
    let mut r1: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for r in &rows_home { let role = role_for_index(r1.len()); r1.push((r.0 as u32, (r1.len() + 1) as u8, role, attrs_from(r))); }
    let mut r2: Vec<(u32, u8, Role, PlayerAttrs)> = Vec::new();
    for r in &rows_away { let role = role_for_index(r2.len()); r2.push((r.0 as u32, (r2.len() + 1) as u8, role, attrs_from(r))); }
    let mut eng = MatchEngine::new([(0, hn, hc), (1, an, ac)], [r1, r2]);
    eng.start();
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
