
## Motor de Partido 2D con Tauri + Rust + React

---

## **1. ESTRUCTURA DEL PROYECTO**

```bash
futsal-manager/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs                    # Entry point Tauri
│   │   ├── commands.rs                # IPC Commands
│   │   ├── match_engine/
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs              # Motor principal
│   │   │   ├── entities.rs            # Entidades ECS
│   │   │   ├── components.rs          # Componentes
│   │   │   ├── systems.rs             # Sistemas
│   │   │   ├── rules.rs               # Reglas futsal
│   │   │   └── resolution.rs          # Resolución acciones
│   │   ── database/
│   │       ├── mod.rs
│   │       └── schema.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/
│   ├── components/
│   │   ├── MatchView.tsx
│   │   ├── FutsalPitch.tsx
│   │   └── MatchStats.tsx
│   ├── App.tsx
│   └── main.tsx
├── package.json
└── vite.config.ts
```

---

## **2. BACKEND RUST - MOTOR DE PARTIDO**

### **2.1 Cargo.toml (src-tauri/Cargo.toml)**

```toml
[package]
name = "futsal-manager"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "1.5", features = [] }

[dependencies]
tauri = { version = "1.5", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
tokio = { version = "1", features = ["full"] }
rand = "0.8"
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

### **2.2 Componentes ECS (src-tauri/src/match_engine/components.rs)**

```rust
use serde::{Deserialize, Serialize};

/// Posición en el campo (40x20 metros)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,  // 0-40
    pub y: f32,  // 0-20
}

/// Velocidad de movimiento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

/// Atributos del jugador (referencia a BD)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
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
}

/// Estado del jugador en el partido
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    Idle,
    Running,
    Sprinting,
    Tired,
    HasBall,
    Defending,
    Attacking,
}

/// Componente de jugador
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerComponent {
    pub id: u32,
    pub team_id: u32,
    pub shirt_number: u8,
    pub position_role: PositionRole,
    pub attributes: PlayerAttributes,
    pub current_stamina: f32,
    pub state: PlayerState,
    pub minutes_played: u32,
}

/// Roles posicionales en futsal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PositionRole {
    POR,  // Portero
    CIE,  // Cierre
    ALA,  // Ala (rotacional)
    PIV,  // Pívot
    UNI,  // Universal
}

/// Componente de balón
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallComponent {
    pub position: Position,
    pub velocity: Velocity,
    pub holder_id: Option<u32>,  // Jugador que tiene el balón
    pub team_possession: Option<u32>,  // Equipo en posesión
}

/// Componente de equipo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamComponent {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub formation: Formation,
    pub tactics: Tactics,
    pub fouls_this_half: u8,
    pub timeouts_remaining: u8,
    pub score: u8,
}

/// Formaciones de futsal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Formation {
    ThreeOne,  // 3-1
    FourZero,  // 4-0
    TwoTwo,    // 2-2
    FiveZero,  // 5-0 (powerplay)
}

/// Tácticas del equipo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tactics {
    pub tempo: u8,           // 1-100
    pub pressing: u8,        // 1-100
    pub defensive_line: u8,  // 1-100
    pub width: u8,           // 1-100
    pub powerplay_enabled: bool,
}

/// Estado del partido
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchState {
    PreMatch,
    FirstHalf,
    HalfTime,
    SecondHalf,
    ExtraTime,
    Penalties,
    Finished,
}

/// Componente de partido
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchComponent {
    pub state: MatchState,
    pub current_time: u32,      // Segundos (0-2400 = 40 min)
    pub current_half: u8,       // 1 o 2
    pub tick_count: u64,
}
```

### **2.3 Entidades ECS (src-tauri/src/match_engine/entities.rs)**

```rust
use crate::match_engine::components::*;
use serde::{Deserialize, Serialize};

/// Tipo de entidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Ball,
    Team,
    Match,
}

/// Entidad genérica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,
    pub components: EntityComponents,
}

/// Componentes de una entidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityComponents {
    pub position: Option<Position>,
    pub velocity: Option<Velocity>,
    pub player: Option<PlayerComponent>,
    pub ball: Option<BallComponent>,
    pub team: Option<TeamComponent>,
    pub match_state: Option<MatchComponent>,
}

impl Entity {
    pub fn new_player(
        id: u32,
        team_id: u32,
        shirt_number: u8,
        position_role: PositionRole,
        attributes: PlayerAttributes,
    ) -> Self {
        Entity {
            id,
            entity_type: EntityType::Player,
            components: EntityComponents {
                position: Some(Position { x: 0.0, y: 0.0 }),
                velocity: Some(Velocity { vx: 0.0, vy: 0.0 }),
                player: Some(PlayerComponent {
                    id,
                    team_id,
                    shirt_number,
                    position_role,
                    attributes,
                    current_stamina: 100.0,
                    state: PlayerState::Idle,
                    minutes_played: 0,
                }),
                ball: None,
                team: None,
                match_state: None,
            },
        }
    }

