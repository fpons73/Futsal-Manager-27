use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize)]
pub struct NationRow { pub id: i64, pub name: String, pub confederation: String, pub confederation_id: i64, pub reputation: i64, pub futsal_level: i64 }
#[derive(Serialize, Deserialize)]
pub struct ClubRow { pub id: i64, pub name: String, pub short_name: String, pub nation: String, pub nation_id: i64, pub city: String, pub city_id: Option<i64>, pub stadium: String, pub capacity: i64, pub reputation: i64, pub primary_color: String, pub secondary_color: String, pub crest_path: Option<String>, pub coach_id: Option<i64>, pub coach_name: Option<String>, pub staff_count: i64, pub squad_count: i64 }
#[derive(Serialize, Deserialize)]
pub struct PlayerRow { pub id: i64, pub first_name: String, pub last_name: String, pub common_name: String, pub nation: String, pub nation_id: i64, pub club: String, pub club_id: Option<i64>, pub position: String, pub ca: i64, pub pa: i64, pub age: i64, pub foot: String }
#[derive(Serialize, Deserialize)]
pub struct CompetitionRow { pub id: i64, pub name: String, pub nation: Option<String>, pub nation_id: Option<i64>, pub tier: Option<i64>, pub total_teams: Option<i64>, pub season: String, pub format: String }
#[derive(Serialize, Deserialize, Clone)]
pub struct StaffRow { pub id: i64, pub first_name: String, pub last_name: String, pub common_name: String, pub nation: String, pub nation_id: i64, pub role: String, pub club_id: Option<i64>, pub club_name: Option<String>, pub tactical: i64, pub man_management: i64, pub judging: i64, pub motivating: i64, pub working_youngsters: i64, pub physio_level: i64, pub wage_weekly: f64 }

