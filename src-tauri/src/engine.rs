use rand::rngs::StdRng;
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FutsalRules {
    pub half_seconds: u32,
    pub half_time_seconds: u32,
    pub total_seconds: u32,
    pub fouls_for_double: u8,
    pub timeouts_per_half: u8,
    pub kick_in_seconds: u8,
}

impl Default for FutsalRules {
    fn default() -> Self {
        Self {
            half_seconds: 20 * 60,
            half_time_seconds: 10 * 60,
            total_seconds: 40 * 60,
            fouls_for_double: 6,
            timeouts_per_half: 1,
            kick_in_seconds: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Role {
    POR,
    CIE,
    ALA,
    PIV,
    UNI,
}

impl Role {
    pub fn from_str(s: &str) -> Self {
        match s {
            "POR" => Role::POR,
            "CIE" => Role::CIE,
            "ALA" => Role::ALA,
            "PIV" => Role::PIV,
            _ => Role::UNI,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerAttrs {
    pub passing: f32,
    pub finishing: f32,
    pub dribbling: f32,
    pub tackling: f32,
    pub vision: f32,
    pub anticipation: f32,
    pub positioning: f32,
    pub stamina: f32,
    pub acceleration: f32,
    pub pace: f32,
    pub composure: f32,
    pub technique: f32,
    pub reflexes: f32,
}

impl PlayerAttrs {
    pub fn average(ca: i64, role: Role) -> Self {
        let base = (ca as f32 / 10.0).clamp(4.0, 18.0);
        let mut rng = StdRng::from_entropy();
        let jitter = |rng: &mut StdRng| rng.gen_range(-1.5..1.5);
        let mut mk = |bonus: f32| (base + bonus + jitter(&mut rng)).clamp(1.0, 20.0);
        match role {
            Role::POR => Self {
                passing: mk(-1.0), finishing: mk(-3.0), dribbling: mk(-2.0),
                tackling: mk(-2.0), vision: mk(0.0), anticipation: mk(1.0),
                positioning: mk(2.0), stamina: mk(0.0), acceleration: mk(0.0),
                pace: mk(0.0), composure: mk(1.0), technique: mk(0.0), reflexes: mk(4.0),
            },
            Role::CIE => Self {
                passing: mk(1.0), finishing: mk(-1.5), dribbling: mk(0.0),
                tackling: mk(3.0), vision: mk(1.0), anticipation: mk(2.0),
                positioning: mk(3.0), stamina: mk(1.0), acceleration: mk(0.5),
                pace: mk(0.5), composure: mk(0.5), technique: mk(0.5), reflexes: mk(-4.0),
            },
            Role::ALA => Self {
                passing: mk(1.0), finishing: mk(0.5), dribbling: mk(2.5),
                tackling: mk(-1.0), vision: mk(1.0), anticipation: mk(0.5),
                positioning: mk(0.0), stamina: mk(1.0), acceleration: mk(2.0),
                pace: mk(2.0), composure: mk(0.5), technique: mk(1.5), reflexes: mk(-4.0),
            },
            Role::PIV => Self {
                passing: mk(0.0), finishing: mk(3.0), dribbling: mk(0.5),
                tackling: mk(-2.0), vision: mk(0.0), anticipation: mk(0.5),
                positioning: mk(0.5), stamina: mk(0.5), acceleration: mk(0.0),
                pace: mk(0.0), composure: mk(1.5), technique: mk(1.5), reflexes: mk(-4.0),
            },
            Role::UNI => Self {
                passing: mk(1.0), finishing: mk(0.5), dribbling: mk(1.0),
                tackling: mk(0.5), vision: mk(1.0), anticipation: mk(1.0),
                positioning: mk(1.0), stamina: mk(1.0), acceleration: mk(1.0),
                pace: mk(1.0), composure: mk(0.5), technique: mk(1.0), reflexes: mk(-2.0),
            },
        }
    }
    pub fn from_ints(passing: i64, finishing: i64, dribbling: i64, tackling: i64, vision: i64, anticipation: i64, positioning: i64, stamina: i64, acceleration: i64, pace: i64, composure: i64, technique: i64, reflexes: i64) -> Self {
        Self {
            passing: passing as f32, finishing: finishing as f32, dribbling: dribbling as f32,
            tackling: tackling as f32, vision: vision as f32, anticipation: anticipation as f32,
            positioning: positioning as f32, stamina: stamina as f32, acceleration: acceleration as f32,
            pace: pace as f32, composure: composure as f32, technique: technique as f32, reflexes: reflexes as f32,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EnginePlayer {
    pub id: u32,
    pub team_id: u32,
    pub shirt: u8,
    pub role: Role,
    pub attrs: PlayerAttrs,
    pub x: f32,
    pub y: f32,
    pub stamina_now: f32,
    pub on_pitch: bool,
    pub is_gk: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchEvent {
    pub minute: u32,
    pub second: u32,
    pub kind: String,
    pub team_id: u32,
    pub player_id: Option<u32>,
    pub description: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerSnapshot {
    pub id: u32,
    pub team_id: u32,
    pub shirt: u8,
    pub x: f32,
    pub y: f32,
    pub stamina: f32,
    pub role: String,
    pub on_pitch: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchSnapshot {
    pub state: String,
    pub half: u8,
    pub time_seconds: u32,
    pub score: [u8; 2],
    pub fouls: [u8; 2],
    pub shots: [u32; 2],
    pub possession: [u8; 2],
    pub players: Vec<PlayerSnapshot>,
    pub ball: (f32, f32),
    pub ball_holder: Option<u32>,
    pub events: Vec<MatchEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum MatchState {
    PreMatch,
    FirstHalf,
    HalfTime,
    SecondHalf,
    Finished,
}

pub struct MatchEngine {
    pub teams: [(u32, String, String); 2],
    pub players: Vec<EnginePlayer>,
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_holder: Option<u32>,
    pub possession: [u32; 2],
    pub rules: FutsalRules,
    pub time: u32,
    pub half: u8,
    pub state: MatchState,
    pub score: [u8; 2],
    pub fouls: [u8; 2],
    pub shots: [u32; 2],
    pub shots_on: [u32; 2],
    pub events: Vec<MatchEvent>,
    pub rng: StdRng,
    pub powerplay: [bool; 2],
    pub bench: [Vec<u32>; 2],
    on_pitch_ids: [Vec<u32>; 2],
}

impl MatchEngine {
    pub fn new(team_names: [(u32, String, String); 2], rosters: [Vec<(u32, u8, Role, PlayerAttrs)>; 2]) -> Self {
        let mut players = Vec::new();
        let mut bench: [Vec<u32>; 2] = [Vec::new(), Vec::new()];
        let mut on_pitch: [Vec<u32>; 2] = [Vec::new(), Vec::new()];

        for (ti, roster) in rosters.iter().enumerate() {
            for (idx, (pid, shirt, role, attrs)) in roster.iter().enumerate() {
                let is_gk = *role == Role::POR;
                let ep = EnginePlayer {
                    id: *pid,
                    team_id: ti as u32,
                    shirt: *shirt,
                    role: *role,
                    attrs: attrs.clone(),
                    x: 0.0,
                    y: 0.0,
                    stamina_now: 100.0,
                    on_pitch: idx < 5,
                    is_gk,
                };
                if idx < 5 {
                    on_pitch[ti].push(*pid);
                } else {
                    bench[ti].push(*pid);
                }
                players.push(ep);
            }
        }

        let mut eng = Self {
            teams: team_names,
            players,
            ball_x: 20.0,
            ball_y: 10.0,
            ball_holder: None,
            possession: [0, 0],
            rules: FutsalRules::default(),
            time: 0,
            half: 1,
            state: MatchState::PreMatch,
            score: [0, 0],
            fouls: [0, 0],
            shots: [0, 0],
            shots_on: [0, 0],
            events: Vec::new(),
            rng: StdRng::from_entropy(),
            powerplay: [false, false],
            bench,
            on_pitch_ids: on_pitch,
        };
        eng.reset_positions();
        eng.ball_holder = eng.on_pitch_ids[0].first().copied();
        eng
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.rng = StdRng::seed_from_u64(seed);
        self
    }

    fn reset_positions(&mut self) {
        for p in &mut self.players {
            if !p.on_pitch { continue; }
            let (x, y) = tactical_target(p.role, p.team_id, false);
            p.x = x;
            p.y = y;
        }
    }

    pub fn start(&mut self) {
        self.state = MatchState::FirstHalf;
        self.time = 0;
        self.half = 1;
        self.events.push(MatchEvent {
            minute: 0, second: 0, kind: "kickoff".into(), team_id: 0,
            player_id: self.ball_holder, description: "Inicio del partido".into(), x: 20.0, y: 10.0,
        });
    }

    fn holder_team(&self) -> Option<u32> {
        if let Some(pid) = self.ball_holder {
            self.players.iter().find(|p| p.id == pid).map(|p| p.team_id)
        } else { None }
    }

    pub fn tick(&mut self) -> Vec<MatchEvent> {
        if self.state == MatchState::Finished || self.state == MatchState::PreMatch || self.state == MatchState::HalfTime {
            return Vec::new();
        }

        let mut new_events = Vec::new();
        self.time += 1;

        if self.time == self.rules.half_seconds && self.half == 1 {
            self.state = MatchState::HalfTime;
            self.fouls = [0, 0];
            self.powerplay = [false, false];
            new_events.push(MatchEvent {
                minute: 20, second: 0, kind: "halftime".into(), team_id: 0, player_id: None,
                description: "Descanso".into(), x: 20.0, y: 10.0,
            });
            self.events.extend(new_events.clone());
            return new_events;
        }
        if self.state == MatchState::HalfTime {
            self.state = MatchState::SecondHalf;
            self.half = 2;
            self.time = self.rules.half_seconds + 1;
            self.reset_positions();
            self.ball_holder = self.on_pitch_ids[1].first().copied();
        }

        if self.time >= self.rules.total_seconds {
            self.state = MatchState::Finished;
            new_events.push(MatchEvent {
                minute: 40, second: 0, kind: "finished".into(), team_id: 0, player_id: None,
                description: format!("Final {}-{}", self.score[0], self.score[1]),
                x: 20.0, y: 10.0,
            });
            self.events.extend(new_events.clone());
            return new_events;
        }

        let losing_powerplay = self.time > self.rules.total_seconds - 180;
        if losing_powerplay {
            for t in 0..2 {
                let other = 1 - t;
                if self.score[t] < self.score[other] {
                    self.powerplay[t] = true;
                }
            }
        }

        self.update_movement(1.0);
        self.update_stamina(1.0);

        if self.time % 8 == 0 {
            self.maybe_substitute();
        }

        if self.time % 2 == 0 {
            if let Some(ev) = self.resolve_action() {
                new_events.push(ev.clone());
                self.events.push(ev);
            }
        }

        new_events
    }

    fn update_movement(&mut self, dt: f32) {
        let possessing = self.holder_team();
        for p in &mut self.players {
            if !p.on_pitch { continue; }
            let attacking = Some(p.team_id) == possessing;
            let (tx, ty) = tactical_target(p.role, p.team_id, attacking);
            let dx = tx - p.x;
            let dy = ty - p.y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > 0.3 {
                let speed = (p.attrs.pace * 0.04 + p.attrs.acceleration * 0.03).clamp(0.4, 2.2);
                let fatigue = (p.stamina_now / 100.0).clamp(0.5, 1.0);
                p.x += (dx / dist) * speed * fatigue * dt;
                p.y += (dy / dist) * speed * fatigue * dt;
                p.x = p.x.clamp(0.5, 39.5);
                p.y = p.y.clamp(0.5, 19.5);
            }
        }
        if let Some(pid) = self.ball_holder {
            if let Some(pl) = self.players.iter().find(|p| p.id == pid) {
                self.ball_x = pl.x;
                self.ball_y = pl.y;
            }
        }
    }

    fn update_stamina(&mut self, dt: f32) {
        for p in &mut self.players {
            if !p.on_pitch { continue; }
            let drain = if Some(p.id) == self.ball_holder { 0.09 } else { 0.04 };
            p.stamina_now -= drain * dt * (1.5 - p.attrs.stamina / 40.0);
            if Some(p.id) != self.ball_holder {
                p.stamina_now += 0.02 * dt;
            }
            p.stamina_now = p.stamina_now.clamp(0.0, 100.0);
        }
    }

    fn maybe_substitute(&mut self) {
        let mut to_swap: Vec<(u32, u32)> = Vec::new();
        for t in 0..2 {
            for &pid in &self.on_pitch_ids[t].clone() {
                if let Some(pl) = self.players.iter().find(|p| p.id == pid) {
                    if should_substitute(pl.stamina_now, self.time) && !self.bench[t].is_empty() {
                        let bench_pid = self.bench[t][0];
                        to_swap.push((pid, bench_pid));
                        break;
                    }
                }
            }
        }
        for (out_id, in_id) in to_swap {
            let team = self.players.iter().find(|p| p.id == out_id).map(|p| p.team_id).unwrap_or(0) as usize;
            if let Some(pos) = self.bench[team].iter().position(|&x| x == in_id) {
                self.bench[team].remove(pos);
            }
            self.bench[team].push(out_id);
            if let Some(idx) = self.on_pitch_ids[team].iter().position(|&x| x == out_id) {
                self.on_pitch_ids[team][idx] = in_id;
            }
            for p in &mut self.players {
                if p.id == out_id { p.on_pitch = false; }
                if p.id == in_id { p.on_pitch = true; p.stamina_now = 95.0; let (tx, ty) = tactical_target(p.role, p.team_id, false); p.x = tx; p.y = ty; }
            }
            self.events.push(MatchEvent {
                minute: self.time / 60, second: self.time % 60, kind: "substitution".into(),
                team_id: team as u32, player_id: Some(in_id),
                description: format!("Cambio: entra {} por {}", in_id, out_id),
                x: self.ball_x, y: self.ball_y,
            });
        }
    }

    fn resolve_action(&mut self) -> Option<MatchEvent> {
        let holder = self.ball_holder?;
        let holder_idx = self.players.iter().position(|p| p.id == holder)?;
        let holder_team = self.players[holder_idx].team_id;
        let holder_attrs = self.players[holder_idx].attrs.clone();
        let holder_x = self.players[holder_idx].x;
        let holder_y = self.players[holder_idx].y;

        let opp_team = 1 - holder_team;
        let is_powerplay = self.powerplay[holder_team as usize];

        let goal_x = if holder_team == 0 { 40.0 } else { 0.0 };
        let dist_to_goal = ((holder_x - goal_x).abs().powi(2) + (holder_y - 10.0).powi(2)).sqrt();
        let angle = ((10.0 - holder_y).abs() / dist_to_goal.max(1.0)).asin().to_degrees().abs();

        let do_shoot = dist_to_goal < 12.0 && self.rng.gen_bool( (0.35 + (holder_attrs.finishing / 60.0)) as f64 );
        if do_shoot {
            self.shots[holder_team as usize] += 1;
            let prob = calculate_goal_probability(&holder_attrs, dist_to_goal, angle, is_powerplay);
            let noise: f32 = self.rng.gen_range(0.85..1.15);
            let effective = (prob * noise).clamp(0.0, 0.95);
            let roll: f32 = self.rng.gen();
            let gk = self.players.iter().filter(|p| p.team_id == opp_team && p.role == Role::POR && p.on_pitch).next();
            let gk_mod = gk.map(|g| 1.0 - g.attrs.reflexes / 40.0).unwrap_or(1.0);
            let final_prob = effective * gk_mod;

            if roll < final_prob * 0.55 {
                self.score[holder_team as usize] += 1;
                self.shots_on[holder_team as usize] += 1;
                self.ball_holder = self.on_pitch_ids[opp_team as usize].first().copied();
                self.ball_x = 20.0; self.ball_y = 10.0;
                return Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "goal".into(),
                    team_id: holder_team, player_id: Some(holder),
                    description: format!("GOOOOL de {}!", holder),
                    x: holder_x, y: holder_y,
                });
            } else if roll < final_prob + 0.25 {
                self.shots_on[holder_team as usize] += 1;
                self.ball_holder = self.players.iter().find(|p| p.team_id == opp_team && p.role == Role::POR && p.on_pitch).map(|p| p.id);
                return Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "save".into(),
                    team_id: opp_team, player_id: gk.map(|g| g.id),
                    description: "Parada del portero".into(), x: holder_x, y: holder_y,
                });
            } else {
                let recov = if self.rng.gen_bool(0.5) { opp_team } else { holder_team };
                self.ball_holder = self.on_pitch_ids[recov as usize].choose(&mut self.rng).copied();
                return Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "shot_off".into(),
                    team_id: holder_team, player_id: Some(holder),
                    description: "Tiro fuera".into(), x: holder_x, y: holder_y,
                });
            }
        }

        let teammates: Vec<u32> = self.on_pitch_ids[holder_team as usize].iter().copied().filter(|&id| id != holder).collect();
        if teammates.is_empty() { return None; }

        let target = *teammates.choose(&mut self.rng).unwrap();
        let defender = self.on_pitch_ids[opp_team as usize].choose(&mut self.rng).copied();
        let def_attrs = defender.and_then(|did| self.players.iter().find(|p| p.id == did).map(|p| p.attrs.clone()));

        let (result, _prob) = resolve_duel(&holder_attrs, &def_attrs.unwrap_or_else(|| holder_attrs.clone()), "pass", &mut self.rng);

        match result {
            DuelResult::Success => {
                self.ball_holder = Some(target);
                if let Some(tp) = self.players.iter().find(|p| p.id == target) {
                    self.ball_x = tp.x; self.ball_y = tp.y;
                }
                self.possession[holder_team as usize] += 1;
                None
            }
            DuelResult::Foul => {
                self.fouls[opp_team as usize] += 1;
                let is_sixth = self.fouls[opp_team as usize] >= self.rules.fouls_for_double;
                if is_sixth {
                    let dp_prob = 0.72;
                    let roll: f32 = self.rng.gen();
                    if roll < dp_prob * (holder_attrs.composure / 20.0) {
                        self.score[holder_team as usize] += 1;
                        self.events.push(MatchEvent {
                            minute: self.time / 60, second: self.time % 60, kind: "double_penalty_goal".into(),
                            team_id: holder_team, player_id: Some(holder),
                            description: "Gol de doble penalti!".into(), x: 30.0, y: 10.0,
                        });
                    }
                    self.fouls[opp_team as usize] = 0;
                    self.ball_holder = self.on_pitch_ids[opp_team as usize].first().copied();
                    return Some(MatchEvent {
                        minute: self.time / 60, second: self.time % 60, kind: "double_penalty".into(),
                        team_id: holder_team, player_id: Some(holder),
                        description: "Doble penalti por 6ª falta".into(), x: 30.0, y: 10.0,
                    });
                }
                self.ball_holder = Some(holder);
                Some(MatchEvent {
                    minute: self.time / 60, second: self.time % 60, kind: "foul".into(),
                    team_id: opp_team, player_id: defender,
                    description: format!("Falta de {} ({}ª del equipo)", defender.unwrap_or(0), self.fouls[opp_team as usize]),
                    x: holder_x, y: holder_y,
                })
            }
            DuelResult::Failure => {
                if let Some(did) = defender {
                    self.ball_holder = Some(did);
                    self.possession[opp_team as usize] += 1;
                    return Some(MatchEvent {
                        minute: self.time / 60, second: self.time % 60, kind: "interception".into(),
                        team_id: opp_team, player_id: Some(did),
                        description: "Intercepción".into(), x: holder_x, y: holder_y,
                    });
                }
                None
            }
        }
    }

    pub fn simulate_full(&mut self) -> MatchSnapshot {
        self.start();
        while self.state != MatchState::Finished {
            if self.state == MatchState::HalfTime {
                self.state = MatchState::SecondHalf;
                self.half = 2;
                self.time = self.rules.half_seconds + 1;
                self.reset_positions();
                self.ball_holder = self.on_pitch_ids[1].first().copied();
                continue;
            }
            self.tick();
        }
        self.snapshot()
    }

    pub fn snapshot(&self) -> MatchSnapshot {
        let players = self.players.iter().map(|p| PlayerSnapshot {
            id: p.id, team_id: p.team_id, shirt: p.shirt, x: p.x, y: p.y,
            stamina: p.stamina_now, role: format!("{:?}", p.role), on_pitch: p.on_pitch,
        }).collect();
        let total_poss = (self.possession[0] + self.possession[1]).max(1) as f32;
        let poss_pct = [
            ((self.possession[0] as f32 / total_poss) * 100.0) as u8,
            ((self.possession[1] as f32 / total_poss) * 100.0) as u8,
        ];
        MatchSnapshot {
            state: format!("{:?}", self.state),
            half: self.half,
            time_seconds: self.time,
            score: self.score,
            fouls: self.fouls,
            shots: self.shots,
            possession: poss_pct,
            players,
            ball: (self.ball_x, self.ball_y),
            ball_holder: self.ball_holder,
            events: self.events.clone(),
        }
    }
}

fn tactical_target(role: Role, team_id: u32, attacking: bool) -> (f32, f32) {
    let left = team_id == 0;
    match role {
        Role::POR => if left { (2.0, 10.0) } else { (38.0, 10.0) },
        Role::CIE => if left { (8.0, 10.0) } else { (32.0, 10.0) },
        Role::ALA => {
            if left {
                if attacking { (22.0, 5.5) } else { (12.0, 6.0) }
            } else {
                if attacking { (18.0, 14.5) } else { (28.0, 14.0) }
            }
        }
        Role::PIV => if left { (31.0, 10.0) } else { (9.0, 10.0) },
        Role::UNI => if left { (16.0, 10.0) } else { (24.0, 10.0) },
    }
}

#[derive(Debug)]
enum DuelResult { Success, Failure, Foul }

fn resolve_duel(attacker: &PlayerAttrs, defender: &PlayerAttrs, action: &str, rng: &mut StdRng) -> (DuelResult, f32) {
    let atk = match action {
        "pass" => attacker.passing * 0.5 + attacker.vision * 0.3 + attacker.technique * 0.2,
        "dribble" => attacker.dribbling * 0.5 + attacker.acceleration * 0.3 + attacker.technique * 0.2,
        _ => attacker.passing,
    };
    let def = defender.tackling * 0.4 + defender.anticipation * 0.3 + defender.positioning * 0.3;
    let noise: f32 = rng.gen_range(0.85..1.15);
    let prob = ((atk / (atk + def).max(1.0)) * noise).clamp(0.05, 0.95);
    let roll: f32 = rng.gen();
    let res = if roll < prob * 0.92 {
        DuelResult::Success
    } else if roll < prob + 0.12 {
        if rng.gen_bool(0.22) { DuelResult::Foul } else { DuelResult::Failure }
    } else {
        if rng.gen_bool(0.18) { DuelResult::Foul } else { DuelResult::Failure }
    };
    (res, prob)
}

fn calculate_goal_probability(shooter: &PlayerAttrs, distance: f32, angle: f32, is_powerplay: bool) -> f32 {
    let base = if distance < 3.0 { 0.68 } else if distance < 6.0 { 0.38 } else if distance < 10.0 { 0.18 } else { 0.05 };
    let angle_mod = (angle / 90.0).clamp(0.2, 1.0);
    let skill = (shooter.finishing / 20.0) * 0.5 + (shooter.composure / 20.0) * 0.3 + (shooter.technique / 20.0) * 0.2;
    let pp = if is_powerplay { 1.25 } else { 1.0 };
    (base * angle_mod * (0.5 + skill) * pp).clamp(0.02, 0.85)
}

fn should_substitute(stamina: f32, time: u32) -> bool {
    if stamina < 38.0 { return true; }
    if time > 300 && stamina < 58.0 { return true; }
    if time % 240 == 0 && stamina < 68.0 { return true; }
    false
}

pub async fn simulate_clubs(pool: &sqlx::SqlitePool, home_club: i64, away_club: i64) -> Result<MatchSnapshot, String> {
    let home_row: Option<(String, String)> = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(home_club).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let away_row: Option<(String, String)> = sqlx::query_as("SELECT name, primary_color FROM clubs WHERE id=?")
        .bind(away_club).fetch_optional(pool).await.map_err(|e| e.to_string())?;
    let (hn, hc) = home_row.ok_or("home club no encontrado")?;
    let (an, ac) = away_row.ok_or("away club no encontrado")?;

    async fn load_roster(pool: &sqlx::SqlitePool, club_id: i64, team_id: u32) -> Result<Vec<(u32, u8, Role, PlayerAttrs)>, String> {
        let rows = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)>(
            "SELECT p.id, pa.passing, pa.finishing, pa.dribbling, pa.tackling, pa.vision, pa.anticipation, pa.positioning, pa.stamina, pa.acceleration, pa.pace, pa.composure, pa.technique, pa.reflexes, pp.ala_natural FROM players p JOIN contracts c ON c.player_id=p.id AND c.club_id=? AND c.is_active=1 JOIN player_attributes pa ON pa.player_id=p.id JOIN player_positions pp ON pp.player_id=p.id LIMIT 12"
        ).bind(club_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for (pid, passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique, reflexes, _ala) in rows {
            let role = if out.len() < 2 { Role::POR } else if out.len() < 4 { Role::CIE } else if out.len() < 8 { Role::ALA } else if out.len() < 10 { Role::PIV } else { Role::UNI };
            let attrs = PlayerAttrs::from_ints(passing, finishing, dribbling, tackling, vision, anticipation, positioning, stamina, acceleration, pace, composure, technique, reflexes);
            out.push((pid as u32, (out.len() + 1) as u8, role, attrs));
        }
        if out.len() < 10 { return Err(format!("club {club_id} solo tiene {} jugadores activos", out.len())); }
        let _ = team_id;
        Ok(out)
    }

    let r1 = load_roster(pool, home_club, 0).await?;
    let r2 = load_roster(pool, away_club, 1).await?;
    let mut eng = MatchEngine::new(
        [(0, hn, hc), (1, an, ac)],
        [r1, r2],
    );
    Ok(eng.simulate_full())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_attrs(ca: i64, role: Role) -> PlayerAttrs { PlayerAttrs::average(ca, role) }

    #[test]
    fn duel_attacker_stronger_wins_more() {
        let strong = mk_attrs(170, Role::ALA);
        let weak = mk_attrs(70, Role::CIE);
        let mut rng = StdRng::seed_from_u64(42);
        let mut wins = 0;
        for _ in 0..200 {
            let (r, _) = resolve_duel(&strong, &weak, "pass", &mut rng);
            if matches!(r, DuelResult::Success) { wins += 1; }
        }
        assert!(wins > 120, "fuerte debe ganar >60%, gano {wins}/200");
    }

    #[test]
    fn goal_prob_distance() {
        let shooter = mk_attrs(150, Role::PIV);
        let p_close = calculate_goal_probability(&shooter, 2.0, 45.0, false);
        let p_far = calculate_goal_probability(&shooter, 14.0, 45.0, false);
        assert!(p_close > p_far, "cerca ({p_close}) > lejos ({p_far})");
        assert!(p_close > 0.3);
        assert!(p_far < 0.1);
    }

    #[test]
    fn full_match_produces_valid_score() {
        let t1: Vec<(u32,u8,Role,PlayerAttrs)> = (1..=12).map(|i| {
            let role = if i<=2 { Role::POR } else if i<=4 { Role::CIE } else if i<=8 { Role::ALA } else if i<=10 { Role::PIV } else { Role::UNI };
            (i, i as u8, role, mk_attrs(120, role))
        }).collect();
        let t2: Vec<(u32,u8,Role,PlayerAttrs)> = (101..=112).map(|i| {
            let role = if i<=102 { Role::POR } else if i<=104 { Role::CIE } else if i<=108 { Role::ALA } else if i<=110 { Role::PIV } else { Role::UNI };
            (i, (i-100) as u8, role, mk_attrs(115, role))
        }).collect();
        let mut eng = MatchEngine::new(
            [(0, "A".into(), "#f00".into()), (1, "B".into(), "#00f".into())],
            [t1, t2],
        ).with_seed(12345);
        let snap = eng.simulate_full();
        assert_eq!(snap.state, "Finished");
        assert_eq!(snap.time_seconds, 2400);
        assert!(snap.score[0] + snap.score[1] <= 15, "goles totales razonables: {:?}", snap.score);
        assert!(snap.events.iter().any(|e| e.kind=="goal" || e.kind=="double_penalty_goal") || snap.score==[0,0]);
    }

    #[test]
    fn stamina_drains_over_match() {
        let t1: Vec<(u32,u8,Role,PlayerAttrs)> = (1..=12).map(|i| {
            let role = if i<=2 { Role::POR } else { Role::ALA };
            (i, i as u8, role, mk_attrs(130, role))
        }).collect();
        let t2: Vec<(u32,u8,Role,PlayerAttrs)> = (101..=112).map(|i| (i, (i-100) as u8, Role::ALA, mk_attrs(130, Role::ALA))).collect();
        let mut eng = MatchEngine::new(
            [(0,"A".into(),"#f00".into()), (1,"B".into(),"#00f".into())],
            [t1,t2],
        ).with_seed(99);
        eng.start();
        for _ in 0..600 { eng.tick(); }
        let low = eng.players.iter().filter(|p| p.on_pitch && p.stamina_now < 85.0).count();
        assert!(low > 0, "algún jugador debe haber perdido stamina tras 10 min");
    }

    #[tokio::test]
    async fn simulate_from_db() {
        let pool = crate::db::init_memory_pool().await.unwrap();
        crate::world::seed_world(&pool).await.unwrap();
        let (hid,): (i64,) = sqlx::query_as("SELECT id FROM clubs WHERE short_name='BAR'").fetch_one(&pool).await.unwrap();
        let (aid,): (i64,) = sqlx::query_as("SELECT id FROM clubs WHERE short_name='INT'").fetch_one(&pool).await.unwrap();
        let snap = crate::engine::simulate_clubs(&pool, hid, aid).await.unwrap();
        assert_eq!(snap.state, "Finished");
        assert!(snap.score[0] + snap.score[1] < 20);
        assert!(snap.events.len() > 2);
    }
}