    pub fn new_ball() -> Self {
        Entity {
            id: 0,
            entity_type: EntityType::Ball,
            components: EntityComponents {
                position: Some(Position { x: 20.0, y: 10.0 }),
                velocity: Some(Velocity { vx: 0.0, vy: 0.0 }),
                player: None,
                ball: Some(BallComponent {
                    position: Position { x: 20.0, y: 10.0 },
                    velocity: Velocity { vx: 0.0, vy: 0.0 },
                    holder_id: None,
                    team_possession: None,
                }),
                team: None,
                match_state: None,
            },
        }
    }

    pub fn new_team(id: u32, name: String, color: String) -> Self {
        Entity {
            id,
            entity_type: EntityType::Team,
            components: EntityComponents {
                position: None,
                velocity: None,
                player: None,
                ball: None,
                team: Some(TeamComponent {
                    id,
                    name,
                    color,
                    formation: Formation::ThreeOne,
                    tactics: Tactics {
                        tempo: 50,
                        pressing: 50,
                        defensive_line: 50,
                        width: 50,
                        powerplay_enabled: true,
                    },
                    fouls_this_half: 0,
                    timeouts_remaining: 1,
                    score: 0,
                }),
                match_state: None,
            },
        }
    }

    pub fn new_match() -> Self {
        Entity {
            id: 0,
            entity_type: EntityType::Match,
            components: EntityComponents {
                position: None,
                velocity: None,
                player: None,
                ball: None,
                team: None,
                match_state: Some(MatchComponent {
                    state: MatchState::PreMatch,
                    current_time: 0,
                    current_half: 1,
                    tick_count: 0,
                }),
            },
        }
    }
}
```

### **2.4 Reglas de Futsal (src-tauri/src/match_engine/rules.rs)**

```rust
use serde::{Deserialize, Serialize};

/// Reglas oficiales de fútbol sala
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutsalRules {
    pub match_duration_minutes: u8,
    pub half_duration_minutes: u8,
    pub half_time_minutes: u8,
    pub timeout_per_team_per_half: u8,
    pub timeout_duration_seconds: u8,
    pub max_players_on_pitch: u8,
    pub max_substitutes: u16,  // Ilimitadas
    pub team_fouls_for_double_penalty: u8,
    pub double_penalty_distance_m: f32,
    pub kick_in_time_seconds: u8,
    pub goalkeeper_possession_seconds: u8,
    pub substitution_zone_length_m: f32,
}

impl Default for FutsalRules {
    fn default() -> Self {
        Self {
            match_duration_minutes: 40,
            half_duration_minutes: 20,
            half_time_minutes: 10,
            timeout_per_team_per_half: 1,
            timeout_duration_seconds: 60,
            max_players_on_pitch: 5,
            max_substitutes: 999,  // Ilimitadas
            team_fouls_for_double_penalty: 6,
            double_penalty_distance_m: 10.0,
            kick_in_time_seconds: 4,
            goalkeeper_possession_seconds: 4,
            substitution_zone_length_m: 5.0,
        }
    }
}

impl FutsalRules {
    pub fn total_match_seconds(&self) -> u32 {
        self.match_duration_minutes as u32 * 60
    }

    pub fn half_duration_seconds(&self) -> u32 {
        self.half_duration_minutes as u32 * 60
    }
}
```

### **2.5 Resolución de Acciones (src-tauri/src/match_engine/resolution.rs)**

```rust
use crate::match_engine::components::*;
use rand::Rng;

/// Tipos de acciones
#[derive(Debug, Clone)]
pub enum ActionType {
    Pass,
    Shot,
    Dribble,
    Tackle,
    Intercept,
    Block,
}

/// Resultado de una acción
#[derive(Debug, Clone)]
pub enum ActionResult {
    Success,
    Failure,
    Foul,
    Goal,
    Save,
    OutOfBounds,
}