pub async fn list_nations(pool: &SqlitePool) -> Result<Vec<NationRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, i64, String, i64, i64)>("SELECT n.id, n.name, n.confederation_id, c.short_name, n.reputation, n.futsal_level FROM nations n JOIN confederations c ON c.id=n.confederation_id ORDER BY n.name")
        .fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, confederation_id, confederation, reputation, futsal_level)| NationRow { id, name, confederation, confederation_id, reputation, futsal_level }).collect())
}
pub async fn list_clubs(pool: &SqlitePool) -> Result<Vec<ClubRow>, String> {
    #[derive(sqlx::FromRow)]
    #[allow(dead_code)]
    struct Cr {
        id: i64, name: String, short_name: String, nation: String, nation_id: i64,
        city: Option<String>, city_id: Option<i64>, stadium: Option<String>, capacity: Option<i64>,
        reputation: i64, primary_color: String, secondary_color: String,
        crest_path: Option<String>, coach_id: Option<i64>, coach_name: Option<String>,
        staff_count: i64, squad_count: i64,
    }
    let rows = sqlx::query_as::<_, Cr>(
        "SELECT c.id, c.name, c.short_name, n.name AS nation, c.nation_id, ci.name AS city, c.city_id, s.name AS stadium, s.capacity, c.reputation, c.primary_color, c.secondary_color, c.crest_path, c.coach_id, coach.common_name AS coach_name,
                (SELECT COUNT(*) FROM staff st WHERE st.club_id=c.id) AS staff_count,
                (SELECT COUNT(*) FROM contracts ct WHERE ct.club_id=c.id AND ct.is_active=1) AS squad_count
         FROM clubs c JOIN nations n ON n.id=c.nation_id LEFT JOIN cities ci ON ci.id=c.city_id LEFT JOIN stadiums s ON s.id=c.stadium_id LEFT JOIN staff coach ON coach.id=c.coach_id ORDER BY c.reputation DESC, c.name"
    ).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|r| ClubRow { id: r.id, name: r.name, short_name: r.short_name, nation: r.nation, nation_id: r.nation_id, city: r.city.unwrap_or_default(), city_id: r.city_id, stadium: r.stadium.unwrap_or_default(), capacity: r.capacity.unwrap_or(2000), reputation: r.reputation, primary_color: r.primary_color, secondary_color: r.secondary_color, crest_path: r.crest_path, coach_id: r.coach_id, coach_name: r.coach_name, staff_count: r.staff_count, squad_count: r.squad_count }).collect())
}
pub async fn list_players(pool: &SqlitePool, limit: i64) -> Result<Vec<PlayerRow>, String> {
    let lim = limit.clamp(20, 2000);
    let rows = sqlx::query_as::<_, (i64, String, String, String, String, i64, Option<String>, Option<i64>, Option<String>, i64, i64, String)>(
        "SELECT p.id, p.first_name, p.last_name, p.common_name, n.name, p.nation_id, cl.name, c.club_id, COALESCE(pp.pos,'UNI'), ps.current_ability, ps.potential_ability, p.preferred_foot FROM players p JOIN nations n ON n.id=p.nation_id JOIN player_states ps ON ps.player_id=p.id LEFT JOIN contracts c ON c.player_id=p.id AND c.is_active=1 LEFT JOIN clubs cl ON cl.id=c.club_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id ORDER BY ps.current_ability DESC LIMIT ?"
    ).bind(lim).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, first_name, last_name, common_name, nation, nation_id, club, club_id, position, ca, pa, foot)| {
        PlayerRow { id, first_name, last_name, common_name, nation, nation_id, club: club.unwrap_or_default(), club_id, position: position.unwrap_or_else(|| "UNI".into()), ca, pa, age: 0, foot }
    }).collect())
}
pub async fn list_players_by_club(pool: &SqlitePool, club_id: i64) -> Result<Vec<PlayerRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, String, String, i64, Option<String>, Option<i64>, Option<String>, i64, i64, String)>(
        "SELECT p.id, p.first_name, p.last_name, p.common_name, n.name, p.nation_id, cl.name, c.club_id, COALESCE(pp.pos,'UNI'), ps.current_ability, ps.potential_ability, p.preferred_foot FROM players p JOIN nations n ON n.id=p.nation_id JOIN player_states ps ON ps.player_id=p.id JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 LEFT JOIN clubs cl ON cl.id=c.club_id LEFT JOIN (SELECT player_id, CASE WHEN por_natural>=18 THEN 'POR' WHEN cie_natural>=18 THEN 'CIE' WHEN piv_natural>=18 THEN 'PIV' WHEN ala_natural>=18 THEN 'ALA' ELSE 'UNI' END as pos FROM player_positions) pp ON pp.player_id=p.id ORDER BY ps.current_ability DESC"
    ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, first_name, last_name, common_name, nation, nation_id, club, club_id, position, ca, pa, foot)| {
        PlayerRow { id, first_name, last_name, common_name, nation, nation_id, club: club.unwrap_or_default(), club_id, position: position.unwrap_or_else(|| "UNI".into()), ca, pa, age: 0, foot }
    }).collect())
}
pub async fn assign_player(pool: &SqlitePool, player_id: i64, club_id: i64) -> Result<(), String> {
    let has: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE player_id=? AND is_active=1").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (ca,): (i64,) = sqlx::query_as("SELECT current_ability FROM player_states WHERE player_id=?").bind(player_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if has.0 > 0 {
        sqlx::query("UPDATE contracts SET club_id=?, wage_weekly=? WHERE player_id=? AND is_active=1").bind(club_id).bind(ca as f64*18.0).bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    } else {
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(player_id).bind(club_id).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub async fn release_player(pool: &SqlitePool, player_id: i64) -> Result<(), String> {
    sqlx::query("UPDATE contracts SET is_active=0 WHERE player_id=? AND is_active=1").bind(player_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn list_competitions(pool: &SqlitePool) -> Result<Vec<CompetitionRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, Option<i64>, Option<String>, Option<i64>, Option<i64>, String, String)>(
        "SELECT comp.id, comp.name, comp.nation_id, n.name, comp.tier, comp.total_teams, comp.season, comp.format FROM competitions comp LEFT JOIN nations n ON n.id=comp.nation_id ORDER BY comp.tier NULLS LAST, comp.name"
    ).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, name, nation_id, nation, tier, total_teams, season, format)| CompetitionRow { id, name, nation, nation_id, tier, total_teams, season, format }).collect())
}
pub async fn list_stadiums(pool: &SqlitePool) -> Result<Vec<(i64, String, String, i64)>, String> {
    let rows: Vec<(i64, String, String, i64)> = sqlx::query_as("SELECT s.id, s.name, COALESCE(ci.name,'-'), s.capacity FROM stadiums s LEFT JOIN cities ci ON ci.id=s.city_id ORDER BY s.capacity DESC").fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows)
}

