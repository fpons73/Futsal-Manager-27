pub mod data;
pub mod prd;

use chrono::NaiveDate;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sqlx::SqlitePool;

use data::{BRAZIL_CLUBS, PORTUGAL_CLUBS, SPAIN_CLUBS};

const SEASON: &str = "2026/2027";
const GAME_DATE: &str = "2026-07-10";

pub async fn seed_world(pool: &SqlitePool) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let mut conf_ids: std::collections::HashMap<&str, i64> = Default::default();
    for (name, short, rep) in [("UEFA","UEFA",950),("CONMEBOL","CONMEBOL",900),("AFC","AFC",850),("CAF","CAF",750),("OFC","OFC",600),("CONCACAF","CONCACAF",650)] {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO confederations(name,short_name,reputation) VALUES(?,?,?) RETURNING id")
            .bind(name).bind(short).bind(rep)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        conf_ids.insert(short, id);
    }

    let mut nation_ids: std::collections::HashMap<String, i64> = Default::default();
    for (name, conf, rep, level) in prd::ALL_NATIONS {
        let cid = *conf_ids.get(conf).unwrap();
        let (id,): (i64,) = sqlx::query_as("INSERT INTO nations(name,confederation_id,reputation,futsal_level) VALUES(?,?,?,?) RETURNING id")
            .bind(*name).bind(cid).bind(*rep).bind(*level)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        nation_ids.insert(name.to_string(), id);
    }

    let mut city_ids: std::collections::HashMap<String, i64> = Default::default();
    let mut all_cities: Vec<(&str, String)> = Vec::new();
    for c in SPAIN_CLUBS { all_cities.push((c.city, "España".to_string())); }
    for c in BRAZIL_CLUBS { all_cities.push((c.city, "Brasil".to_string())); }
    for c in PORTUGAL_CLUBS { all_cities.push((c.city, "Portugal".to_string())); }
    let mut seen = std::collections::HashSet::new();
    for (city, nat) in all_cities {
        if seen.contains(city) { continue; }
        seen.insert(city.to_string());
        if let Some(&nid) = nation_ids.get(&nat) {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name,nation_id,population) VALUES(?,?,500000) RETURNING id")
                .bind(city).bind(nid)
                .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
            city_ids.insert(city.to_string(), id);
        }
    }
    for (nation, _, _, _) in prd::ALL_NATIONS {
        let cap_name = format!("{} Capital", nation);
        if !city_ids.contains_key(&cap_name) {
            if let Some(&nid) = nation_ids.get(*nation) {
                let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name,nation_id,population) VALUES(?,?,300000) RETURNING id")
                    .bind(&cap_name).bind(nid).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
                city_ids.insert(cap_name, id);
            }
        }
    }

    let mut comp_ids: Vec<i64> = Vec::new();
    for comp in prd::ALL_COMPS {
        let nid = comp.nation.and_then(|n| nation_ids.get(n).copied());
        let kind = if comp.nation.is_none() { "national_team" } else { "club" };
        let (id,): (i64,) = sqlx::query_as("INSERT INTO competitions(name,nation_id,tier,total_teams,season,format,kind) VALUES(?,?,?,?,?,?,?) RETURNING id")
            .bind(comp.name).bind(nid).bind(comp.tier).bind(comp.teams).bind(SEASON).bind(if comp.tier.is_some() { "Round Robin" } else { "Cup" }).bind(kind)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        comp_ids.push(id);
    }

    let mut rng = StdRng::from_entropy();
    let base_date = NaiveDate::parse_from_str(GAME_DATE, "%Y-%m-%d").unwrap();

    // Mapear nación -> clubes a crear (solo para ligas, no copas)
    // Para simplificar, creamos clubes para cada nación que tiene al menos una liga (tier Some)
    // El número de clubes por nación = máximo total_teams de sus ligas
    let mut nation_max_teams: std::collections::HashMap<String, i64> = Default::default();
    for comp in prd::ALL_COMPS {
        if let Some(nation) = comp.nation {
            if comp.tier.is_some() {
                let entry = nation_max_teams.entry(nation.to_string()).or_insert(0);
                *entry = (*entry).max(comp.teams);
            }
        }
    }

    struct OwnedClub { name: String, short: String, city: String, stadium: String, capacity: i64, rep: i64, c1: String, c2: String, nid: i64, nation: String }
    let mut clubs_to_create: Vec<OwnedClub> = Vec::new();
    for c in SPAIN_CLUBS {
        if let Some(&nid) = nation_ids.get("España") {
            clubs_to_create.push(OwnedClub { name: c.name.into(), short: c.short.into(), city: c.city.into(), stadium: c.stadium.into(), capacity: c.capacity, rep: c.reputation, c1: c.color.into(), c2: c.color2.into(), nid, nation: "España".into() });
        }
    }
    for c in BRAZIL_CLUBS {
        if let Some(&nid) = nation_ids.get("Brasil") {
            clubs_to_create.push(OwnedClub { name: c.name.into(), short: c.short.into(), city: c.city.into(), stadium: c.stadium.into(), capacity: c.capacity, rep: c.reputation, c1: c.color.into(), c2: c.color2.into(), nid, nation: "Brasil".into() });
        }
    }
    for c in PORTUGAL_CLUBS {
        if let Some(&nid) = nation_ids.get("Portugal") {
            clubs_to_create.push(OwnedClub { name: c.name.into(), short: c.short.into(), city: c.city.into(), stadium: c.stadium.into(), capacity: c.capacity, rep: c.reputation, c1: c.color.into(), c2: c.color2.into(), nid, nation: "Portugal".into() });
        }
    }
    for (nation, max_teams) in &nation_max_teams {
        if ["España","Brasil","Portugal"].contains(&nation.as_str()) { continue; }
        let nid = match nation_ids.get(nation.as_str()) { Some(v) => *v, None => continue };
        let count = *max_teams as usize;
        for i in 0..count {
            let rep = (720 - (i as i64 * 12)).max(500);
            clubs_to_create.push(OwnedClub {
                name: format!("{} Futsal {}", nation, i+1),
                short: format!("{}{}", nation[..2.min(nation.len())].to_uppercase(), i+1),
                city: format!("{} Capital", nation),
                stadium: format!("{} Arena {}", nation, i+1),
                capacity: 1500 + (rep % 3000), rep, c1: "#0f4c3a".into(), c2: "#ffffff".into(), nid, nation: nation.clone(),
            });
        }
    }

    let mut club_ids: Vec<i64> = Vec::new();
    let mut club_nation: Vec<String> = Vec::new();

    for oc in clubs_to_create {
        let city_id = city_ids.get(&oc.city).copied().unwrap_or_else(|| *city_ids.values().next().unwrap());
        let (stadium_id,): (i64,) = sqlx::query_as("INSERT INTO stadiums(name,city_id,capacity,pitch_type) VALUES(?,?,?, 'parquet') RETURNING id")
            .bind(&oc.stadium).bind(city_id).bind(oc.capacity)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        let (club_id,): (i64,) = sqlx::query_as("INSERT INTO clubs(name,short_name,nation_id,city_id,stadium_id,reputation,primary_color,secondary_color) VALUES(?,?,?,?,?,?,?,?) RETURNING id")
            .bind(&oc.name).bind(&oc.short).bind(oc.nid).bind(city_id).bind(stadium_id).bind(oc.rep).bind(&oc.c1).bind(&oc.c2)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        club_ids.push(club_id);
        club_nation.push(oc.nation.clone());
        let balance = (oc.rep as f64) * 1800.0 + rng.gen_range(50_000.0..250_000.0);
        let wage_budget = (oc.rep as f64) * 12.0 + 2000.0;
        sqlx::query("INSERT INTO club_finances(club_id,balance,transfer_budget,wage_budget,total_wages) VALUES(?,?,?,?,0)").bind(club_id).bind(balance).bind(balance*0.25).bind(wage_budget).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        let formations = ["3-1","4-0","2-2"];
        let f = formations[rng.gen_range(0..formations.len())];
        sqlx::query("INSERT INTO tactics(club_id,formation,tempo,pressing,defensive_line,width,playing_style,powerplay_enabled) VALUES(?,?,?,?,?,?,?,1)").bind(club_id).bind(f).bind(rng.gen_range(40..75) as i64).bind(rng.gen_range(40..80) as i64).bind(rng.gen_range(35..70) as i64).bind(rng.gen_range(40..70) as i64).bind(if rng.gen_bool(0.5) {"balanced"}else{"counter"}).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        let nat_for_names: &str = if ["España","Brasil","Portugal"].contains(&oc.nation.as_str()) { &oc.nation } else { "España" };
        generate_squad(&mut tx, club_id, oc.nid, nat_for_names, oc.rep, base_date, &mut rng).await?;
        for (day, type_id, intensity) in [(0,1,70),(1,2,75),(2,4,65),(3,3,75),(4,7,60)] {
            sqlx::query("INSERT OR IGNORE INTO training_schedule(club_id, day_of_week, training_type_id, intensity) VALUES(?,?,?,?)").bind(club_id).bind(day).bind(type_id).bind(intensity).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        }
        sqlx::query("INSERT OR IGNORE INTO youth_academy(club_id, level) VALUES(?,50)").bind(club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }

    // Crear standings solo para ligas (tier Some)
    // Necesitamos mapear comp -> clubes de esa nación
    // Para cada liga, tomar N clubes de esa nación (los más reputados)
    let mut nation_clubs: std::collections::HashMap<String, Vec<i64>> = Default::default();
    for (idx, cid) in club_ids.iter().enumerate() {
        let nat = &club_nation[idx];
        nation_clubs.entry(nat.clone()).or_default().push(*cid);
    }
    for comp in prd::ALL_COMPS {
        if comp.tier.is_none() { continue; }
        let nation = match comp.nation { Some(n) => n, None => continue };
        let clubs_for_nation = match nation_clubs.get(nation) { Some(v) => v, None => continue };
        let take = (comp.teams as usize).min(clubs_for_nation.len());
        let selected: Vec<i64> = clubs_for_nation.iter().take(take).copied().collect();
        let comp_id: i64 = sqlx::query_as::<_, (i64,)>("SELECT id FROM competitions WHERE name=? AND season=?").bind(comp.name).bind(SEASON).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?.0;
        for &club in &selected {
            sqlx::query("INSERT INTO league_standings(competition_id,season,club_id,position,played,won,drawn,lost,goals_for,goals_against,goal_difference,points,form_last_5) VALUES(?,?,?,?,0,0,0,0,0,0,0,0,'')").bind(comp_id).bind(SEASON).bind(club).bind(0).execute(&mut *tx).await.map_err(|e| e.to_string())?;
        }
    }

    sqlx::query("INSERT INTO game_state(id, game_date, season, game_speed) VALUES(1, ?, ?, 'normal')").bind(GAME_DATE).bind(SEASON).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    crate::competition::generate_calendars(pool).await?;
    Ok(())
}