/// Resolver un duelo entre atacante y defensor
pub fn resolve_duel(
    attacker: &PlayerComponent,
    defender: &PlayerComponent,
    action: ActionType,
) -> (ActionResult, f32) {
    let attacker_rating = calculate_action_rating(&attacker.attributes, &action);
    let defender_rating = calculate_defense_rating(&defender.attributes, &action);
    
    // Ruido gaussiano para variabilidad
    let mut rng = rand::thread_rng();
    let noise: f32 = rng.gen_range(0.85..1.15);
    
    let success_probability = (attacker_rating / (attacker_rating + defender_rating)) * noise;
    
    let result = if success_probability > 0.6 {
        ActionResult::Success
    } else if success_probability > 0.4 {
        ActionResult::Failure
    } else {
        // 20% probabilidad de falta
        if rng.gen::<f32>() < 0.2 {
            ActionResult::Foul
        } else {
            ActionResult::Failure
        }
    };
    
    (result, success_probability)
}

/// Calcular rating de acción ofensiva
fn calculate_action_rating(attrs: &PlayerAttributes, action: &ActionType) -> f32 {
    match action {
        ActionType::Pass => {
            attrs.passing * 0.5 + attrs.vision * 0.3 + attrs.technique * 0.2
        }
        ActionType::Shot => {
            attrs.finishing * 0.5 + attrs.composure * 0.3 + attrs.technique * 0.2
        }
        ActionType::Dribble => {
            attrs.dribbling * 0.4 + attrs.acceleration * 0.3 + attrs.technique * 0.3
        }
        _ => 0.0,
    }
}

/// Calcular rating defensivo
fn calculate_defense_rating(attrs: &PlayerAttributes, action: &ActionType) -> f32 {
    match action {
        ActionType::Pass | ActionType::Dribble => {
            attrs.tackling * 0.4 + attrs.positioning * 0.3 + attrs.anticipation * 0.3
        }
        ActionType::Shot => {
            attrs.positioning * 0.5 + attrs.anticipation * 0.3 + attrs.tackling * 0.2
        }
        _ => 0.0,
    }
}

/// Calcular probabilidad de gol
pub fn calculate_goal_probability(
    shooter: &PlayerComponent,
    distance_m: f32,
    angle_degrees: f32,
    is_powerplay: bool,
) -> f32 {
    // Probabilidad base según distancia
    let base_probability = match distance_m {
        d if d < 3.0 => 0.75,
        d if d < 6.0 => 0.45,
        d if d < 10.0 => 0.20,
        _ => 0.05,
    };
    
    // Modificador de ángulo (0-1)
    let angle_modifier = angle_degrees / 90.0;
    
    // Modificador de habilidad
    let skill_modifier = (shooter.attributes.finishing / 20.0) * 0.5
        + (shooter.attributes.composure / 20.0) * 0.3
        + (shooter.attributes.technique / 20.0) * 0.2;
    
    // Modificador de powerplay
    let powerplay_modifier = if is_powerplay { 1.4 } else { 1.0 };
    
    base_probability * angle_modifier * skill_modifier * powerplay_modifier
}

/// Actualizar fatiga del jugador
pub fn update_stamina(
    player: &mut PlayerComponent,
    action_intensity: f32,
    duration_seconds: f32,
) {
    let stamina_drain = action_intensity * 0.01 * duration_seconds;
    
    // Modificador por condición física actual
    let fitness_modifier = player.current_stamina / 100.0;
    
    player.current_stamina -= stamina_drain * (2.0 - fitness_modifier);
    
    // Recuperación pasiva cuando no tiene el balón
    if player.state != PlayerState::HasBall {
        player.current_stamina += 0.02 * duration_seconds;
    }
    
    // Clamp 0-100
    player.current_stamina = player.current_stamina.clamp(0.0, 100.0);
    
    // Actualizar estado según fatiga
    if player.current_stamina < 30.0 {
        player.state = PlayerState::Tired;
    } else if player.current_stamina < 60.0 {
        player.state = PlayerState::Running;
    }
}

/// Decidir si hacer sustitución volante
pub fn should_substitute(player: &PlayerComponent, match_time_seconds: u32) -> bool {
    let minutes_played = match_time_seconds / 60;
    
    // Cambio obligatorio por fatiga
    if player.current_stamina < 40.0 {
        return true;
    }
    
    // Cambio preventivo después de 5 minutos con fatiga media
    if minutes_played > 5 && player.current_stamina < 60.0 {
        return true;
    }
    
    // Cambio táctico programado (cada 3-4 minutos en futsal)
    if minutes_played > 0 && minutes_played % 4 == 0 && player.current_stamina < 70.0 {
        return true;
    }
    
    false
}
```

### **2.6 Sistemas ECS (src-tauri/src/match_engine/systems.rs)**

```rust
use crate::match_engine::components::*;
use crate::match_engine::entities::Entity;
use crate::match_engine::resolution;

