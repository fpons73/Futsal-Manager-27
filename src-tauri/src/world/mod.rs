pub mod data;

use chrono::NaiveDate;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sqlx::SqlitePool;

use data::{BRAZIL_CLUBS, PORTUGAL_CLUBS, SPAIN_CLUBS};

const SEASON: &str = "2026/2027";
const GAME_DATE: &str = "2026-07-10";

#[derive(Clone, Copy)]
struct NationInfo {
    name: &'static str,
    conf: &'static str,
    level: i64,
    rep: i64,
}

const NATIONS: &[NationInfo] = &[
    NationInfo { name: "España", conf: "UEFA", level: 92, rep: 900 },
    NationInfo { name: "Brasil", conf: "CONMEBOL", level: 95, rep: 950 },
    NationInfo { name: "Portugal", conf: "UEFA", level: 88, rep: 850 },
];

pub async fn seed_world(pool: &SqlitePool) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let uefa_id: i64 = {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO confederations(name,short_name,reputation) VALUES('UEFA','UEFA',950) RETURNING id")
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        id
    };
    let conmebol_id: i64 = {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO confederations(name,short_name,reputation) VALUES('CONMEBOL','CONMEBOL',900) RETURNING id")
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        id
    };

    let mut nation_ids: std::collections::HashMap<&str, i64> = Default::default();
    for n in NATIONS {
        let cid = if n.conf == "UEFA" { uefa_id } else { conmebol_id };
        let (id,): (i64,) = sqlx::query_as("INSERT INTO nations(name,confederation_id,reputation,futsal_level) VALUES(?,?,?,?) RETURNING id")
            .bind(n.name).bind(cid).bind(n.rep).bind(n.level)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        nation_ids.insert(n.name, id);
    }

    let mut city_ids: std::collections::HashMap<String, i64> = Default::default();
    let mut all_cities: Vec<(&str, &str)> = Vec::new();
    for c in SPAIN_CLUBS { all_cities.push((c.city, "España")); }
    for c in BRAZIL_CLUBS { all_cities.push((c.city, "Brasil")); }
    for c in PORTUGAL_CLUBS { all_cities.push((c.city, "Portugal")); }
    let mut seen = std::collections::HashSet::new();
    for (city, nat) in all_cities {
        if seen.contains(city) { continue; }
        seen.insert(city);
        let nid = *nation_ids.get(nat).unwrap();
        let (id,): (i64,) = sqlx::query_as("INSERT INTO cities(name,nation_id,population) VALUES(?,?,500000) RETURNING id")
            .bind(city).bind(nid)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        city_ids.insert(city.to_string(), id);
    }

    let esp_id = *nation_ids.get("España").unwrap();
    let bra_id = *nation_ids.get("Brasil").unwrap();
    let por_id = *nation_ids.get("Portugal").unwrap();

    let comp_esp: i64 = {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO competitions(name,nation_id,tier,total_teams,season,format) VALUES('Primera Division de Futbol Sala',?,?,16,?, 'Round Robin') RETURNING id")
            .bind(esp_id).bind(1).bind(SEASON)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        id
    };
    let comp_bra: i64 = {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO competitions(name,nation_id,tier,total_teams,season,format) VALUES('Liga Nacional de Futsal (LNF)',?,?,16,?, 'Round Robin') RETURNING id")
            .bind(bra_id).bind(1).bind(SEASON)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        id
    };
    let comp_por: i64 = {
        let (id,): (i64,) = sqlx::query_as("INSERT INTO competitions(name,nation_id,tier,total_teams,season,format) VALUES('Liga Placard',?,?,14,?, 'Round Robin') RETURNING id")
            .bind(por_id).bind(1).bind(SEASON)
            .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        id
    };

    let mut rng = StdRng::from_entropy();
    let base_date = NaiveDate::parse_from_str(GAME_DATE, "%Y-%m-%d").unwrap();

    let mut club_defs: Vec<(&data::ClubDef, i64)> = Vec::new();
    for c in SPAIN_CLUBS { club_defs.push((c, esp_id)); }
    for c in BRAZIL_CLUBS { club_defs.push((c, bra_id)); }
    for c in PORTUGAL_CLUBS { club_defs.push((c, por_id)); }

    let mut club_ids: Vec<i64> = Vec::new();

    for (def, nid) in club_defs {
        let city_id = *city_ids.get(def.city).unwrap_or(&city_ids.values().next().copied().unwrap());
        let stadium_id: i64 = {
            let (id,): (i64,) = sqlx::query_as("INSERT INTO stadiums(name,city_id,capacity,pitch_type) VALUES(?,?,?, 'parquet') RETURNING id")
                .bind(def.stadium).bind(city_id).bind(def.capacity)
                .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
            id
        };
        let (club_id,): (i64,) = sqlx::query_as(
            "INSERT INTO clubs(name,short_name,nation_id,city_id,stadium_id,reputation,primary_color,secondary_color) VALUES(?,?,?,?,?,?,?,?) RETURNING id"
        )
        .bind(def.name).bind(def.short).bind(nid).bind(city_id).bind(stadium_id).bind(def.reputation).bind(def.color).bind(def.color2)
        .fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        club_ids.push(club_id);

        let balance = (def.reputation as f64) * 1800.0 + rng.gen_range(50_000.0..250_000.0);
        let wage_budget = (def.reputation as f64) * 12.0 + 2000.0;
        sqlx::query("INSERT INTO club_finances(club_id,balance,transfer_budget,wage_budget,total_wages) VALUES(?,?,?,?,0)")
            .bind(club_id).bind(balance).bind(balance * 0.25).bind(wage_budget)
            .execute(&mut *tx).await.map_err(|e| e.to_string())?;

        let formations = ["3-1","4-0","2-2"];
        let f = formations[rng.gen_range(0..formations.len())];
        sqlx::query("INSERT INTO tactics(club_id,formation,tempo,pressing,defensive_line,width,playing_style,powerplay_enabled) VALUES(?,?,?,?,?,?,?,1)")
            .bind(club_id).bind(f)
            .bind(rng.gen_range(40..75) as i64)
            .bind(rng.gen_range(40..80) as i64)
            .bind(rng.gen_range(35..70) as i64)
            .bind(rng.gen_range(40..70) as i64)
            .bind(if rng.gen_bool(0.5) { "balanced" } else { "counter" })
            .execute(&mut *tx).await.map_err(|e| e.to_string())?;

        let nat_name = if nid == esp_id { "España" } else if nid == bra_id { "Brasil" } else { "Portugal" };
        generate_squad(&mut tx, club_id, nid, nat_name, def.reputation, base_date, &mut rng).await?;
        for (day, type_id, intensity) in [(0,1,70),(1,2,75),(2,4,65),(3,3,75),(4,7,60)] {
            sqlx::query("INSERT OR IGNORE INTO training_schedule(club_id, day_of_week, training_type_id, intensity) VALUES(?,?,?,?)")
                .bind(club_id).bind(day).bind(type_id).bind(intensity)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
        }
        sqlx::query("INSERT OR IGNORE INTO youth_academy(club_id, level) VALUES(?,50)").bind(club_id).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }

    for &cid in &[comp_esp, comp_bra, comp_por] {
        let clubs_in_comp: Vec<i64> = if cid == comp_por {
            club_ids.iter().filter(|&&id| {
                let idx = club_ids.iter().position(|&x| x == id).unwrap();
                idx >= 32
            }).copied().collect()
        } else if cid == comp_bra {
            club_ids.iter().skip(16).take(16).copied().collect()
        } else {
            club_ids.iter().take(16).copied().collect()
        };
        for &club in &clubs_in_comp {
            sqlx::query("INSERT INTO league_standings(competition_id,season,club_id,position,played,won,drawn,lost,goals_for,goals_against,goal_difference,points,form_last_5) VALUES(?,?,?,?,0,0,0,0,0,0,0,0,'')")
                .bind(cid).bind(SEASON).bind(club).bind(0)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
        }
    }

    sqlx::query("INSERT INTO game_state(id, game_date, season, game_speed) VALUES(1, ?, ?, 'normal')")
        .bind(GAME_DATE)
        .bind(SEASON)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

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
        assert_eq!(clubs, 46, "46 clubes (16+16+14)");
        assert_eq!(players, 552, "46*12 jugadores");
        assert_eq!(comps, 3);
        assert_eq!(stadiums, 46);
        let (standings,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_standings").fetch_one(&pool).await.unwrap();
        assert_eq!(standings, 46);
        let (fin,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM club_finances").fetch_one(&pool).await.unwrap();
        assert_eq!(fin, 46);
        let (contracts,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contracts").fetch_one(&pool).await.unwrap();
        assert_eq!(contracts, 552);
        let (matches,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM matches").fetch_one(&pool).await.unwrap();
        assert_eq!(matches, 662, "662 partidos doble robin (240+240+182)");
        let (d0,): (String,) = sqlx::query_as("SELECT game_date FROM game_state WHERE id=1").fetch_one(&pool).await.unwrap();
        assert_eq!(d0, "2026-07-10");
    }
}