pub async fn create_nation(pool: &SqlitePool, name: String, confederation_id: i64, reputation: i64, futsal_level: i64) -> Result<i64, String> {
    let (id,): (i64,) = sqlx::query_as("INSERT INTO nations(name, confederation_id, reputation, futsal_level) VALUES(?,?,?,?) RETURNING id").bind(name).bind(confederation_id).bind(reputation).bind(futsal_level).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(id)
}
pub async fn update_nation(pool: &SqlitePool, id: i64, name: String, reputation: i64, futsal_level: i64) -> Result<(), String> {
    sqlx::query("UPDATE nations SET name=?, reputation=?, futsal_level=? WHERE id=?").bind(name).bind(reputation).bind(futsal_level).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn delete_nation(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs WHERE nation_id=?").bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if cnt > 0 { return Err(format!("No se puede borrar: {} clubes dependen de esta nación", cnt)); }
    sqlx::query("DELETE FROM nations WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn create_club(pool: &SqlitePool, name: String, short: String, nation_id: i64, city: String, stadium: String, capacity: i64, rep: i64, c1: String, c2: String) -> Result<i64, String> {
    let city_id = if city.is_empty() { None } else {
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM cities WHERE name=? AND nation_id=?").bind(&city).bind(nation_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
        if let Some((id,)) = existing { Some(id) } else {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name, nation_id, population) VALUES(?,?,500000) RETURNING id").bind(&city).bind(nation_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
            Some(id)
        }
    };
    let stadium_id = if stadium.is_empty() { None } else {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO stadiums(name, city_id, capacity) VALUES(?,?,?) RETURNING id").bind(&stadium).bind(city_id).bind(capacity).fetch_one(pool).await.map_err(|e| e.to_string())?;
        Some(id)
    };
    let (id,): (i64,) = sqlx::query_as("INSERT INTO clubs(name, short_name, nation_id, city_id, stadium_id, reputation, primary_color, secondary_color) VALUES(?,?,?,?,?,?,?,?) RETURNING id")
        .bind(name).bind(short).bind(nation_id).bind(city_id).bind(stadium_id).bind(rep).bind(c1).bind(c2).fetch_one(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO club_finances(club_id, balance, transfer_budget, wage_budget) VALUES(?,?,?,?)").bind(id).bind(rep as f64*1800.0).bind(rep as f64*450.0).bind(rep as f64*12.0+2000.0).execute(pool).await.ok();
    sqlx::query("INSERT INTO tactics(club_id, formation) VALUES(?, '3-1')").bind(id).execute(pool).await.ok();
    for (day, tid, intensity) in [(0,1,70),(1,2,75),(2,4,65),(3,3,75),(4,7,60)] {
        sqlx::query("INSERT OR IGNORE INTO training_schedule(club_id, day_of_week, training_type_id, intensity) VALUES(?,?,?,?)").bind(id).bind(day).bind(tid).bind(intensity).execute(pool).await.ok();
    }
    // Crear 12 jugadores ficticios para el club
    let base_date = chrono::NaiveDate::from_ymd_opt(2026,7,10).unwrap();
    for idx in 0..12 {
        let role = match idx { 0|1 => "POR", 2|3 => "CIE", 4..=7 => "ALA", 8|9 => "PIV", _ => "UNI" };
        let age = if idx<2 { 26 } else { 24 };
        let dob = base_date - chrono::Duration::days(age*365);
        let first = "Nuevo"; let last = format!("Jugador{}", idx+1);
        let ca = 70 + (rep/20) as i64; let pa = ca + 15;
        let (pid,): (i64,) = sqlx::query_as("INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?) RETURNING id")
            .bind(first).bind(last.clone()).bind(format!("{} {}", first, last)).bind(dob.format("%Y-%m-%d").to_string()).bind(nation_id).bind(180).bind(75).fetch_one(pool).await.map_err(|e| e.to_string())?;
        let (por,cie,ala,piv,uni) = match role { "POR"=>(20,2,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
        sqlx::query("INSERT INTO player_positions(player_id,por_natural,cie_natural,ala_natural,piv_natural,uni_natural) VALUES(?,?,?,?,?,?)").bind(pid).bind(por).bind(cie).bind(ala).bind(piv).bind(uni).execute(pool).await.ok();
        sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability) VALUES(?,?,?)").bind(pid).bind(ca).bind(pa).execute(pool).await.ok();
        sqlx::query("INSERT INTO player_attributes(player_id) VALUES(?)").bind(pid).execute(pool).await.ok();
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(pid).bind(id).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.ok();
    }
    sqlx::query("UPDATE club_finances SET total_wages=(SELECT SUM(wage_weekly) FROM contracts WHERE club_id=? AND is_active=1) WHERE club_id=?").bind(id).bind(id).execute(pool).await.ok();
    Ok(id)
}
pub async fn update_club(pool: &SqlitePool, id: i64, name: String, short: String, nation_id: i64, city: String, stadium: String, capacity: i64, rep: i64, c1: String, c2: String) -> Result<(), String> {
    let city_id = if city.is_empty() { None } else {
        let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM cities WHERE name=? AND nation_id=?").bind(&city).bind(nation_id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
        if let Some((id,)) = existing { Some(id) } else {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name, nation_id, population) VALUES(?,?,500000) RETURNING id").bind(&city).bind(nation_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
            Some(id)
        }
    };
    let cur_stadium: Option<(Option<i64>,)> = sqlx::query_as("SELECT stadium_id FROM clubs WHERE id=?").bind(id).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let existing_stadium = cur_stadium.and_then(|(s,)| s);
    let stadium_id = if stadium.is_empty() { existing_stadium } else {
        if let Some(es) = existing_stadium {
            sqlx::query("UPDATE stadiums SET name=?, capacity=? WHERE id=?").bind(&stadium).bind(capacity).bind(es).execute(pool).await.map_err(|e| e.to_string())?;
            Some(es)
        } else {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO stadiums(name, city_id, capacity) VALUES(?,?,?) RETURNING id").bind(&stadium).bind(city_id).bind(capacity).fetch_one(pool).await.map_err(|e| e.to_string())?;
            Some(id)
        }
    };
    sqlx::query("UPDATE clubs SET name=?, short_name=?, nation_id=?, city_id=?, stadium_id=?, reputation=?, primary_color=?, secondary_color=? WHERE id=?")
        .bind(name).bind(short).bind(nation_id).bind(city_id).bind(stadium_id).bind(rep).bind(c1).bind(c2).bind(id)
        .execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn update_player(pool: &SqlitePool, id: i64, first: String, last: String, nation_id: i64, club_id: Option<i64>, ca: i64, pa: i64, pos: String) -> Result<(), String> {
    sqlx::query("UPDATE players SET first_name=?, last_name=?, common_name=?, nation_id=? WHERE id=?").bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(nation_id).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("UPDATE player_states SET current_ability=?, potential_ability=? WHERE player_id=?").bind(ca).bind(pa).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    let (por,cie,ala,piv,uni) = match pos.as_str() { "POR"=>(20,2,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
    sqlx::query("UPDATE player_positions SET por_natural=?, cie_natural=?, ala_natural=?, piv_natural=?, uni_natural=? WHERE player_id=?").bind(por).bind(cie).bind(ala).bind(piv).bind(uni).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    if let Some(cid) = club_id {
        let has: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts WHERE player_id=? AND is_active=1").bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if has.0 == 0 {
            sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(id).bind(cid).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.map_err(|e| e.to_string())?;
        } else {
            sqlx::query("UPDATE contracts SET club_id=?, wage_weekly=? WHERE player_id=? AND is_active=1").bind(cid).bind(ca as f64*18.0).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
pub async fn update_competition(pool: &SqlitePool, id: i64, name: String, nation_id: Option<i64>, tier: Option<i64>, teams: i64, season: String) -> Result<(), String> {
    sqlx::query("UPDATE competitions SET name=?, nation_id=?, tier=?, total_teams=?, season=? WHERE id=?").bind(name).bind(nation_id).bind(tier).bind(teams).bind(season).bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn delete_club(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE home_club_id=? OR away_club_id=?").bind(id).bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if cnt>0 { return Err(format!("No se puede borrar: {} partidos referencian al club", cnt)); }
    sqlx::query("UPDATE contracts SET is_active=0 WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM club_finances WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM tactics WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM training_schedule WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM league_standings WHERE club_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM clubs WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn create_player(pool: &SqlitePool, first: String, last: String, nation_id: i64, club_id: Option<i64>, ca: i64, pa: i64, pos: String) -> Result<i64, String> {
    let dob = chrono::NaiveDate::from_ymd_opt(2000,6,15).unwrap().format("%Y-%m-%d").to_string();
    let (pid,): (i64,) = sqlx::query_as("INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?) RETURNING id")
        .bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(dob).bind(nation_id).bind(180).bind(75).fetch_one(pool).await.map_err(|e| e.to_string())?;
    let (por,cie,ala,piv,uni) = match pos.as_str() { "POR"=>(20,2,1,1,3), "CIE"=>(1,20,12,8,10), "ALA"=>(1,10,20,10,14), "PIV"=>(1,6,10,20,12), _=>(3,10,14,14,20) };
    sqlx::query("INSERT INTO player_positions(player_id,por_natural,cie_natural,ala_natural,piv_natural,uni_natural) VALUES(?,?,?,?,?,?)").bind(pid).bind(por).bind(cie).bind(ala).bind(piv).bind(uni).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability) VALUES(?,?,?)").bind(pid).bind(ca).bind(pa).execute(pool).await.map_err(|e| e.to_string())?;
    sqlx::query("INSERT INTO player_attributes(player_id) VALUES(?)").bind(pid).execute(pool).await.map_err(|e| e.to_string())?;
    if let Some(cid) = club_id {
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)").bind(pid).bind(cid).bind(ca as f64*18.0).bind("2026-07-10").bind("2029-07-10").execute(pool).await.map_err(|e| e.to_string())?;
    }
    Ok(pid)
}
pub async fn delete_player(pool: &SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE contracts SET is_active=0 WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM player_attributes WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM player_states WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM player_positions WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM injuries WHERE player_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM players WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn create_competition(pool: &SqlitePool, name: String, nation_id: Option<i64>, tier: Option<i64>, teams: i64, season: String) -> Result<i64, String> {
    let (id,): (i64,) = sqlx::query_as("INSERT INTO competitions(name, nation_id, tier, total_teams, season) VALUES(?,?,?,?,?) RETURNING id").bind(name).bind(nation_id).bind(tier).bind(teams).bind(season).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(id)
}
pub async fn delete_competition(pool: &SqlitePool, id: i64) -> Result<(), String> {
    let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=?").bind(id).fetch_one(pool).await.map_err(|e| e.to_string())?;
    if cnt>0 { return Err(format!("No se puede borrar: {} partidos existen", cnt)); }
    sqlx::query("DELETE FROM league_standings WHERE competition_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM competitions WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_staff(pool: &SqlitePool, club_id: Option<i64>) -> Result<Vec<StaffRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, String, String, i64, String, Option<i64>, Option<String>, i64, i64, i64, i64, i64, i64, f64)>(
        "SELECT st.id, st.first_name, st.last_name, COALESCE(st.common_name, st.first_name || ' ' || st.last_name), n.name, st.nation_id, st.role, st.club_id, cl.name, st.tactical, st.man_management, st.judging, st.motivating, st.working_youngsters, st.physio_level, st.wage_weekly FROM staff st JOIN nations n ON n.id=st.nation_id LEFT JOIN clubs cl ON cl.id=st.club_id WHERE (? IS NULL OR st.club_id=?) ORDER BY st.role, st.last_name"
    ).bind(club_id).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, first_name, last_name, common_name, nation, nation_id, role, club_id, club_name, tactical, man_management, judging, motivating, working_youngsters, physio_level, wage_weekly)| StaffRow { id, first_name, last_name, common_name, nation, nation_id, role, club_id, club_name, tactical, man_management, judging, motivating, working_youngsters, physio_level, wage_weekly }).collect())
}
pub async fn list_coaches(pool: &SqlitePool) -> Result<Vec<StaffRow>, String> {
    let rows = sqlx::query_as::<_, (i64, String, String, String, String, i64, String, Option<i64>, Option<String>, i64, i64, i64, i64, i64, i64, f64)>(
        "SELECT st.id, st.first_name, st.last_name, COALESCE(st.common_name, st.first_name || ' ' || st.last_name), n.name, st.nation_id, st.role, st.club_id, cl.name, st.tactical, st.man_management, st.judging, st.motivating, st.working_youngsters, st.physio_level, st.wage_weekly FROM staff st JOIN nations n ON n.id=st.nation_id LEFT JOIN clubs cl ON cl.id=st.club_id WHERE st.role='coach' ORDER BY st.last_name"
    ).fetch_all(pool).await.map_err(|e| e.to_string())?;
    Ok(rows.into_iter().map(|(id, first_name, last_name, common_name, nation, nation_id, role, club_id, club_name, tactical, man_management, judging, motivating, working_youngsters, physio_level, wage_weekly)| StaffRow { id, first_name, last_name, common_name, nation, nation_id, role, club_id, club_name, tactical, man_management, judging, motivating, working_youngsters, physio_level, wage_weekly }).collect())
}
pub async fn create_staff(pool: &SqlitePool, first: String, last: String, nation_id: i64, role: String, club_id: Option<i64>, tactical: i64, man_management: i64, judging: i64, motivating: i64, working_youngsters: i64, physio_level: i64, wage_weekly: f64) -> Result<i64, String> {
    let (id,): (i64,) = sqlx::query_as("INSERT INTO staff(first_name,last_name,common_name,nation_id,role,club_id,tactical,man_management,judging,motivating,working_youngsters,physio_level,wage_weekly) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?) RETURNING id")
        .bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(nation_id).bind(&role).bind(club_id).bind(tactical).bind(man_management).bind(judging).bind(motivating).bind(working_youngsters).bind(physio_level).bind(wage_weekly)
        .fetch_one(pool).await.map_err(|e| e.to_string())?;
    if role == "coach" {
        if let Some(cid) = club_id { sqlx::query("UPDATE clubs SET coach_id=? WHERE id=?").bind(id).bind(cid).execute(pool).await.ok(); }
    }
    Ok(id)
}
pub async fn update_staff(pool: &SqlitePool, id: i64, first: String, last: String, nation_id: i64, role: String, club_id: Option<i64>, tactical: i64, man_management: i64, judging: i64, motivating: i64, working_youngsters: i64, physio_level: i64, wage_weekly: f64) -> Result<(), String> {
    sqlx::query("UPDATE staff SET first_name=?, last_name=?, common_name=?, nation_id=?, role=?, club_id=?, tactical=?, man_management=?, judging=?, motivating=?, working_youngsters=?, physio_level=?, wage_weekly=? WHERE id=?")
        .bind(&first).bind(&last).bind(format!("{} {}", first, last)).bind(nation_id).bind(&role).bind(club_id).bind(tactical).bind(man_management).bind(judging).bind(motivating).bind(working_youngsters).bind(physio_level).bind(wage_weekly).bind(id)
        .execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn delete_staff(pool: &SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE clubs SET coach_id=NULL WHERE coach_id=?").bind(id).execute(pool).await.ok();
    sqlx::query("DELETE FROM staff WHERE id=?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn set_coach(pool: &SqlitePool, club_id: i64, staff_id: Option<i64>) -> Result<(), String> {
    sqlx::query("UPDATE clubs SET coach_id=? WHERE id=?").bind(staff_id).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(())
}
pub async fn set_crest(pool: &SqlitePool, club_id: i64, data_b64: &str, ext: &str) -> Result<String, String> {
    let bytes = base64_decode(data_b64).ok_or("Imagen no válida (base64)")?;
    let dir = crate::db::app_data_dir().join("crests");
    let _ = std::fs::create_dir_all(&dir);
    let safe_ext = if ["png","jpg","jpeg","webp","gif","svg"].contains(&ext.to_lowercase().as_str()) { ext.to_lowercase() } else { "png".into() };
    let filename = format!("club_{}.{}", club_id, safe_ext);
    let path = dir.join(&filename);
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())?;
    sqlx::query("UPDATE clubs SET crest_path=? WHERE id=?").bind(path.display().to_string()).bind(club_id).execute(pool).await.map_err(|e| e.to_string())?;
    Ok(path.display().to_string())
}

fn base64_decode(s: &str) -> Option<Vec<u8>> {
    use base64::Engine as _;
    let s = s.trim();
    if let Some(i) = s.find(',') { // data URL
        base64::engine::general_purpose::STANDARD.decode(&s[i+1..]).ok()
    } else {
        base64::engine::general_purpose::STANDARD.decode(s).ok()
    }
}