/// Sistema de movimiento
pub fn movement_system(entities: &mut Vec<Entity>, delta_time: f32) {
    for entity in entities.iter_mut() {
        if entity.entity_type == EntityType::Player {
            if let (Some(pos), Some(vel), Some(player)) = (
                &mut entity.components.position,
                &entity.components.velocity,
                &mut entity.components.player,
            ) {
                // Actualizar posición
                pos.x += vel.vx * delta_time;
                pos.y += vel.vy * delta_time;
                
                // Limitar al campo (40x20)
                pos.x = pos.x.clamp(0.0, 40.0);
                pos.y = pos.y.clamp(0.0, 20.0);
                
                // Actualizar fatiga
                let intensity = match player.state {
                    PlayerState::Sprinting => 1.5,
                    PlayerState::Running => 1.0,
                    PlayerState::Defending | PlayerState::Attacking => 0.8,
                    _ => 0.3,
                };
                
                resolution::update_stamina(player, intensity, delta_time);
            }
        }
    }
}

/// Sistema de IA táctica
pub fn tactical_ai_system(entities: &mut Vec<Entity>, match_state: &MatchComponent) {
    for entity in entities.iter_mut() {
        if entity.entity_type == EntityType::Player {
            if let (Some(pos), Some(player)) = (
                &mut entity.components.position,
                &entity.components.player,
            ) {
                // Posicionamiento según formación y rol
                let target_position = calculate_tactical_position(
                    player.position_role,
                    player.team_id,
                    match_state.current_time,
                );
                
                // Mover hacia posición objetivo
                let dx = target_position.x - pos.x;
                let dy = target_position.y - pos.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > 0.5 {
                    let speed = player.attributes.pace * 0.1;
                    if let Some(vel) = &mut entity.components.velocity {
                        vel.vx = (dx / distance) * speed;
                        vel.vy = (dy / distance) * speed;
                    }
                }
            }
        }
    }
}

/// Calcular posición táctica según rol
fn calculate_tactical_position(
    role: PositionRole,
    team_id: u32,
    match_time: u32,
) -> Position {
    let attacking = match_time % 120 < 60;  // Alternar ataque/defensa
    
    match role {
        PositionRole::POR => {
            if team_id == 0 {
                Position { x: 1.0, y: 10.0 }
            } else {
                Position { x: 39.0, y: 10.0 }
            }
        }
        PositionRole::CIE => {
            if team_id == 0 {
                Position { x: 8.0, y: 10.0 }
            } else {
                Position { x: 32.0, y: 10.0 }
            }
        }
        PositionRole::ALA => {
            if team_id == 0 {
                if attacking {
                    Position { x: 20.0, y: 5.0 }
                } else {
                    Position { x: 12.0, y: 5.0 }
                }
            } else {
                if attacking {
                    Position { x: 20.0, y: 15.0 }
                } else {
                    Position { x: 28.0, y: 15.0 }
                }
            }
        }
        PositionRole::PIV => {
            if team_id == 0 {
                Position { x: 30.0, y: 10.0 }
            } else {
                Position { x: 10.0, y: 10.0 }
            }
        }
        PositionRole::UNI => {
            if team_id == 0 {
                Position { x: 15.0, y: 10.0 }
            } else {
                Position { x: 25.0, y: 10.0 }
            }
        }
    }
}

/// Sistema de resolución de acciones
pub fn action_resolution_system(
    entities: &mut Vec<Entity>,
    ball_entity: &mut Entity,
) -> Vec<MatchEvent> {
    let mut events = Vec::new();
    
    // Lógica de pases, tiros, regates
    // ... (implementación completa en el motor)
    
    events
}

/// Evento de partido
#[derive(Debug, Clone)]
pub struct MatchEvent {
    pub time: u32,
    pub event_type: String,
    pub player_id: u32,
    pub team_id: u32,
    pub description: String,
}
```

### **2.7 Motor Principal (src-tauri/src/match_engine/engine.rs)**

```rust
use crate::match_engine::components::*;
use crate::match_engine::entities::Entity;
use crate::match_engine::rules::FutsalRules;
use crate::match_engine::systems;

/// Motor de partido
pub struct MatchEngine {
    pub entities: Vec<Entity>,
    pub rules: FutsalRules,
    pub tick_rate: u8,
    pub is_running: bool,
}