async fn generate_squad(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    club_id: i64,
    nation_id: i64,
    nation_name: &str,
    reputation: i64,
    base_date: NaiveDate,
    rng: &mut impl Rng,
) -> Result<(), String> {
    let roles: &[&str] = &["POR","POR","CIE","CIE","ALA","ALA","ALA","ALA","PIV","PIV","UNI","UNI"];
    for (idx, role) in roles.iter().enumerate() {
        let age: i64 = if idx < 2 { rng.gen_range(22..36) } else if idx < 8 { rng.gen_range(19..32) } else { rng.gen_range(20..34) };
        let dob = base_date - chrono::Duration::days(age * 365 + rng.gen_range(0..365) as i64);
        let dob_s = dob.format("%Y-%m-%d").to_string();
        let first = data::pick_first(nation_name, rng);
        let last = data::pick_last(nation_name, rng);
        let foot = ["right","left","both"][rng.gen_range(0..3)];
        let height = rng.gen_range(168..195);
        let weight = rng.gen_range(65..92);

        let (ca, pa) = gen_ca_pa(reputation, age, rng);
        let attrs = gen_attributes(role, ca, rng);

        let (pid,): (i64,) = sqlx::query_as(
            "INSERT INTO players(first_name,last_name,common_name,date_of_birth,nation_id,preferred_foot,height_cm,weight_kg) VALUES(?,?,?,?,?,?,?,?) RETURNING id"
        )
        .bind(first).bind(last).bind(format!("{first} {last}")).bind(&dob_s).bind(nation_id).bind(foot).bind(height).bind(weight)
        .fetch_one(&mut **tx).await.map_err(|e| e.to_string())?;

        let (por, cie, ala, piv, uni) = match *role {
            "POR" => (20, 2, 1, 1, 3),
            "CIE" => (1, 20, 12, 8, 10),
            "ALA" => (1, 10, 20, 10, 14),
            "PIV" => (1, 6, 10, 20, 12),
            _ => (3, 10, 14, 14, 20),
        };
        sqlx::query("INSERT INTO player_positions(player_id,por_natural,cie_natural,ala_natural,piv_natural,uni_natural) VALUES(?,?,?,?,?,?)")
            .bind(pid).bind(por).bind(cie).bind(ala).bind(piv).bind(uni)
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        sqlx::query("INSERT INTO player_states(player_id,current_ability,potential_ability,condition_val,match_fitness,morale,sharpness,happiness) VALUES(?,?,?,?,?,?,?,?)")
            .bind(pid).bind(ca).bind(pa).bind(100).bind(rng.gen_range(85..100)).bind(rng.gen_range(60..90)).bind(rng.gen_range(40..80)).bind(rng.gen_range(60..90))
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO player_attributes(player_id,first_touch,dribbling,ball_control,technique,passing,vision,crossing,long_shots,finishing,heading,penalty_taking,tackling,marking,interception,blocking,anticipation,decisions,positioning,off_the_ball,work_rate,composure,concentration,determination,bravery,aggression,leadership,teamwork,flair,acceleration,pace,agility,balance,stamina,strength,jumping,reflexes,handling,one_on_ones,positioning_gk,rushing_out,throwing,kicking,professionalism,consistency,important_matches,injury_proneness) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"
        )
        .bind(pid)
        .bind(attrs[0]).bind(attrs[1]).bind(attrs[2]).bind(attrs[3]).bind(attrs[4]).bind(attrs[5]).bind(attrs[6]).bind(attrs[7]).bind(attrs[8]).bind(attrs[9]).bind(attrs[10]).bind(attrs[11]).bind(attrs[12]).bind(attrs[13]).bind(attrs[14]).bind(attrs[15]).bind(attrs[16]).bind(attrs[17]).bind(attrs[18]).bind(attrs[19]).bind(attrs[20]).bind(attrs[21]).bind(attrs[22]).bind(attrs[23]).bind(attrs[24]).bind(attrs[25]).bind(attrs[26]).bind(attrs[27]).bind(attrs[28]).bind(attrs[29]).bind(attrs[30]).bind(attrs[31]).bind(attrs[32]).bind(attrs[33]).bind(attrs[34]).bind(attrs[35]).bind(attrs[36]).bind(attrs[37]).bind(attrs[38]).bind(attrs[39]).bind(attrs[40]).bind(attrs[41]).bind(attrs[42]).bind(attrs[43]).bind(attrs[44]).bind(attrs[45])
        .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        let wage = (ca as f64 * 18.0 + rng.gen_range(0.0..400.0)).round();
        let years = rng.gen_range(1..4);
        let end = base_date + chrono::Duration::days(years * 365);
        sqlx::query("INSERT INTO contracts(player_id,club_id,wage_weekly,start_date,end_date,is_active) VALUES(?,?,?,?,?,1)")
            .bind(pid).bind(club_id).bind(wage).bind(GAME_DATE).bind(end.format("%Y-%m-%d").to_string())
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;

        let total: f64 = sqlx::query_scalar::<_, Option<f64>>("SELECT SUM(wage_weekly) FROM contracts WHERE club_id=? AND is_active=1")
            .bind(club_id).fetch_one(&mut **tx).await.map_err(|e| e.to_string())?.unwrap_or(0.0);
        sqlx::query("UPDATE club_finances SET total_wages=? WHERE club_id=?").bind(total).bind(club_id)
            .execute(&mut **tx).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn gen_ca_pa(rep: i64, age: i64, rng: &mut impl Rng) -> (i64, i64) {
    let base = 62 + (rep - 520) * 45 / 420;
    let mut ca = (base as f64 + rng.gen_range(-8.0..10.0)).round() as i64;
    ca = ca.clamp(45, 185);
    if age < 21 { ca = (ca as f64 * 0.72) as i64; }
    else if age < 24 { ca = (ca as f64 * 0.86) as i64; }
    else if age > 32 { ca = (ca as f64 * 0.92) as i64; }
    ca = ca.clamp(40, 190);
    let gap = if age <= 20 { rng.gen_range(18..50) } else if age <= 24 { rng.gen_range(8..32) } else if age <= 28 { rng.gen_range(2..16) } else { rng.gen_range(0..6) };
    let mut pa = ca + gap;
    pa = pa.clamp(ca, 200);
    (ca, pa)
}

fn gen_attributes(role: &str, ca: i64, rng: &mut impl Rng) -> Vec<i64> {
    let base = (ca / 10).clamp(4, 18) as f64;
    let mut v = Vec::with_capacity(46);
    let bonuses: Vec<f64> = match role {
        "POR" => vec![0.,0.,0.,0., 0.,0.,0.,0., -3.,-2.,0., -3.,-3.,-2.,-2., -1.,-1.,-1.,-2.,-1., -1.,0.,0.,0.,0.,0.,0., -2., -1.,-1.,-1.,-1.,0.,-1.,0., 5.,5.,5.,5.,5.,5.,5., 0.,0.,0.,0.],
        "CIE" => vec![0.,-1.,0.,0., 1.,1.,-1.,-1., -2.,0.,0., 4.,4.,4.,4., 3.,2.,3.,0.,2., 1.,1.,1.,1.,1.,2.,1., -1., 0.,0.,1.,1.,2.,2.,0., -4.,-4.,-4.,-4.,-4.,-4.,-4., 0.,0.,0.,0.],
        "ALA" => vec![1.,3.,2.,2., 1.,1.,1.,0., 0.,0.,0., -1.,-1.,0.,-1., 0.,0.,0.,1.,1., 0.,0.,0.,0.,0.,0.,0., 2., 3.,3.,3.,2.,1.,0.,0., -4.,-4.,-4.,-4.,-4.,-4.,-4., 0.,0.,0.,0.],
        "PIV" => vec![1.,0.,2.,1., 0.,0.,-1.,0., 4.,2.,1., -2.,-2.,-1.,0., 0.,1.,0.,2.,1., 1.,1.,0.,0.,1.,0.,0., 0., -1.,-1.,0.,1.,1.,3.,1., -4.,-4.,-4.,-4.,-4.,-4.,-4., 0.,0.,0.,0.],
        _     => vec![0.,1.,1.,1., 1.,1.,0.,0., 1.,0.,0., 0.,0.,0.,0., 1.,1.,1.,1.,1., 0.,0.,0.,0.,0.,0.,0., 1., 1.,1.,1.,1.,1.,1.,0., -2.,-2.,-2.,-2.,-2.,-2.,-2., 0.,0.,0.,0.],
    };
    for i in 0..46 {
        let b = bonuses.get(i).copied().unwrap_or(0.0);
        let mut val = base + b + rng.gen_range(-2.5..2.5);
        val = val.clamp(1.0, 20.0);
        v.push(val.round() as i64);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[tokio::test]
    async fn seed_creates_expected_counts() {
        let pool = db::init_memory_pool().await.unwrap();
        seed_world(&pool).await.unwrap();
        let (clubs,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM clubs").fetch_one(&pool).await.unwrap();
        let (players,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM players").fetch_one(&pool).await.unwrap();
        let (comps,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM competitions").fetch_one(&pool).await.unwrap();
        let (stadiums,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stadiums").fetch_one(&pool).await.unwrap();
        assert!(clubs >= 46, "al menos 46 clubes, got {}", clubs);
        assert_eq!(players, clubs * 12, "12 jugadores por club");
        assert_eq!(comps, 43, "43 competiciones del PRD (ligas, copas, 2ª división y selecciones)");
        assert_eq!(stadiums, clubs);
        let (standings,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings").fetch_one(&pool).await.unwrap();
        assert!(standings >= 46, "al menos 46 standings");
        let (fin,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM club_finances").fetch_one(&pool).await.unwrap();
        assert_eq!(fin, clubs);
        let (contracts,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts").fetch_one(&pool).await.unwrap();
        assert_eq!(contracts, players);
        let (matches,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches").fetch_one(&pool).await.unwrap();
        assert!(matches >= 662, "al menos 662 partidos, got {}", matches);
        let (d0,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.unwrap();
        assert_eq!(d0, "2026-07-10");
    }
}

