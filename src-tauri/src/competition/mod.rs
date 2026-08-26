use chrono::NaiveDate;
use sqlx::SqlitePool;

pub fn build_round_robin(team_ids: &[i64]) -> Vec<Vec<(i64, i64)>> {
    let n = team_ids.len();
    let mut teams = team_ids.to_vec();
    let is_odd = n % 2 == 1;
    if is_odd {
        teams.push(-1);
    }
    let m = teams.len();
    let rounds = m - 1;
    let mut result: Vec<Vec<(i64, i64)>> = Vec::with_capacity(rounds * 2);
    let mut current = teams.clone();
    for _ in 0..rounds {
        let mut round: Vec<(i64, i64)> = Vec::new();
        for i in 0..(m / 2) {
            let a = current[i];
            let b = current[m - 1 - i];
            if a != -1 && b != -1 {
                round.push((a, b));
            }
        }
        result.push(round);
        let last = current.pop().unwrap();
        current.insert(1, last);
    }
    let first_leg = result.clone();
    let mut second_leg: Vec<Vec<(i64, i64)>> = Vec::new();
    for round in first_leg.iter() {
        second_leg.push(round.iter().map(|(a, b)| (*b, *a)).collect());
    }
    result.extend(second_leg);
    result
}

pub async fn generate_calendars(pool: &SqlitePool) -> Result<(), String> {
    let comps: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, season FROM competitions")
            .fetch_all(pool).await.map_err(|e| e.to_string())?;

    let start = NaiveDate::from_ymd_opt(2026, 8, 22).unwrap();

    for (comp_id, season) in comps {
        let club_rows: Vec<(i64,)> =
            sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id=? ORDER BY club_id")
                .bind(comp_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
        let team_ids: Vec<i64> = club_rows.into_iter().map(|(id,)| id).collect();
        if team_ids.is_empty() { continue; }

        let rounds = build_round_robin(&team_ids);

        let existing: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=?")
            .bind(comp_id).fetch_one(pool).await.map_err(|e| e.to_string())?;
        if existing.0 > 0 { continue; }

        for (idx, round) in rounds.iter().enumerate() {
            let date = start + chrono::Duration::days(idx as i64 * 7);
            let date_s = date.format("%Y-%m-%d").to_string();
            let round_no = (idx + 1) as i64;
            for (home, away) in round {
                let stadium: Option<(i64,)> =
                    sqlx::query_as("SELECT stadium_id FROM clubs WHERE id=?")
                        .bind(home).fetch_optional(pool).await.map_err(|e| e.to_string())?;
                let sid = stadium.and_then(|(s,)| Some(s));
                sqlx::query("INSERT INTO matches(competition_id,season,round,date,home_club_id,away_club_id,stadium_id,status) VALUES(?,?,?,?,?,?,?, 'scheduled')")
                    .bind(comp_id).bind(&season).bind(round_no).bind(&date_s).bind(home).bind(away).bind(sid)
                    .execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::world;

    #[tokio::test]
    async fn calendar_counts_and_balance() {
        let pool = db::init_memory_pool().await.unwrap();
        world::seed_world(&pool).await.unwrap();
        generate_calendars(&pool).await.unwrap();

        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches").fetch_one(&pool).await.unwrap();
        assert_eq!(total, 240 + 240 + 182, "662 partidos esperados (doble robin)");

        for (comp_name, n) in [("Primera Division de Futbol Sala", 16), ("Liga Nacional de Futsal (LNF)", 16), ("Liga Placard", 14)] {
            let (comp_id,): (i64,) = sqlx::query_as("SELECT id FROM competitions WHERE name=?").bind(comp_name).fetch_one(&pool).await.unwrap();
            let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=?").bind(comp_id).fetch_one(&pool).await.unwrap();
            let expected = (n as i64) * ((n as i64) - 1);
            assert_eq!(cnt, expected, "{comp_name} partidos");

            let clubs: Vec<(i64,)> = sqlx::query_as("SELECT club_id FROM league_standings WHERE competition_id=?").bind(comp_id).fetch_all(&pool).await.unwrap();
            for (cid,) in clubs {
                let (played,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches WHERE competition_id=? AND (home_club_id=? OR away_club_id=?)")
                    .bind(comp_id).bind(cid).bind(cid).fetch_one(&pool).await.unwrap();
                assert_eq!(played, (n as i64 - 1) * 2, "club {cid} debe jugar {} partidos", (n - 1) * 2);
            }
        }

        let (distinct_rounds,): (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT round) FROM matches WHERE competition_id=(SELECT id FROM competitions WHERE name='Primera Division de Futbol Sala')")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(distinct_rounds, 30);
    }

    #[test]
    fn builder_pure_logic() {
        let teams: Vec<i64> = (1..=4).collect();
        let rounds = build_round_robin(&teams);
        assert_eq!(rounds.len(), 6);
        for r in &rounds { assert_eq!(r.len(), 2); }
        let all: Vec<(i64,i64)> = rounds.iter().flatten().copied().collect();
        for &t in &teams {
            let cnt = all.iter().filter(|(a,b)| *a==t || *b==t).count();
            assert_eq!(cnt, 6);
        }
    }
}