impl MatchEngine {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            rules: FutsalRules::default(),
            tick_rate: 60,
            is_running: false,
        }
    }

    /// Inicializar partido con equipos
    pub fn initialize_match(
        &mut self,
        team1_players: Vec<(u8, PositionRole, PlayerAttributes)>,
        team2_players: Vec<(u8, PositionRole, PlayerAttributes)>,
    ) {
        // Crear entidad de partido
        self.entities.push(Entity::new_match());
        
        // Crear equipos
        let team1 = Entity::new_team(0, "Barcelona".to_string(), "#A50044".to_string());
        let team2 = Entity::new_team(1, "Real Madrid".to_string(), "#FFFFFF".to_string());
        self.entities.push(team1);
        self.entities.push(team2);
        
        // Crear balón
        self.entities.push(Entity::new_ball());
        
        // Crear jugadores equipo 1
        for (i, (shirt, role, attrs)) in team1_players.into_iter().enumerate() {
            let player = Entity::new_player(
                (i + 1) as u32,
                0,
                shirt,
                role,
                attrs,
            );
            self.entities.push(player);
        }
        
        // Crear jugadores equipo 2
        for (i, (shirt, role, attrs)) in team2_players.into_iter().enumerate() {
            let player = Entity::new_player(
                (i + 100) as u32,
                1,
                shirt,
                role,
                attrs,
            );
            self.entities.push(player);
        }
    }

    /// Tick del motor (60 TPS)
    pub fn tick(&mut self) -> Vec<systems::MatchEvent> {
        if !self.is_running {
            return Vec::new();
        }
        
        let delta_time = 1.0 / self.tick_rate as f32;
        let mut events = Vec::new();
        
        // Actualizar tiempo de partido
        if let Some(match_entity) = self.entities.iter_mut().find(|e| e.entity_type == EntityType::Match) {
            if let Some(match_state) = &mut match_entity.components.match_state {
                match_state.tick_count += 1;
                match_state.current_time += 1;
                
                // Verificar fin de primer tiempo
                if match_state.current_time == self.rules.half_duration_seconds() {
                    match_state.state = MatchState::HalfTime;
                }
                
                // Verificar fin de partido
                if match_state.current_time >= self.rules.total_match_seconds() {
                    match_state.state = MatchState::Finished;
                    self.is_running = false;
                }
            }
        }
        
        // Ejecutar sistemas
        systems::movement_system(&mut self.entities, delta_time);
        systems::tactical_ai_system(&mut self.entities, 
            &self.entities.iter()
                .find(|e| e.entity_type == EntityType::Match)
                .unwrap()
                .components
                .match_state
                .clone()
                .unwrap()
        );
        
        // Resolver acciones
        let ball_entity = self.entities.iter_mut()
            .find(|e| e.entity_type == EntityType::Ball)
            .unwrap();
        events = systems::action_resolution_system(&mut self.entities, ball_entity);
        
        events
    }

    /// Iniciar partido
    pub fn start(&mut self) {
        if let Some(match_entity) = self.entities.iter_mut().find(|e| e.entity_type == EntityType::Match) {
            if let Some(match_state) = &mut match_entity.components.match_state {
                match_state.state = MatchState::FirstHalf;
            }
        }
        self.is_running = true;
    }

    /// Pausar partido
    pub fn pause(&mut self) {
        self.is_running = false;
    }

    /// Obtener estado actual para frontend
    pub fn get_state(&self) -> MatchStateResponse {
        let match_state = self.entities.iter()
            .find(|e| e.entity_type == EntityType::Match)
            .and_then(|e| e.components.match_state.clone());
        
        let players: Vec<PlayerStateResponse> = self.entities.iter()
            .filter(|e| e.entity_type == EntityType::Player)
            .filter_map(|e| {
                if let (Some(pos), Some(player)) = (&e.components.position, &e.components.player) {
                    Some(PlayerStateResponse {
                        id: player.id,
                        team_id: player.team_id,
                        shirt_number: player.shirt_number,
                        x: pos.x,
                        y: pos.y,
                        stamina: player.current_stamina,
                        state: format!("{:?}", player.state),
                    })
                } else {
                    None
                }
            })
            .collect();
        
        let ball = self.entities.iter()
            .find(|e| e.entity_type == EntityType::Ball)
            .and_then(|e| e.components.ball.clone())
            .map(|b| BallStateResponse {
                x: b.position.x,
                y: b.position.y,
            });
        
        MatchStateResponse {
            state: match_state.map(|m| format!("{:?}", m.state)),
            current_time: match_state.map(|m| m.current_time),
            players,
            ball,
        }
    }
}

/// Respuesta de estado para frontend
#[derive(Debug, Clone, serde::Serialize)]
pub struct MatchStateResponse {
    pub state: Option<String>,
    pub current_time: Option<u32>,
    pub players: Vec<PlayerStateResponse>,
    pub ball: Option<BallStateResponse>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerStateResponse {
    pub id: u32,
    pub team_id: u32,
    pub shirt_number: u8,
    pub x: f32,
    pub y: f32,
    pub stamina: f32,
    pub state: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BallStateResponse {
    pub x: f32,
    pub y: f32,
}
```

### **2.8 Comandos Tauri IPC (src-tauri/src/commands.rs)**

```rust
use crate::match_engine::engine::MatchEngine;
use crate::match_engine::components::*;
use std::sync::Mutex;

// Estado global del motor
lazy_static::lazy_static! {
    pub static ref MATCH_ENGINE: Mutex<MatchEngine> = Mutex::new(MatchEngine::new());
}

/// Inicializar partido de prueba
#[tauri::command]
pub fn initialize_test_match() -> Result<String, String> {
    let mut engine = MATCH_ENGINE.lock().map_err(|e| e.to_string())?;
    
    // Jugadores de prueba
    let team1_players = vec![
        (1, PositionRole::POR, create_test_attributes(15.0)),
        (2, PositionRole::CIE, create_test_attributes(14.0)),
        (3, PositionRole::ALA, create_test_attributes(16.0)),
        (4, PositionRole::ALA, create_test_attributes(15.0)),
        (5, PositionRole::PIV, create_test_attributes(17.0)),
    ];
    
    let team2_players = vec![
        (1, PositionRole::POR, create_test_attributes(14.0)),
        (2, PositionRole::CIE, create_test_attributes(13.0)),
        (3, PositionRole::ALA, create_test_attributes(15.0)),
        (4, PositionRole::ALA, create_test_attributes(14.0)),
        (5, PositionRole::PIV, create_test_attributes(16.0)),
    ];
    
    engine.initialize_match(team1_players, team2_players);
    engine.start();
    
    Ok("Partido inicializado".to_string())
}

/// Obtener estado del partido
#[tauri::command]
pub fn get_match_state() -> Result<crate::match_engine::engine::MatchStateResponse, String> {
    let engine = MATCH_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.get_state())
}

/// Avanzar un tick
#[tauri::command]
pub fn tick_match() -> Result<Vec<String>, String> {
    let mut engine = MATCH_ENGINE.lock().map_err(|e| e.to_string())?;
    let events = engine.tick();
    Ok(events.iter().map(|e| e.description.clone()).collect())
}

/// Crear atributos de prueba
fn create_test_attributes(avg: f32) -> PlayerAttributes {
    PlayerAttributes {
        passing: avg,
        finishing: avg,
        dribbling: avg,
        tackling: avg,
        vision: avg,
        anticipation: avg,
        positioning: avg,
        stamina: avg,
        acceleration: avg,
        pace: avg,
        composure: avg,
        technique: avg,
    }
}
```

### **2.9 Main.rs (src-tauri/src/main.rs)**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod match_engine;
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::initialize_test_match,
            commands::get_match_state,
            commands::tick_match,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## **3. FRONTEND REACT - VISUALIZACIÓN 2D**

### **3.1 package.json**

```json
{
  "name": "futsal-manager-frontend",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "@tauri-apps/api": "^1.5.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-konva": "^18.2.10",
    "konva": "^9.3.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.43",
    "@types/react-dom": "^18.2.17",
    "@vitejs/plugin-react": "^4.2.1",
    "typescript": "^5.3.3",
    "vite": "^5.0.8",
    "@tauri-apps/cli": "^1.5.0"
  }
}
```

### **3.2 Componente Principal de Partido (src/components/MatchView.tsx)**

```typescript
import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import FutsalPitch from './FutsalPitch';
import MatchStats from './MatchStats';

interface PlayerState {
  id: number;
  team_id: number;
  shirt_number: number;
  x: number;
  y: number;
  stamina: number;
  state: string;
}

interface BallState {
  x: number;
  y: number;
}

interface MatchState {
  state: string;
  current_time: number;
  players: PlayerState[];
  ball: BallState;
}

const MatchView: React.FC = () => {
  const [matchState, setMatchState] = useState<MatchState | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [events, setEvents] = useState<string[]>([]);

  // Inicializar partido de prueba
  const initializeMatch = async () => {
    try {
      await invoke('initialize_test_match');
      setIsRunning(true);
    } catch (error) {
      console.error('Error inicializando partido:', error);
    }
  };

  // Loop de actualización (60 FPS)
  useEffect(() => {
    if (!isRunning) return;

    const interval = setInterval(async () => {
      try {
        // Avanzar tick
        const newEvents = await invoke<string[]>('tick_match');
        if (newEvents.length > 0) {
          setEvents(prev => [...newEvents, ...prev].slice(0, 10));
        }

        // Obtener estado actualizado
        const state = await invoke<MatchState>('get_match_state');
        setMatchState(state);

        // Verificar si terminó
        if (state.state === 'Finished') {
          setIsRunning(false);
        }
      } catch (error) {
        console.error('Error en tick:', error);
      }
    }, 1000 / 60); // 60 FPS

    return () => clearInterval(interval);
  }, [isRunning]);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="match-view">
      <div className="match-header">
        <h1>Barcelona vs Real Madrid</h1>
        <div className="match-info">
          <span className="time">{matchState ? formatTime(matchState.current_time) : '00:00'}</span>
          <span className="state">{matchState?.state || 'Pre-Match'}</span>
        </div>
      </div>

      <div className="match-content">
        <FutsalPitch 
          players={matchState?.players || []} 
          ball={matchState?.ball} 
        />
        
        <MatchStats events={events} />
      </div>

      <div className="match-controls">
        <button onClick={initializeMatch} disabled={isRunning}>
          Iniciar Partido
        </button>
        <button onClick={() => setIsRunning(!isRunning)}>
          {isRunning ? 'Pausar' : 'Continuar'}
        </button>
      </div>
    </div>
  );
};

export default MatchView;
```

### **3.3 Campo 2D con Konva (src/components/FutsalPitch.tsx)**

```typescript
import { Stage, Layer, Rect, Circle, Text, Line } from 'react-konva';

interface PlayerState {
  id: number;
  team_id: number;
  shirt_number: number;
  x: number;
  y: number;
  stamina: number;
  state: string;
}

interface BallState {
  x: number;
  y: number;
}

interface FutsalPitchProps {
  players: PlayerState[];
  ball: BallState | null;
}

const PITCH_WIDTH = 800;
const PITCH_HEIGHT = 400;
const SCALE = 20; // 1 metro = 20px

const FutsalPitch: React.FC<FutsalPitchProps> = ({ players, ball }) => {
  const teamColors = ['#A50044', '#FFFFFF'];
  const teamStrokes = ['#000000', '#000000'];

  return (
    <Stage width={PITCH_WIDTH} height={PITCH_HEIGHT}>
      <Layer>
        {/* Fondo del campo */}
        <Rect
          x={0}
          y={0}
          width={PITCH_WIDTH}
          height={PITCH_HEIGHT}
          fill="#2d8a2d"
        />

        {/* Líneas del campo */}
        <Rect
          x={10}
          y={10}
          width={PITCH_WIDTH - 20}
          height={PITCH_HEIGHT - 20}
          stroke="#ffffff"
          strokeWidth={2}
          fill={null}
        />

        {/* Línea central */}
        <Line
          points={[PITCH_WIDTH / 2, 10, PITCH_WIDTH / 2, PITCH_HEIGHT - 10]}
          stroke="#ffffff"
          strokeWidth={2}
        />

        {/* Círculo central */}
        <Circle
          x={PITCH_WIDTH / 2}
          y={PITCH_HEIGHT / 2}
          radius={40}
          stroke="#ffffff"
          strokeWidth={2}
          fill={null}
        />

        {/* Áreas de penalti (6m) */}
        <Rect
          x={10}
          y={120}
          width={120}
          height={160}
          stroke="#ffffff"
          strokeWidth={2}
          fill={null}
        />
        <Rect
          x={PITCH_WIDTH - 130}
          y={120}
          width={120}
          height={160}
          stroke="#ffffff"
          strokeWidth={2}
          fill={null}
        />

        {/* Puntos de penalti (6m) */}
        <Circle
          x={130}
          y={PITCH_HEIGHT / 2}
          radius={3}
          fill="#ffffff"
        />
        <Circle
          x={PITCH_WIDTH - 130}
          y={PITCH_HEIGHT / 2}
          radius={3}
          fill="#ffffff"
        />

        {/* Puntos de doble penalti (10m) */}
        <Circle
          x={200}
          y={PITCH_HEIGHT / 2}
          radius={3}
          fill="#ffffff"
        />
        <Circle
          x={PITCH_WIDTH - 200}
          y={PITCH_HEIGHT / 2}
          radius={3}
          fill="#ffffff"
        />

        {/* Jugadores */}
        {players.map((player) => (
          <PlayerEntity
            key={player.id}
            player={player}
            color={teamColors[player.team_id]}
            stroke={teamStrokes[player.team_id]}
          />
        ))}

        {/* Balón */}
        {ball && (
          <Circle
            x={ball.x * SCALE}
            y={ball.y * SCALE}
            radius={6}
            fill="#ffffff"
            stroke="#000000"
            strokeWidth={1}
          />
        )}
      </Layer>
    </Stage>
  );
};

interface PlayerEntityProps {
  player: PlayerState;
  color: string;
  stroke: string;
}

const PlayerEntity: React.FC<PlayerEntityProps> = ({ player, color, stroke }) => {
  const x = player.x * SCALE;
  const y = player.y * SCALE;
  const opacity = player.stamina < 30 ? 0.6 : 1.0;

  return (
    <group x={x} y={y}>
      {/* Círculo del jugador */}
      <Circle
        radius={10}
        fill={color}
        stroke={stroke}
        strokeWidth={2}
        opacity={opacity}
      />
      
      {/* Dorsal */}
      <Text
        text={player.shirt_number.toString()}
        fontSize={12}
        fill="#ffffff"
        align="center"
        offsetX={6}
        offsetY={4}
      />

      {/* Indicador de fatiga */}
      {player.stamina < 60 && (
        <Rect
          x={-10}
          y={-15}
          width={20 * (player.stamina / 100)}
          height={3}
          fill={player.stamina < 30 ? '#ff0000' : '#ffff00'}
        />
      )}
    </group>
  );
};

export default FutsalPitch;
```

### **3.4 Panel de Estadísticas (src/components/MatchStats.tsx)**

```typescript
interface MatchStatsProps {
  events: string[];
}

const MatchStats: React.FC<MatchStatsProps> = ({ events }) => {
  return (
    <div className="match-stats">
      <h3>Eventos del Partido</h3>
      <div className="events-list">
        {events.map((event, index) => (
          <div key={index} className="event-item">
            {event}
          </div>
        ))}
      </div>
    </div>
  );
};

export default MatchStats;
```

### **3.5 App.tsx Principal**

```typescript
import MatchView from './components/MatchView';

function App() {
  return (
    <div className="App">
      <MatchView />
    </div>
  );
}

export default App;
```

### **3.6 Estilos CSS (src/index.css)**

```css
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: #1a1a1a;
  color: #ffffff;
}

.App {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
}

.match-view {
  background: #2a2a2a;
  border-radius: 12px;
  padding: 20px;
  max-width: 1200px;
  width: 100%;
}

.match-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding-bottom: 15px;
  border-bottom: 2px solid #444;
}

.match-header h1 {
  font-size: 24px;
  color: #fff;
}

.match-info {
  display: flex;
  gap: 20px;
  font-size: 18px;
}

.time {
  font-weight: bold;
  color: #00ff00;
}

.state {
  color: #aaa;
}

.match-content {
  display: flex;
  gap: 20px;
  margin-bottom: 20px;
}

.match-stats {
  width: 300px;
  background: #1a1a1a;
  border-radius: 8px;
  padding: 15px;
}

.match-stats h3 {
  margin-bottom: 15px;
  color: #fff;
}

.events-list {
  max-height: 400px;
  overflow-y: auto;
}

.event-item {
  padding: 8px;
  margin-bottom: 5px;
  background: #2a2a2a;
  border-radius: 4px;
  font-size: 14px;
}

.match-controls {
  display: flex;
  gap: 10px;
  justify-content: center;
}

button {
  padding: 12px 24px;
  font-size: 16px;
  border: none;
  border-radius: 6px;
  background: #0066cc;
  color: white;
  cursor: pointer;
  transition: background 0.2s;
}

button:hover {
  background: #0052a3;
}

button:disabled {
  background: #555;
  cursor: not-allowed;
}
```

---

## **4. INSTRUCCIONES DE EJECUCIÓN**

### **4.1 Setup del Proyecto**

```bash
# 1. Crear proyecto Tauri
cargo create-tauri-app futsal-manager --template react-ts
cd futsal-manager

# 2. Instalar dependencias Rust
cd src-tauri
cargo add sqlx --features runtime-tokio-rustls,sqlite
cargo add tokio --features full
cargo add rand uuid chrono lazy_static

# 3. Instalar dependencias frontend
cd ..
npm install react-konva konva

# 4. Ejecutar en desarrollo
npm run tauri dev
```

### **4.2 Estructura de Archivos Final**

```
futsal-manager/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs
│   │   ├── match_engine/
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs
│   │   │   ├── entities.rs
│   │   │   ├── components.rs
│   │   │   ├── systems.rs
│   │   │   ├── rules.rs
│   │   │   └── resolution.rs
│   │   └── database/
│   │       ├── mod.rs
│   │       └── schema.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/
│   ├── components/
│   │   ├── MatchView.tsx
│   │   ├── FutsalPitch.tsx
│   │   └── MatchStats.tsx
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── package.json
├── tsconfig.json
── vite.config.ts
```

---

