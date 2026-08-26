
# **FUTSAL MANAGER 2027
## Documento de Requisitos del Producto (PRD) - Versión 2.0

---

## **1. VISIÓN GENERAL**

### **1.1 Propósito del Producto**
Desarrollar el simulador de gestión de fútbol sala más completo y realista del mercado, combinando la profundidad táctica del fútbol sala moderno con una interfaz accesible y un motor de partidos 2D cenital estilo "chapas" que visualice cada acción en tiempo real.

### **1.2 Público Objetivo**
- Aficionados al fútbol sala que buscan profundidad táctica
- Jugadores de Football Manager que desean una experiencia más ágil
- Entrenadores de futsal que quieren experimentar con tácticas

### **1.3 Pilares de Diseño**
1. **Autenticidad Futsal**: Reglas, tácticas y dinámicas 100% reales
2. **Profundidad Estratégica**: Cantera, ojeo limitado, economía realista
3. **Rendimiento**: Simulación ultrarrápida de múltiples ligas simultáneas
4. **Visualización Clara**: Motor 2D que muestre la esencia del juego sin sobrecargar

---

## **2. ARQUITECTURA TECNOLÓGICA DEFINITIVA**

### **2.1 Stack Tecnológico**

```
─────────────────────────────────────────────────────────┐
│                    FRONTEND (UI/UX)                      │
│  React 18 + TypeScript + TailwindCSS + Zustand (State)  │
│  - Componentes: Shadcn/UI + TanStack Table              │
│  - Gráficos: Recharts (estadísticas)                    │
│  - Mapas/Posiciones: Konva.js (canvas 2D)               │
└─────────────────────────────────────────────────────────┘
                            ↕ IPC (Tauri Commands)
┌─────────────────────────────────────────────────────────┐
│              BACKEND CORE (Rust Native)                  │
│  Tauri v2 + Rust (Simulación + Lógica de Juego)         │
│  - Motor de Partidos: ECS (Bevy o custom)               │
│  - Base de Datos: SQLx + SQLite (WAL mode)              │
│  - Serialización: Serde (JSON/MessagePack)              │
│  - Pathfinding: A* para movimientos tácticos            │
─────────────────────────────────────────────────────────┘
                            ↕
┌─────────────────────────────────────────────────────────┐
│              PERSISTENCIA (SQLite)                       │
│  - Modo WAL (Write-Ahead Logging)                       │
│  - Índices optimizados para consultas masivas           │
│  - Migraciones: SQLx migrate                            │
└─────────────────────────────────────────────────────────┘
```

**Justificación de Rust sobre Python:**
- **Rendimiento**: 10-50x más rápido en cálculos de simulación
- **Seguridad de memoria**: Sin garbage collector, crucial para loops de 60 TPS
- **Concurrencia**: Manejo nativo de múltiples partidos simultáneos
- **ECS Pattern**: Bevy/Custom ECS es nativo en Rust, ideal para motores de juego

---

## **3. ESQUEMA DE BASE DE DATOS COMPLETO**

### **3.1 Mundo y Geografía**

```sql
-- Confederaciones
CREATE TABLE confederations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,           -- UEFA, CONMEBOL, AFC, CAF, OFC
    short_name TEXT NOT NULL,            -- UEFA, CONMEBOL, etc.
    reputation INTEGER DEFAULT 1000      -- 1-10000
);

-- Naciones
CREATE TABLE nations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    confederation_id INTEGER NOT NULL,
    reputation INTEGER DEFAULT 500,      -- 1-10000
    futsal_level INTEGER DEFAULT 50,     -- 1-100 (nivel de la liga)
    has_national_team BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (confederation_id) REFERENCES confederations(id)
);

-- Regiones (para ojeo)
CREATE TABLE regions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    nation_id INTEGER NOT NULL,
    talent_pool INTEGER DEFAULT 50,      -- 1-100
    FOREIGN KEY (nation_id) REFERENCES nations(id)
);

-- Ciudades
CREATE TABLE cities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    region_id INTEGER,
    nation_id INTEGER NOT NULL,
    population INTEGER,
    FOREIGN KEY (nation_id) REFERENCES nations(id)
);
```

### **3.2 Competiciones (Basado en tu lista)**

```sql
-- Tipos de competición
CREATE TABLE competition_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,                  -- League, Cup, International
    has_groups BOOLEAN DEFAULT FALSE,
    has_knockout BOOLEAN DEFAULT FALSE,
    promotion_relegation BOOLEAN DEFAULT FALSE
);

-- Competiciones
CREATE TABLE competitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    nation_id INTEGER,                   -- NULL si es internacional
    competition_type_id INTEGER NOT NULL,
    tier INTEGER DEFAULT 1,              -- 1 = Primera, 2 = Segunda, etc.
    current_season TEXT NOT NULL,        -- "2026/2027"
    total_teams INTEGER,
    format TEXT,                         -- "Round Robin", "Groups + KO", etc.
    has_playoffs BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (nation_id) REFERENCES nations(id),
    FOREIGN KEY (competition_type_id) REFERENCES competition_types(id)
);

-- Niveles de competición predefinidos
INSERT INTO competitions (name, nation_id, tier, total_teams) VALUES
-- ESPAÑA
('Primera División de Fútbol Sala', 1, 1, 16),        -- España ID=1
('Segunda División B de Fútbol Sala - Grupo 1', 1, 3, 16),
('Segunda División B de Fútbol Sala - Grupo 2', 1, 3, 16),
('Segunda División B de Fútbol Sala - Grupo 3', 1, 3, 16),
('Segunda División B de Fútbol Sala - Grupo 4', 1, 3, 16),
('Segunda División B de Fútbol Sala - Grupo 5', 1, 3, 16),
('Segunda División B de Fútbol Sala - Grupo 6', 1, 3, 16),
('Copa de España de Fútbol Sala', 1, NULL, 32),
('Supercopa de España', 1, NULL, 4),

-- BRASIL
('Liga Nacional de Futsal (LNF)', 2, 1, 16),          -- Brasil ID=2
('Taça Brasil de Futsal', 2, NULL, 32),

-- PORTUGAL
('Liga Placard', 3, 1, 14),                            -- Portugal ID=3
('Taça de Portugal de Futsal', 3, NULL, 64),

-- RUSIA
('Superliga Rusa de Fútbol Sala', 4, 1, 14),          -- Rusia ID=4

-- ITALIA
('Serie A Futsal', 5, 1, 14),                          -- Italia ID=5
('Serie A2 Futsal', 5, 2, 28),

-- KAZAJISTÁN
('Kazakhstani Futsal Championship', 6, 1, 12),        -- Kazajistán ID=6

-- IRÁN
('Iranian Futsal Super League', 7, 1, 14),            -- Irán ID=7

-- ARGENTINA
('Argentine Futsal Primera División', 8, 1, 12),      -- Argentina ID=8

-- UCRANIA
('Ukrainian Futsal Extraliga', 9, 1, 12),             -- Ucrania ID=9

-- FRANCIA
('French Futsal D1', 10, 1, 14),                       -- Francia ID=10

-- JAPÓN
('Japan Futsal League (F.League)', 11, 1, 10),        -- Japón ID=11

-- CROACIA
('Croatian First Futsal League', 12, 1, 12),          -- Croacia ID=12

-- REPÚBLICA CHECA
('Czech Futsal First League', 13, 1, 12),             -- Rep. Checa ID=13

-- SERBIA
('Serbian Futsal League', 14, 1, 12),                 -- Serbia ID=14

-- POLONIA
('Polish Futsal Ekstraklasa', 15, 1, 14),             -- Polonia ID=15

-- AZERBAIYÁN
('Azerbaijani Futsal Higher League', 16, 1, 10),      -- Azerbaiyán ID=16

-- COLOMBIA
('Colombian Futsal League', 17, 1, 12),               -- Colombia ID=17

-- TAILANDIA
('Thai Futsal League', 18, 1, 12),                    -- Tailandia ID=18

-- URUGUAY
('Liga de Futsal de Uruguay', 19, 1, 10),             -- Uruguay ID=19

-- INTERNACIONALES
('Mundial de Fútbol Sala', NULL, NULL, 24),           -- NULL = Internacional
('Copa América de Fútbol Sala', NULL, NULL, 10),
('Eurocopa de Fútbol Sala', NULL, NULL, 16),
('AFC Futsal Asian Cup', NULL, NULL, 16),
('CAF Futsal Africa Cup of Nations', NULL, NULL, 12),
('CONMEBOL Futsal Championship', NULL, NULL, 10),
('OFC Futsal Men Cup', NULL, NULL, 8);

-- Clasificatorios (sub-competiciones)
CREATE TABLE qualifying_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_competition_id INTEGER NOT NULL,
    name TEXT NOT NULL,                    -- "Grupo A", "UEFA Zone", etc.
    nation_id INTEGER,
    FOREIGN KEY (parent_competition_id) REFERENCES competitions(id)
);
```

### **3.3 Clubes e Instalaciones**

```sql
-- Pabellones
CREATE TABLE stadiums (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    city_id INTEGER,
    capacity INTEGER NOT NULL,
    pitch_type TEXT CHECK(pitch_type IN ('parquet', 'rubber', 'synthetic')),
    has_video_analysis BOOLEAN DEFAULT FALSE,
    training_facilities INTEGER DEFAULT 50,    -- 1-100
    youth_facilities INTEGER DEFAULT 50,        -- 1-100
    FOREIGN KEY (city_id) REFERENCES cities(id)
);

-- Clubes
CREATE TABLE clubs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    short_name TEXT,                           -- Para UI (3-4 letras)
    nation_id INTEGER NOT NULL,
    city_id INTEGER,
    stadium_id INTEGER,
    founded_year INTEGER,
    reputation INTEGER DEFAULT 100,            -- 1-10000
    is_user_controlled BOOLEAN DEFAULT FALSE,
    has_reserve_team BOOLEAN DEFAULT FALSE,
    has_youth_academy BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (nation_id) REFERENCES nations(id),
    FOREIGN KEY (stadium_id) REFERENCES stadiums(id)
);

-- Finanzas del club
CREATE TABLE club_finances (
    club_id INTEGER PRIMARY KEY,
    balance REAL DEFAULT 0,                    -- Saldo actual
    transfer_budget REAL DEFAULT 0,            -- Presupuesto fichajes
    wage_budget REAL DEFAULT 0,                -- Presupuesto salarial
    total_wages REAL DEFAULT 0,                -- Gastos salariales actuales
    sponsorship_income REAL DEFAULT 0,
    ticket_income REAL DEFAULT 0,
    prize_money REAL DEFAULT 0,
    last_updated TEXT,
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);

-- Historial de temporadas
CREATE TABLE club_season_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    season TEXT NOT NULL,                      -- "2026/2027"
    competition_id INTEGER NOT NULL,
    final_position INTEGER,
    points INTEGER,
    played INTEGER,
    won INTEGER,
    drawn INTEGER,
    lost INTEGER,
    goals_for INTEGER,
    goals_against INTEGER,
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (competition_id) REFERENCES competitions(id)
);
```

### **3.4 Jugadores - Sistema Completo**

```sql
-- Datos estáticos del jugador
CREATE TABLE players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    common_name TEXT,                          -- Nombre común (opcional)
    date_of_birth TEXT NOT NULL,               -- ISO8601
    nation_id INTEGER NOT NULL,
    second_nation_id INTEGER,                  -- Doble nacionalidad
    city_of_birth TEXT,
    preferred_foot TEXT CHECK(preferred_foot IN ('left', 'right', 'both')),
    height_cm INTEGER,                         -- En cm (160-200)
    weight_kg INTEGER,                         -- En kg (60-100)
    media_description TEXT,                    -- "Joven promesa", "Veterano", etc.
    is_retired BOOLEAN DEFAULT FALSE,
    retired_date TEXT,
    FOREIGN KEY (nation_id) REFERENCES nations(id)
);

-- Posiciones de Futsal (sin distinción estricta izquierda/derecha)
CREATE TABLE player_positions (
    player_id INTEGER PRIMARY KEY,
    por_natural INTEGER DEFAULT 0,             -- 1-20 (Portero)
    cie_natural INTEGER DEFAULT 0,             -- 1-20 (Cierre)
    ala_natural INTEGER DEFAULT 0,             -- 1-20 (Ala - rotacional)
    piv_natural INTEGER DEFAULT 0,             -- 1-20 (Pívot)
    uni_natural INTEGER DEFAULT 0,             -- 1-20 (Universal)
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Estados dinámicos (se actualizan diariamente)
CREATE TABLE player_states (
    player_id INTEGER PRIMARY KEY,
    current_ability INTEGER DEFAULT 50,        -- 1-200 (CA)
    potential_ability INTEGER DEFAULT 100,     -- 1-200 (PA)
    condition INTEGER DEFAULT 100,             -- 0-100 (Condición física)
    match_fitness INTEGER DEFAULT 100,         -- 0-100 (Forma de partido)
    morale INTEGER DEFAULT 50,                 -- 0-100
    sharpness INTEGER DEFAULT 50,              -- 0-100 (Puesta a punto)
    happiness INTEGER DEFAULT 50,              -- 0-100
    last_updated TEXT,
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Atributos Técnicos (1-20)
CREATE TABLE player_attributes_technical (
    player_id INTEGER PRIMARY KEY,
    -- Control y manejo del balón
    first_touch INTEGER DEFAULT 10,            -- Primer toque
    dribbling INTEGER DEFAULT 10,              -- Regate
    ball_control INTEGER DEFAULT 10,           -- Control en espacio reducido
    technique INTEGER DEFAULT 10,              -- Técnica general
    
    -- Pase y creación
    passing INTEGER DEFAULT 10,                -- Pase corto/largo
    vision INTEGER DEFAULT 10,                 -- Visión de juego
    crossing INTEGER DEFAULT 10,               -- Centros (desde banda)
    long_shots INTEGER DEFAULT 10,             -- Tiros lejanos
    
    -- Definición
    finishing INTEGER DEFAULT 10,              -- Definición
    heading INTEGER DEFAULT 10,                -- Juego aéreo (menos relevante)
    penalty_taking INTEGER DEFAULT 10,         -- Lanzamiento penaltis
    
    -- Defensa
    tackling INTEGER DEFAULT 10,               -- Entrada
    marking INTEGER DEFAULT 10,                -- Maraje
    interception INTEGER DEFAULT 10,           -- Intercepción
    blocking INTEGER DEFAULT 10,               -- Bloqueo de tiros
    
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Atributos Mentales (1-20)
CREATE TABLE player_attributes_mental (
    player_id INTEGER PRIMARY KEY,
    -- Toma de decisiones
    anticipation INTEGER DEFAULT 10,           -- Anticipación
    decisions INTEGER DEFAULT 10,              -- Decisiones
    positioning INTEGER DEFAULT 10,            -- Colocación táctica
    off_the_ball INTEGER DEFAULT 10,           -- Desmarques
    work_rate INTEGER DEFAULT 10,              -- Intensidad de trabajo
    
    -- Características psicológicas
    composure INTEGER DEFAULT 10,              -- Calma bajo presión
    concentration INTEGER DEFAULT 10,          -- Concentración
    determination INTEGER DEFAULT 10,          -- Determinación
    bravery INTEGER DEFAULT 10,                -- Valentía
    aggression INTEGER DEFAULT 10,             -- Agresividad
    leadership INTEGER DEFAULT 10,             -- Liderazgo
    teamwork INTEGER DEFAULT 10,               -- Trabajo en equipo
    flair INTEGER DEFAULT 10,                  -- Creatividad/Regate
    
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Atributos Físicos (1-20)
CREATE TABLE player_attributes_physical (
    player_id INTEGER PRIMARY KEY,
    acceleration INTEGER DEFAULT 10,           -- Aceleración (crítico en futsal)
    pace INTEGER DEFAULT 10,                   -- Velocidad máxima
    agility INTEGER DEFAULT 10,                -- Agilidad (cambios de dirección)
    balance INTEGER DEFAULT 10,                -- Equilibrio
    stamina INTEGER DEFAULT 10,                -- Resistencia (CRÍTICO en futsal)
    strength INTEGER DEFAULT 10,               -- Fuerza física
    jumping INTEGER DEFAULT 10,                -- Salto
    
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Atributos de Portero (1-20)
CREATE TABLE player_attributes_goalkeeper (
    player_id INTEGER PRIMARY KEY,
    reflexes INTEGER DEFAULT 10,               -- Reflejos
    handling INTEGER DEFAULT 10,               -- Manejo del balón
    one_on_ones INTEGER DEFAULT 10,            -- Uno contra uno
    positioning_gk INTEGER DEFAULT 10,         -- Colocación de portero
    rushing_out INTEGER DEFAULT 10,            -- Salidas
    throwing INTEGER DEFAULT 10,               -- Saques de mano (contraataque)
    kicking INTEGER DEFAULT 10,                -- Saques con pie
    
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Atributos Ocultos (no visibles completamente sin scouting)
CREATE TABLE player_attributes_hidden (
    player_id INTEGER PRIMARY KEY,
    professionalism INTEGER DEFAULT 10,        -- Profesionalidad (progresión)
    ambition INTEGER DEFAULT 10,               -- Ambición (fichajes)
    loyalty INTEGER DEFAULT 10,                -- Lealtad al club
    pressure INTEGER DEFAULT 10,               -- Juego bajo presión
    injury_proneness INTEGER DEFAULT 10,       -- Propensión a lesiones
    consistency INTEGER DEFAULT 10,            -- Regularidad
    important_matches INTEGER DEFAULT 10,      -- Rendimiento en partidos importantes
    versatility INTEGER DEFAULT 10,            -- Versatilidad posicional
    dirtiness INTEGER DEFAULT 10,              -- Tendencia a cometer faltas
    sportsmanship INTEGER DEFAULT 10,          -- Deportividad
    
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Contratos
CREATE TABLE contracts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,                -- player_id o staff_id
    person_type TEXT CHECK(person_type IN ('player', 'staff')),
    club_id INTEGER NOT NULL,
    wage_weekly REAL NOT NULL,                 -- Salario semanal
    signing_bonus REAL DEFAULT 0,              -- Bonus de fichaje
    release_clause REAL,                       -- Cláusula de rescisión
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    squad_status TEXT CHECK(squad_status IN ('first_team', 'reserve', 'youth', 'on_loan')),
    loan_club_id INTEGER,                      -- Si está cedido
    is_active BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (loan_club_id) REFERENCES clubs(id)
);

-- Historial de clubes del jugador
CREATE TABLE player_club_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    club_id INTEGER NOT NULL,
    date_joined TEXT NOT NULL,
    date_left TEXT,
    transfer_fee REAL DEFAULT 0,
    appearances INTEGER DEFAULT 0,
    goals INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);
```

### **3.5 Cantera y Juveniles**

```sql
-- Categorías juveniles
CREATE TABLE youth_teams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    age_group TEXT CHECK(age_group IN ('U12', 'U14', 'U16', 'U18', 'U20')),
    team_name TEXT,
    coach_id INTEGER,
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);

-- Jugadores de cantera
CREATE TABLE youth_players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    youth_team_id INTEGER NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    date_of_birth TEXT NOT NULL,
    nation_id INTEGER NOT NULL,
    potential_min INTEGER,                   -- Rango de potencial
    potential_max INTEGER,
    current_ability INTEGER DEFAULT 20,      -- 1-200 (bajo en juveniles)
    position_primary TEXT,                   -- POR, CIE, ALA, PIV, UNI
    position_secondary TEXT,
    contract_until TEXT,
    promoted_to_first_team BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (youth_team_id) REFERENCES youth_teams(id),
    FOREIGN KEY (nation_id) REFERENCES nations(id)
);

-- Progresión de juveniles a profesionales
CREATE TABLE youth_promotions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    youth_player_id INTEGER NOT NULL,
    new_player_id INTEGER NOT NULL,          -- ID en tabla players
    promotion_date TEXT NOT NULL,
    FOREIGN KEY (youth_player_id) REFERENCES youth_players(id),
    FOREIGN KEY (new_player_id) REFERENCES players(id)
);
```

### **3.6 Staff y Cuerpo Técnico**

```sql
-- Tipos de staff
CREATE TABLE staff_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,                      -- Manager, Assistant, Scout, Physio, etc.
    max_per_club INTEGER DEFAULT 1
);

-- Staff
CREATE TABLE staff (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    date_of_birth TEXT,
    nation_id INTEGER,
    staff_type_id INTEGER NOT NULL,
    club_id INTEGER,
    wage_weekly REAL,
    contract_until TEXT,
    
    -- Atributos de staff (1-20)
    tactical_knowledge INTEGER DEFAULT 10,
    man_management INTEGER DEFAULT 10,
    judging_player_ability INTEGER DEFAULT 10,
    judging_player_potential INTEGER DEFAULT 10,
    motivating_players INTEGER DEFAULT 10,
    working_with_youngsters INTEGER DEFAULT 10,
    scouting_knowledge INTEGER DEFAULT 10,
    physio_level INTEGER DEFAULT 10,
    
    FOREIGN KEY (nation_id) REFERENCES nations(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (staff_type_id) REFERENCES staff_types(id)
);
```

### **3.7 Ojeo y Scouting**

```sql
-- Centro de ojeo
CREATE TABLE scouting_centers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    nation_id INTEGER NOT NULL,
    region_id INTEGER,
    knowledge_level INTEGER DEFAULT 0,       -- 0-100% conocimiento
    last_scouted_date TEXT,
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (nation_id) REFERENCES nations(id)
);

-- Asignación de scouts
CREATE TABLE scout_assignments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    staff_id INTEGER NOT NULL,
    nation_id INTEGER,
    region_id INTEGER,
    assignment_type TEXT CHECK(assignment_type IN ('nation', 'region', 'club')),
    target_club_id INTEGER,
    start_date TEXT,
    end_date TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (staff_id) REFERENCES staff(id)
);

-- Conocimiento de jugadores (niebla de guerra)
CREATE TABLE player_knowledge (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    knowledge_percentage INTEGER DEFAULT 0,  -- 0-100
    last_updated TEXT,
    known_attributes JSON,                   -- Atributos conocidos (parciales)
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (player_id) REFERENCES players(id),
    UNIQUE(club_id, player_id)
);

-- Informes de scouting
CREATE TABLE scouting_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    scout_id INTEGER,
    report_date TEXT NOT NULL,
    overall_rating INTEGER,                  -- 1-10
    strengths TEXT,                          -- Texto descriptivo
    weaknesses TEXT,
    recommendation TEXT,                     -- "Fichar", "Seguir", "Ignorar"
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (scout_id) REFERENCES staff(id)
);
```

### **3.8 Táctica y Formaciones**

```sql
-- Formaciones de Futsal
CREATE TABLE formations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,               -- "3-1", "4-0", "2-2", "5-0"
    description TEXT,
    defensive_line INTEGER,                  -- 1-100
    pressing_intensity INTEGER,              -- 1-100
    tempo INTEGER,                           -- 1-100
    width INTEGER                            -- 1-100
);

INSERT INTO formations (name, description) VALUES
('3-1', 'Formación clásica con cierre y tres defensas'),
('4-0', 'Rotación total, sin pívot fijo'),
('2-2', 'Equilibrada, dos defensas y dos alaspívot'),
('5-0', 'Ultraofensiva, portero-jugador'),
('4-1', 'Defensiva con pívot anclado');

-- Tácticas del club
CREATE TABLE tactics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    formation_id INTEGER NOT NULL,
    is_default BOOLEAN DEFAULT TRUE,
    
    -- Instrucciones tácticas
    tempo INTEGER DEFAULT 50,                -- 1-100 (Lento -> Rápido)
    pressing_intensity INTEGER DEFAULT 50,   -- 1-100 (Bajo -> Alto)
    defensive_line INTEGER DEFAULT 50,       -- 1-100 (Bajo -> Alto)
    width INTEGER DEFAULT 50,                -- 1-100 (Estrecho -> Ancho)
    
    -- Estilo de juego
    playing_style TEXT CHECK(playing_style IN ('possession', 'counter', 'direct', 'balanced')),
    build_up_play TEXT CHECK(build_up_play IN ('short', 'mixed', 'long')),
    defensive_approach TEXT CHECK(defensive_approach IN ('man_marking', 'zonal', 'mixed')),
    
    -- Powerplay (estrategia 5vs4)
    powerplay_enabled BOOLEAN DEFAULT TRUE,
    powerplay_player_id INTEGER,             -- Jugador de riesgo
    
    -- Saques y estrategias
    corner_taking TEXT,                      -- "Short", "Long", "Mixed"
    free_kick_taking TEXT,
    penalty_takers JSON,                     -- Lista de lanzadores [player_id, priority]
    
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (formation_id) REFERENCES formations(id)
);

-- Roles tácticos por posición
CREATE TABLE tactical_roles (
    tactic_id INTEGER NOT NULL,
    position TEXT NOT NULL,                  -- POR, CIE, ALA, PIV
    player_id INTEGER,                       -- NULL si es genérico
    role_type TEXT,                          -- "Defensive", "Support", "Attack"
    instructions JSON,                       -- Instrucciones específicas
    PRIMARY KEY (tactic_id, position),
    FOREIGN KEY (tactic_id) REFERENCES tactics(id),
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Automatísmos tácticos (entrenamientos)
CREATE TABLE tactical_automatisms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    name TEXT NOT NULL,                      -- "3-1 Presión Alta", "4-0 Rotación"
    formation_base TEXT,
    description TEXT,
    effectiveness INTEGER DEFAULT 50,        -- 1-100 (mejora con entrenamiento)
    last_trained TEXT,
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);
```

### **3.9 Partidos y Calendario**

```sql
-- Partidos
CREATE TABLE matches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    competition_id INTEGER NOT NULL,
    season TEXT NOT NULL,
    round INTEGER,                           -- Jornada
    date TEXT NOT NULL,                      -- ISO8601
    time TEXT,
    home_club_id INTEGER NOT NULL,
    away_club_id INTEGER NOT NULL,
    stadium_id INTEGER,
    attendance INTEGER,
    status TEXT CHECK(status IN ('scheduled', 'live', 'finished', 'postponed', 'cancelled')),
    
    -- Resultado
    home_score INTEGER DEFAULT 0,
    away_score INTEGER DEFAULT 0,
    home_score_first_half INTEGER DEFAULT 0,
    away_score_first_half INTEGER DEFAULT 0,
    
    -- Estadísticas del partido
    home_possession INTEGER,                 -- 0-100
    away_possession INTEGER,
    home_shots INTEGER DEFAULT 0,
    away_shots INTEGER DEFAULT 0,
    home_shots_on_target INTEGER DEFAULT 0,
    away_shots_on_target INTEGER DEFAULT 0,
    home_fouls INTEGER DEFAULT 0,
    away_fouls INTEGER DEFAULT 0,
    home_yellow_cards INTEGER DEFAULT 0,
    away_yellow_cards INTEGER DEFAULT 0,
    home_red_cards INTEGER DEFAULT 0,
    away_red_cards INTEGER DEFAULT 0,
    home_corner_kicks INTEGER DEFAULT 0,     -- Saques de esquina (poco comunes)
    away_corner_kicks INTEGER DEFAULT 0,
    home_6th_foul_count INTEGER DEFAULT 0,   -- Contador de 6ª falta
    away_6th_foul_count INTEGER DEFAULT 0,
    
    -- Powerplay
    home_powerplay_goals INTEGER DEFAULT 0,
    away_powerplay_goals INTEGER DEFAULT 0,
    
    FOREIGN KEY (competition_id) REFERENCES competitions(id),
    FOREIGN KEY (home_club_id) REFERENCES clubs(id),
    FOREIGN KEY (away_club_id) REFERENCES clubs(id),
    FOREIGN KEY (stadium_id) REFERENCES stadiums(id)
);

-- Eventos del partido (timeline)
CREATE TABLE match_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id INTEGER NOT NULL,
    minute INTEGER NOT NULL,                 -- Minuto del partido (0-40+)
    second INTEGER DEFAULT 0,                -- Segundo dentro del minuto
    event_type TEXT NOT NULL,
    player_id INTEGER,
    club_id INTEGER NOT NULL,
    description TEXT,
    x_coordinate REAL,                       -- Posición X del evento (0-40)
    y_coordinate REAL,                       -- Posición Y del evento (0-20)
    FOREIGN KEY (match_id) REFERENCES matches(id),
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);

-- Tipos de eventos
-- 'goal', 'shot', 'shot_on_target', 'shot_off_target', 'save',
-- 'foul', 'yellow_card', 'red_card', 'substitution', 'timeout',
-- '6th_foul', 'double_penalty', 'powerplay_goal', 'powerplay_start',
-- 'possession_won', 'possession_lost', 'pass_completed', 'pass_failed'

-- Alineaciones del partido
CREATE TABLE match_lineups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id INTEGER NOT NULL,
    club_id INTEGER NOT NULL,
    formation_used TEXT,
    starting_five JSON,                      -- [player_id, player_id, ...]
    substitutes JSON,                        -- Jugadores en banquillo
    coach_id INTEGER,
    tactics_snapshot JSON,                   -- Copia de tácticas al inicio
    FOREIGN KEY (match_id) REFERENCES matches(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);

-- Estadísticas individuales por partido
CREATE TABLE match_player_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    club_id INTEGER NOT NULL,
    started BOOLEAN DEFAULT FALSE,
    minutes_played INTEGER DEFAULT 0,
    goals INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    shots INTEGER DEFAULT 0,
    shots_on_target INTEGER DEFAULT 0,
    passes_completed INTEGER DEFAULT 0,
    passes_attempted INTEGER DEFAULT 0,
    pass_accuracy REAL,
    dribbles_completed INTEGER DEFAULT 0,
    dribbles_attempted INTEGER DEFAULT 0,
    tackles_won INTEGER DEFAULT 0,
    interceptions INTEGER DEFAULT 0,
    fouls_committed INTEGER DEFAULT 0,
    fouls_suffered INTEGER DEFAULT 0,
    yellow_cards INTEGER DEFAULT 0,
    red_cards INTEGER DEFAULT 0,
    rating REAL,                             -- 1-10
    distance_covered_m REAL,                 -- Metros recorridos
    sprints INTEGER DEFAULT 0,
    FOREIGN KEY (match_id) REFERENCES matches(id),
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);

-- Clasificación de ligas
CREATE TABLE league_standings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    competition_id INTEGER NOT NULL,
    season TEXT NOT NULL,
    club_id INTEGER NOT NULL,
    position INTEGER,
    played INTEGER DEFAULT 0,
    won INTEGER DEFAULT 0,
    drawn INTEGER DEFAULT 0,
    lost INTEGER DEFAULT 0,
    goals_for INTEGER DEFAULT 0,
    goals_against INTEGER DEFAULT 0,
    goal_difference INTEGER DEFAULT 0,
    points INTEGER DEFAULT 0,
    form_last_5 TEXT,                        -- "WDLWW"
    home_played INTEGER DEFAULT 0,
    home_won INTEGER DEFAULT 0,
    home_drawn INTEGER DEFAULT 0,
    home_lost INTEGER DEFAULT 0,
    away_played INTEGER DEFAULT 0,
    away_won INTEGER DEFAULT 0,
    away_drawn INTEGER DEFAULT 0,
    away_lost INTEGER DEFAULT 0,
    FOREIGN KEY (competition_id) REFERENCES competitions(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    UNIQUE(competition_id, season, club_id)
);

-- Goleadores y estadísticas de competición
CREATE TABLE competition_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    competition_id INTEGER NOT NULL,
    season TEXT NOT NULL,
    stat_type TEXT CHECK(stat_type IN ('top_scorer', 'top_assists', 'clean_sheets')),
    player_id INTEGER,
    club_id INTEGER,
    value INTEGER DEFAULT 0,
    FOREIGN KEY (competition_id) REFERENCES competitions(id),
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);
```

### **3.10 Entrenamientos**

```sql
-- Tipos de entrenamiento
CREATE TABLE training_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,                      -- "Técnica", "Táctica", "Físico", etc.
    category TEXT CHECK(category IN ('technical', 'tactical', 'physical', 'goalkeeper')),
    intensity INTEGER,                       -- 1-100
    fatigue_impact INTEGER,                  -- 1-100
    attributes_improved JSON                 -- ["passing", "dribbling", ...]
);

-- Programa de entrenamiento semanal
CREATE TABLE training_schedule (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    day_of_week INTEGER CHECK(day_of_week BETWEEN 0 AND 6),  -- 0=Lunes, 6=Domingo
    training_type_id INTEGER NOT NULL,
    duration_minutes INTEGER DEFAULT 90,
    intensity INTEGER DEFAULT 50,            -- 1-100
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (training_type_id) REFERENCES training_types(id)
);

-- Sesiones de entrenamiento completadas
CREATE TABLE training_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    date TEXT NOT NULL,
    training_type_id INTEGER NOT NULL,
    attendance INTEGER,                      -- Número de jugadores
    average_intensity INTEGER,
    effectiveness INTEGER,                   -- 1-100
    injuries_caused INTEGER DEFAULT 0,
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (training_type_id) REFERENCES training_types(id)
);

-- Progreso individual en entrenamientos
CREATE TABLE player_training_progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    training_type_id INTEGER NOT NULL,
    sessions_completed INTEGER DEFAULT 0,
    improvement_rate REAL DEFAULT 0,         -- Mejora por sesión
    last_trained TEXT,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (training_type_id) REFERENCES training_types(id)
);
```

### **3.11 Comunicaciones e Inbox**

```sql
-- Mensajes del sistema
CREATE TABLE inbox_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    sender_type TEXT CHECK(sender_type IN ('system', 'player', 'staff', 'board', 'media')),
    sender_id INTEGER,
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    date_sent TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    is_important BOOLEAN DEFAULT FALSE,
    requires_action BOOLEAN DEFAULT FALSE,
    action_type TEXT,                        -- "contract_offer", "transfer_offer", etc.
    action_data JSON,                        -- Datos para la acción
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);

-- Noticias de medios
CREATE TABLE news_articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    date_published TEXT NOT NULL,
    source TEXT,                             -- "Prensa local", "Marca", etc.
    related_club_id INTEGER,
    related_player_id INTEGER,
    related_competition_id INTEGER,
    importance INTEGER DEFAULT 50,           -- 1-100
    FOREIGN KEY (related_club_id) REFERENCES clubs(id),
    FOREIGN KEY (related_player_id) REFERENCES players(id),
    FOREIGN KEY (related_competition_id) REFERENCES competitions(id)
);
```

### **3.12 Transferencias y Mercado**

```sql
-- Ofertas de transferencia
CREATE TABLE transfer_offers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    from_club_id INTEGER NOT NULL,
    to_club_id INTEGER NOT NULL,
    offered_fee REAL NOT NULL,
    wage_offered REAL,
    contract_length_years INTEGER,
    status TEXT CHECK(status IN ('pending', 'accepted', 'rejected', 'withdrawn')),
    date_offered TEXT NOT NULL,
    date_response TEXT,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (from_club_id) REFERENCES clubs(id),
    FOREIGN KEY (to_club_id) REFERENCES clubs(id)
);

-- Historial de transferencias
CREATE TABLE transfer_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    from_club_id INTEGER,
    to_club_id INTEGER NOT NULL,
    transfer_date TEXT NOT NULL,
    transfer_fee REAL DEFAULT 0,
    transfer_type TEXT CHECK(transfer_type IN ('permanent', 'loan', 'free')),
    contract_length_years INTEGER,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (from_club_id) REFERENCES clubs(id),
    FOREIGN KEY (to_club_id) REFERENCES clubs(id)
);

-- Lista de transferibles
CREATE TABLE transfer_list (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    asking_price REAL,
    listed_date TEXT NOT NULL,
    status TEXT CHECK(status IN ('listed', 'offer_received', 'sold', 'removed')),
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (player_id) REFERENCES players(id)
);

-- Intereses de fichaje
CREATE TABLE transfer_interests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id INTEGER NOT NULL,
    player_id INTEGER NOT NULL,
    interest_level INTEGER DEFAULT 50,       -- 1-100
    last_checked TEXT,
    FOREIGN KEY (club_id) REFERENCES clubs(id),
    FOREIGN KEY (player_id) REFERENCES players(id)
);
```

### **3.13 Lesiones y Sanciones**

```sql
-- Lesiones
CREATE TABLE injuries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    injury_type TEXT NOT NULL,               -- "Ankle", "Knee", "Hamstring", etc.
    severity INTEGER NOT NULL,               -- 1-100
    expected_return_date TEXT,
    actual_return_date TEXT,
    injury_date TEXT NOT NULL,
    occurred_in_match_id INTEGER,
    occurred_in_training BOOLEAN DEFAULT FALSE,
    description TEXT,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (occurred_in_match_id) REFERENCES matches(id)
);

-- Sanciones (tarjetas)
CREATE TABLE suspensions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    club_id INTEGER NOT NULL,
    reason TEXT NOT NULL,                    -- "Yellow cards accumulation", "Red card", etc.
    matches_banned INTEGER NOT NULL,
    matches_served INTEGER DEFAULT 0,
    start_date TEXT NOT NULL,
    end_date TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (club_id) REFERENCES clubs(id)
);

-- Historial disciplinario
CREATE TABLE disciplinary_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    match_id INTEGER,
    card_type TEXT CHECK(card_type IN ('yellow', 'red', 'second_yellow')),
    minute INTEGER,
    reason TEXT,
    date TEXT NOT NULL,
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (match_id) REFERENCES matches(id)
);
```

---

## **4. MOTOR DE PARTIDO 2D - ESPECIFICACIONES TÉCNICAS**

### **4.1 Arquitectura del Motor**

```rust
// Estructura ECS (Entity Component System) simplificada
pub struct MatchEngine {
    pub tick_rate: u8,              // 60 ticks por segundo
    pub match_time: u32,            // Tiempo en segundos (0-2400 = 40 min)
    pub current_half: u8,           // 1 o 2
    pub entities: Vec<Entity>,
    pub systems: Vec<System>,
    pub match_state: MatchState,
}

pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,    // Player, Ball, Referee
    pub components: Components,
}

pub struct Components {
    pub position: Position,         // x, y (0-40, 0-20)
    pub velocity: Velocity,         // vx, vy
    pub attributes: PlayerAttrs,    // Referencia a BD
    pub stamina: f32,               // 0-100
    pub state: PlayerState,         // Running, Sprinting, Tired, etc.
}

pub enum MatchState {
    PreMatch,
    FirstHalf,
    HalfTime,
    SecondHalf,
    ExtraTime,
    Penalties,
    Finished,
}
```

### **4.2 Reglas Específicas de Futsal Implementadas**

```rust
pub struct FutsalRules {
    // Tiempo
    pub match_duration_minutes: u8,     // 40 (2x20)
    pub half_time_minutes: u8,          // 10
    pub timeout_per_team: u8,           // 1 por tiempo
    pub timeout_duration_seconds: u8,   // 60
    
    // Faltas
    pub team_fouls_half: u8,            // Contador por tiempo
    pub sixth_foul_penalty: bool,       // Doble penalti desde 10m
    
    // Sustituciones
    pub max_substitutes: u8,            // Ilimitadas (volantes)
    pub substitution_zone: bool,        // Zona de 5m
    
    // Powerplay
    pub powerplay_enabled: bool,        // Portero-jugador
    pub powerplay_min_players: u8,      // 4 (5vs4)
    
    // Tarjetas
    pub yellow_card_sinbin: bool,       // 2 minutos (opcional)
    pub five_fouls_red: bool,           // 5ª falta = roja (regla antigua)
    
    // Saques
    pub kick_in_time_seconds: u8,       // 4 segundos
    pub goalkeeper_possession_seconds: u8, // 4 segundos en su campo
}
```

### **4.3 Lógica de Resolución de Acciones**

```rust
// Sistema de resolución de duelos
pub fn resolve_duel(attacker: &Player, defender: &Player, action: ActionType) -> f32 {
    let attacker_rating = match action {
        ActionType::Dribble => {
            (attacker.dribbling * 0.4 + 
             attacker.agility * 0.3 + 
             attacker.balance * 0.3) as f32
        },
        ActionType::Shot => {
            (attacker.finishing * 0.5 + 
             attacker.composure * 0.3 + 
             attacker.technique * 0.2) as f32
        },
        ActionType::Pass => {
            (attacker.passing * 0.5 + 
             attacker.vision * 0.3 + 
             attacker.first_touch * 0.2) as f32
        },
        _ => 0.0
    };
    
    let defender_rating = match action {
        ActionType::Dribble => {
            (defender.tackling * 0.4 + 
             defender.positioning * 0.3 + 
             defender.anticipation * 0.3) as f32
        },
        ActionType::Shot => {
            (defender.blocking * 0.5 + 
             defender.positioning * 0.3 + 
             defender.bravery * 0.2) as f32
        },
        _ => 0.0
    };
    
    // Ruido gaussiano para variabilidad (0.8 - 1.2)
    let noise = rand::thread_rng().gen_range(0.8..1.2);
    
    (attacker_rating / defender_rating) * noise
}

// Cálculo de probabilidad de gol
pub fn calculate_goal_probability(
    shooter: &Player,
    goalkeeper: &Player,
    distance: f32,          // metros (0-20)
    angle: f32,             // grados (0-90)
    is_powerplay: bool
) -> f32 {
    let base_probability = match distance {
        d if d < 3.0 => 0.7,           // 0-3m: 70% base
        d if d < 6.0 => 0.4,           // 3-6m: 40% base
        d if d < 10.0 => 0.2,          // 6-10m: 20% base
        _ => 0.05                       // +10m: 5% base
    };
    
    // Modificadores
    let angle_modifier = angle / 90.0;  // 0-1
    let skill_modifier = (shooter.finishing as f32 / 20.0) * 0.5 +
                        (shooter.composure as f32 / 20.0) * 0.3 +
                        (shooter.technique as f32 / 20.0) * 0.2;
    
    let goalkeeper_modifier = 1.0 - (goalkeeper.reflexes as f32 / 40.0);
    
    let powerplay_modifier = if is_powerplay { 1.3 } else { 1.0 };
    
    base_probability * angle_modifier * skill_modifier * goalkeeper_modifier * powerplay_modifier
}
```

### **4.4 Sistema de Fatiga (CRÍTICO en Futsal)**

```rust
pub fn update_player_stamina(player: &mut Player, action: Action, duration: f32) {
    let stamina_drain = match action {
        Action::Walking => 0.01 * duration,
        Action::Jogging => 0.03 * duration,
        Action::Running => 0.06 * duration,
        Action::Sprinting => 0.12 * duration,
        Action::HighIntensity => 0.15 * duration,
    };
    
    // Modificador por condición física
    let fitness_modifier = player.match_fitness as f32 / 100.0;
    
    player.stamina -= stamina_drain * (2.0 - fitness_modifier);
    
    // Recuperación pasiva (cuando no tiene el balón)
    if !player.has_possession {
        player.stamina += 0.02 * duration;
    }
    
    // Clamp 0-100
    player.stamina = player.stamina.clamp(0.0, 100.0);
    
    // Efectos en atributos cuando está cansado
    if player.stamina < 30.0 {
        player.effective_attributes *= 0.7;  // -30% rendimiento
    } else if player.stamina < 60.0 {
        player.effective_attributes *= 0.85; // -15% rendimiento
    }
}

// Sistema de rotaciones automáticas (IA)
pub fn should_substitute(player: &Player, match_time: u32) -> bool {
    // En futsal, los cambios son cada 3-5 minutos típicamente
    let minutes_played = match_time / 60;
    
    if player.stamina < 40.0 {
        return true;
    }
    
    if minutes_played > 5 && player.stamina < 60.0 {
        return true;
    }
    
    // Cambio táctico programado
    if player.minutes_played >= 15 && player.stamina < 70.0 {
        return true;
    }
    
    false
}
```

### **4.5 Visualización 2D (Canvas/Konva.js)**

```typescript
// Componente React para el campo
interface MatchViewProps {
  matchId: number;
  isLive: boolean;
  tickRate: number; // 60 TPS
}

const FutsalPitch: React.FC<MatchViewProps> = ({ matchId, isLive, tickRate }) => {
  const [entities, setEntities] = useState<Entity[]>([]);
  const [matchState, setMatchState] = useState<MatchState>();
  
  // Dimensiones del campo (40x20 metros escalados)
  const PITCH_WIDTH = 800;   // px
  const PITCH_HEIGHT = 400;  // px
  const SCALE = 20;          // 1 metro = 20px
  
  // Renderizado de entidades
  const renderPlayer = (player: PlayerEntity) => (
    <Group key={player.id}>
      {/* Círculo del jugador */}
      <Circle
        x={player.x * SCALE}
        y={player.y * SCALE}
        radius={8}
        fill={player.teamColor}
        stroke="#000"
        strokeWidth={2}
        opacity={player.stamina < 30 ? 0.6 : 1.0}
      />
      {/* Dorsal */}
      <Text
        x={player.x * SCALE}
        y={player.y * SCALE}
        text={player.shirtNumber.toString()}
        fontSize={10}
        fill="#fff"
        align="center"
        offsetX={5}
        offsetY={3}
      />
      {/* Indicador de fatiga */}
      {player.stamina < 50 && (
        <Rect
          x={player.x * SCALE - 8}
          y={player.y * SCALE - 12}
          width={16 * (player.stamina / 100)}
          height={3}
          fill={player.stamina < 30 ? "#f00" : "#ff0"}
        />
      )}
    </Group>
  );
  
  return (
    <Stage width={PITCH_WIDTH} height={PITCH_HEIGHT}>
      <Layer>
        {/* Campo */}
        <Rect x={0} y={0} width={PITCH_WIDTH} height={PITCH_HEIGHT} fill="#2d8a2d" />
        
        {/* Líneas del campo */}
        <Rect x={10} y={10} width={PITCH_WIDTH-20} height={PITCH_HEIGHT-20} 
              stroke="#fff" strokeWidth={2} fill={null} />
        
        {/* Línea central */}
        <Line points={[PITCH_WIDTH/2, 10, PITCH_WIDTH/2, PITCH_HEIGHT-10]} 
              stroke="#fff" strokeWidth={2} />
        
        {/* Círculo central */}
        <Circle x={PITCH_WIDTH/2} y={PITCH_HEIGHT/2} radius={40} 
                stroke="#fff" strokeWidth={2} fill={null} />
        
        {/* Áreas de penalti (6m) */}
        <Rect x={10} y={120} width={120} height={160} 
              stroke="#fff" strokeWidth={2} fill={null} />
        <Rect x={PITCH_WIDTH-130} y={120} width={120} height={160} 
              stroke="#fff" strokeWidth={2} fill={null} />
        
        {/* Puntos de penalti */}
        <Circle x={130} y={PITCH_HEIGHT/2} radius={3} fill="#fff" />
        <Circle x={PITCH_WIDTH-130} y={PITCH_HEIGHT/2} radius={3} fill="#fff" />
        
        {/* Punto de doble penalti (10m) */}
        <Circle x={200} y={PITCH_HEIGHT/2} radius={3} fill="#fff" />
        <Circle x={PITCH_WIDTH-200} y={PITCH_HEIGHT/2} radius={3} fill="#fff" />
        
        {/* Entidades (jugadores y balón) */}
        {entities.map(entity => 
          entity.type === 'player' ? renderPlayer(entity) : renderBall(entity)
        )}
      </Layer>
    </Stage>
  );
};
```

---

## **5. SISTEMAS DE JUEGO PRINCIPALES**

### **5.1 Bucle de Avance de Tiempo (Time Processing)**

```rust
pub struct TimeProcessor {
    pub current_date: DateTime,
    pub game_speed: GameSpeed,  // Pause, Normal, x2, x5, Maximum
}

pub enum GameSpeed {
    Pause,
    Normal,     // 1 segundo real = 1 minuto juego
    Fast,       // 1 segundo real = 5 minutos juego
    Maximum,    // Sin renderizado, solo cálculos
}

impl TimeProcessor {
    pub fn process_day(&mut self, club_id: u32) -> Vec<GameEvent> {
        let mut events = Vec::new();
        
        // 1. Recuperación física de jugadores
        events.extend(self.recover_players(club_id));
        
        // 2. Entrenamientos del día
        events.extend(self.process_training(club_id));
        
        // 3. Progreso de cantera
        events.extend(self.process_youth_development(club_id));
        
        // 4. Partidos del día (simulación o visualización)
        let matches = self.get_matches_for_date(self.current_date);
        for match in matches {
            events.extend(self.process_match(match));
        }
        
        // 5. Mercado de fichajes (IA)
        events.extend(self.process_transfer_market());
        
        // 6. Generación de noticias
        events.extend(self.generate_news());
        
        // 7. Actualización de clasificaciones
        self.update_standings();
        
        // 8. Avance de fecha
        self.current_date += Duration::days(1);
        
        events
    }
}
```

### **5.2 Sistema de Ojeo Limitado**

```rust
pub struct ScoutingSystem {
    max_scouts: u8,              // Limitado por nivel de club
    max_nations: u8,             // Máximo países a otear simultáneamente
}

impl ScoutingSystem {
    pub fn assign_scout(
        &mut self,
        club_id: u32,
        scout_id: u32,
        target: ScoutingTarget
    ) -> Result<(), ScoutingError> {
        // Verificar límite de scouts
        let active_assignments = self.get_active_assignments(club_id);
        if active_assignments >= self.max_scouts {
            return Err(ScoutingError::MaxScoutsReached);
        }
        
        // Crear asignación
        self.create_assignment(club_id, scout_id, target)
    }
    
    pub fn update_knowledge(&mut self, club_id: u32, player_id: u32) {
        // Incrementar conocimiento basado en:
        // - Nivel del scout
        // - Tiempo de ojeo
        // - Calidad del jugador (más difícil otear estrellas)
        
        let current_knowledge = self.get_knowledge(club_id, player_id);
        let increment = calculate_knowledge_increment(
            self.scout_ability,
            current_knowledge
        );
        
        self.set_knowledge(club_id, player_id, current_knowledge + increment);
    }
    
    pub fn get_player_attributes_for_club(
        &self,
        club_id: u32,
        player_id: u32
    ) -> DisplayedAttributes {
        let knowledge = self.get_knowledge(club_id, player_id);
        let real_attributes = self.get_real_attributes(player_id);
        
        if knowledge >= 100 {
            // Mostrar atributos exactos
            DisplayedAttributes::Exact(real_attributes)
        } else if knowledge >= 70 {
            // Mostrar rangos estrechos (±1)
            DisplayedAttributes::NarrowRange(real_attributes, 1)
        } else if knowledge >= 40 {
            // Mostrar rangos medios (±3)
            DisplayedAttributes::MediumRange(real_attributes, 3)
        } else {
            // Mostrar rangos amplios (±5) o solo estimación
            DisplayedAttributes::WideRange(real_attributes, 5)
        }
    }
}
```

### **5.3 Progresión y Desarrollo de Jugadores**

```rust
pub struct PlayerDevelopment {
    age: u8,
    current_ability: u16,
    potential_ability: u16,
    professionalism: u8,
    training_facilities: u8,
}

impl PlayerDevelopment {
    pub fn calculate_weekly_improvement(&mut self) -> Vec<AttributeChange> {
        let mut changes = Vec::new();
        
        // Factor edad
        let age_factor = match self.age {
            15..=18 => 1.5,      // Crecimiento rápido
            19..=23 => 1.2,      // Crecimiento medio
            24..=28 => 0.8,      // Estabilización
            29..=32 => 0.5,      // Declive lento
            _ => 0.2,            // Declive rápido
        };
        
        // Factor potencial
        let potential_gap = self.potential_ability - self.current_ability;
        let potential_factor = if potential_gap > 50 {
            1.3
        } else if potential_gap > 20 {
            1.0
        } else {
            0.7
        };
        
        // Factor profesionalidad
        let professionalism_factor = self.professionalism as f32 / 20.0;
        
        // Factor instalaciones
        let facilities_factor = self.training_facilities as f32 / 100.0;
        
        // Calcular mejora por atributo
        for attribute in self.attributes.iter_mut() {
            if attribute.current < attribute.potential {
                let improvement = 0.1 * age_factor * potential_factor * 
                                 professionalism_factor * facilities_factor;
                
                attribute.current += improvement;
                changes.push(AttributeChange {
                    attribute: attribute.name,
                    change: improvement,
                });
            }
        }
        
        changes
    }
    
    pub fn check_peak_age(&self) -> bool {
        // En futsal, el peak suele ser 26-30 años
        (26..=30).contains(&self.age)
    }
}
```

### **5.4 Sistema de Moral y Felicidad**

```rust
pub struct PlayerMorale {
    pub happiness: i8,           // -100 a +100
    pub confidence: i8,          // -100 a +100
    pub team_chemistry: i8,      // -100 a +100
}

impl PlayerMorale {
    pub fn update(&mut self, factors: MoraleFactors) {
        // Factores positivos
        if factors.recent_form == "Excellent" {
            self.confidence += 5;
        }
        if factors.playing_time > 80 {  // % minutos jugados
            self.happiness += 2;
        }
        if factors.team_winning_streak > 3 {
            self.happiness += 3;
            self.confidence += 3;
        }
        if factors.contract_happy {
            self.happiness += 1;
        }
        
        // Factores negativos
        if factors.recent_form == "Poor" {
            self.confidence -= 5;
        }
        if factors.playing_time < 20 {
            self.happiness -= 5;
        }
        if factors.team_losing_streak > 3 {
            self.happiness -= 3;
            self.confidence -= 3;
        }
        if factors.transfer_request_denied {
            self.happiness -= 10;
        }
        if factors.wage_below_expectations {
            self.happiness -= 3;
        }
        
        // Clamp
        self.happiness = self.happiness.clamp(-100, 100);
        self.confidence = self.confidence.clamp(-100, 100);
    }
    
    pub fn get_morale_state(&self) -> MoraleState {
        let average = (self.happiness + self.confidence) / 2;
        
        match average {
            80..=100 => MoraleState::Superb,
            60..=79 => MoraleState::Excellent,
            40..=59 => MoraleState::Good,
            20..=39 => MoraleState::Decent,
            0..=19 => MoraleState::Average,
            -20..=-1 => MoraleState::Poor,
            -40..=-21 => MoraleState::Bad,
            -60..=-41 => MoraleState::VeryBad,
            _ => MoraleState::Appalling,
        }
    }
}
```

---

## **6. INTERFAZ DE USUARIO - PANTALLAS PRINCIPALES**

### **6.1 Dashboard Principal**

```
─────────────────────────────────────────────────────────────┐
│  FUTSAL MANAGER 2026                    [Fecha] [Dinero]   │
─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ PRÓXIMO     │  │ ÚLTIMO      │  │ CLASIFIC.   │        │
│  │ PARTIDO     │  │ RESULTADO   │  │             │        │
│  │             │  │             │  │  1. Barça   │        │
│  │ vs Real     │  │ 4-2         │  │  2. Inter   │        │
│  │ Madrid      │  │ ✓ Victoria  │  │  3. Tú (5º) │        │
│  │ [Ver]       │  │ [Crónica]   │  │  4. ElPozo  │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                             │
│  ─────────────────────────────────────────────────────┐   │
│  │ BANDEJA DE ENTRADA (3)                              │   │
│  │ • Oferta de traspaso: Jugador X                     │   │
│  │ • Lesión: Juan García (2 semanas)                   │   │
│  │ • Junta directiva: Presupuesto aumentado            │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ JUGADORES   │  │ ENTRENAM.   │  │ MERCADO     │        │
│  │ Lesionados  │  │ Progreso    │  │ Ofertas     │        │
│  │ (2)         │  │ Semanal     │  │ Activas (3) │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### **6.2 Pantalla de Partido en Vivo**

```
┌─────────────────────────────────────────────────────────────┐
│  Barcelona 4-2 Real Madrid         [35:42] [2ª Parte]      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│           ┌─────────────────────────────────┐              │
│           │                                 │              │
│           │         CAMPO 2D                │              │
│           │      (Motor en tiempo real)     │              │
│           │                                 │              │
│           └─────────────────────────────────┘              │
│                                                             │
│  ┌─────────────────┐           ┌─────────────────┐        │
│  │ ESTADÍSTICAS    │           │ EVENTOS         │        │
│  │                 │           │                 │        │
│  │ Posesión: 55%   │           │ 34' GOL Barça   │        │
│  │ Tiros: 12-8     │           │ 32' AMARILLA    │        │
│  │ Faltas: 8-11    │           │ 28' GOL Madrid  │        │
│  │ 6ª Faltas: 5-4  │           │ 25' TIMEOUT     │        │
│  │                 │           │ 20' GOL Barça   │        │
│  └─────────────────┘           └─────────────────┘        │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ ALINEACIÓN Y CAMBIOS                                │   │
│  │                                                     │   │
│  │ POR: Ferrán Valera        [Cambiar]                │   │
│  │ CIE: Adolfo Fernández     [Cambiar]                │   │
│  │ ALA: Sergio Lozano        [Cambiar] [Fatiga: 45%]  │   │
│  │ ALA: Esquerdinha          [Cambiar]                │   │
│  │ PIV: Ferrao               [Cambiar]                │   │
│  │                                                     │   │
│  │ Banquillo: [Arrastrar para cambiar]                │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  [Táctica] [Instrucciones] [Tiempos Muertos] [Pausa/Play] │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### **6.3 Pizarra Táctica**

```
┌─────────────────────────────────────────────────────────────┐
│  PIZARRA TÁCTICA - Barcelona                    [Guardar]   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Formación: [3-1 ▼]  [4-0]  [2-2]  [5-0]                  │
│                                                             │
│           ┌─────────────────────────────────┐              │
│           │                                 │              │
│           │     [POR] Ferrán                │              │
│           │                                 │              │
│           │  [ALA]     [CIE]     [ALA]      │              │
│           │   Lozano   Adolfo   Esquerdinha │              │
│           │                                 │              │
│           │           [PIV]                 │              │
│           │           Ferrao                │              │
│           │                                 │              │
│           └─────────────────────────────────┘              │
│                                                             │
│  Instrucciones Tácticas:                                   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Tempo:            [====|========] Medio-Rápido     │   │
│  │ Presión:          [=======|===] Alta               │   │
│  │ Línea Defensiva:  [====|========] Media            │   │
│  │ Amplitud:         [=====|=======] Equilibrada      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  Estilo de Juego:                                          │
│  [ ] Posesión  [✓] Contraataque  [ ] Directo             │
│                                                             │
│  Powerplay:                                                │
│  [✓] Activar   Jugador: [Ferrao ▼]                        │
│                                                             │
│  Automatísmos Entrenados:                                  │
│  • 3-1 Presión Alta (85%)  • 4-0 Rotación (72%)           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### **6.4 Pantalla de Entrenamientos**

```
┌─────────────────────────────────────────────────────────────┐
│  ENTRENAMIENTOS - Semana 24                     [Programar] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ PROGRAMACIÓN SEMANAL                                │   │
│  │                                                     │   │
│  │ Lunes:    [Técnica Individual    ] [Intensidad: 70%]│   │
│  │ Martes:   [Táctica 3-1           ] [Intensidad: 85%]│   │
│  │ Miércoles:[Físico - Resistencia  ] [Intensidad: 60%]│   │
│  │ Jueves:   [Automatismos 4-0      ] [Intensidad: 75%]│   │
│  │ Viernes:  [Partido Entrenamiento ] [Intensidad: 50%]│   │
│  │ Sábado:   [PARTIDO vs Real Madrid]                  │   │
│  │ Domingo:  [Descanso              ]                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ AUTOMATISMOS TÁCTICOS ENTRENADOS                    │   │
│  │                                                     │   │
│  │ • 3-1 Presión Alta        [████████░░] 85%         │   │
│  │ • 4-0 Rotación Continua   [███████░░░] 72%         │   │
│  │ • Powerplay Ofensivo      [██████░░░░] 60%         │   │
│  │ • Saques de Banda         [█████████░] 90%         │   │
│  │                                                     │   │
│  │ [Entrenar Nuevo Automatismo]                        │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ PROGRESO INDIVIDUAL                                 │   │
│  │                                                     │   │
│  │ Jugador       │ Técnica │ Táctica │ Físico         │   │
│  │ Ferrao        │ ███████ │ █████░  │ ██████         │   │
│  │ Lozano        │ ██████  │ ███████ │ █████          │   │
│  │ Adolfo        │ █████   │ ███████ │ ██████         │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### **6.5 Pantalla de Scouting/Ojeo**

```
┌─────────────────────────────────────────────────────────────┐
│  CENTRO DE OJEO - Recursos Limitados            [Informes]  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  RECURSOS DISPONIBLES:                                     │
│  • Scouts activos: 3/5                                     │
│  • Países oteados: 2/4                                     │
│                                                             │
│  ─────────────────────────────────────────────────────┐   │
│  │ PAÍSES EN OJEO                                      │   │
│  │                                                     │   │
│  │ 🇧🇷 Brasil          [████████░░] 80% conocimiento   │   │
│  │   • Liga LNF completa                               │   │
│  │   • 142 jugadores conocidos                         │   │
│  │   • Scout: Carlos Ruiz                              │   │
│  │                                                     │   │
│  │ 🇪🇸 España          [██████░░░░] 60% conocimiento   │   │
│  │   • Primera División                                │   │
│  │   • 89 jugadores conocidos                          │   │
│  │   • Scout: Miguel Ángel                             │   │
│  ─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ JUGADORES DESTACADOS (Conocimiento Parcial)         │   │
│  │                                                     │   │
│  │ Nombre        │ Pos │ Edad │ CA    │ Valor         │   │
│  │ Pito          │ PIV │ 28   │ 145-160│ €450K-600K   │   │
│  │ Taynan        │ ALA │ 26   │ 140-155│ €380K-520K   │   │
│  │ [Ver informe completo]                              │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  [Asignar Scout] [Ver Informes] [Buscar Jugadores]        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## **7. ECONOMÍA Y MERCADO DE FICHAJES**

### **7.1 Valoración de Jugadores**

```rust
pub fn calculate_player_value(player: &Player) -> f32 {
    let base_value = (player.current_ability as f32).powf(2.0) * 100.0;
    
    // Modificador edad
    let age_modifier = match player.age {
        15..=20 => 1.3,      // Joven con potencial
        21..=27 => 1.0,      // Edad prime
        28..=32 => 0.8,      // Veterano
        _ => 0.5,            // Muy mayor
    };
    
    // Modificador potencial
    let potential_gap = player.potential_ability - player.current_ability;
    let potential_modifier = if potential_gap > 30 {
        1.4
    } else if potential_gap > 15 {
        1.2
    } else {
        1.0
    };
    
    // Modificador contrato
    let contract_modifier = if player.contract_years_left < 1 {
        0.6
    } else if player.contract_years_left < 2 {
        0.8
    } else {
        1.0
    };
    
    // Modificador forma
    let form_modifier = match player.recent_form {
        "Excellent" => 1.2,
        "Good" => 1.1,
        "Average" => 1.0,
        "Poor" => 0.8,
        _ => 0.9,
    };
    
    base_value * age_modifier * potential_modifier * contract_modifier * form_modifier
}
```

### **7.2 Sistema de Ofertas**

```rust
pub struct TransferOffer {
    pub player_id: u32,
    pub offering_club_id: u32,
    pub offered_fee: f32,
    pub wage_offered: f32,
    pub contract_length: u8,
}

impl TransferOffer {
    pub fn evaluate_offer(&self, player: &Player, club: &Club) -> OfferDecision {
        let player_value = calculate_player_value(player);
        
        // Criterios del club vendedor
        let fee_acceptable = self.offered_fee >= player_value * 0.9;
        
        let wage_acceptable = if self.wage_offered > club.wage_budget {
            false
        } else {
            self.wage_offered >= player.current_wage * 1.1  // 10% aumento mínimo
        };
        
        let player_happiness = player.happiness + 
                               (self.offered_fee / player_value * 20.0);
        
        // Decisión
        if fee_acceptable && wage_acceptable && player_happiness > 60 {
            OfferDecision::Accept
        } else if fee_acceptable && wage_acceptable {
            OfferDecision::Negotiate
        } else {
            OfferDecision::Reject
        }
    }
}
```

---

## **8. GENERACIÓN PROCEDURAL DE DATOS**

### **8.1 Generador de Jugadores**

```rust
pub struct PlayerGenerator;

impl PlayerGenerator {
    pub fn generate_player(nation: &Nation, age: u8) -> Player {
        // Nombres basados en nacionalidad
        let (first_name, last_name) = NameDatabase::get_name(nation.id);
        
        // Atributos basados en nivel de la nación
        let nation_strength = nation.futsal_level as f32 / 100.0;
        
        // Potencial basado en edad y aleatoriedad
        let potential = match age {
            15..=17 => rand::thread_rng().gen_range(80..180),
            18..=21 => rand::thread_rng().gen_range(100..190),
            22..=25 => rand::thread_rng().gen_range(120..195),
            _ => rand::thread_rng().gen_range(80..160),
        };
        
        // Current ability basado en edad y potencial
        let current = match age {
            15..=17 => potential as f32 * 0.4,
            18..=21 => potential as f32 * 0.6,
            22..=25 => potential as f32 * 0.85,
            26..=29 => potential as f32 * 0.95,
            _ => potential as f32 * 0.8,
        };
        
        // Generar atributos coherentes con la posición
        let position = Self::determine_position();
        let attributes = Self::generate_attributes_for_position(
            position, 
            current as u16,
            nation_strength
        );
        
        Player {
            first_name,
            last_name,
            date_of_birth: Self::calculate_dob(age),
            nation_id: nation.id,
            current_ability: current as u16,
            potential_ability: potential,
            attributes,
            position,
            // ... más campos
        }
    }
}
```

---

## **9. RENDIMIENTO Y OPTIMIZACIÓN**

### **9.1 Estrategias de Optimización**

1. **Base de Datos:**
   - SQLite en modo WAL (Write-Ahead Logging)
   - Índices en todas las claves foráneas
   - Consultas preparadas (prepared statements)
   - Conexión pool con SQLx

2. **Simulación:**
   - Partidos de IA en background sin renderizado
   - Multithreading para simulación de múltiples ligas
   - LOD (Level of Detail): Menos detalle en ligas inferiores

3. **Frontend:**
   - Virtual scrolling en tablas grandes
   - Lazy loading de imágenes/avatars
   - Memoización de componentes React
   - Web Workers para cálculos pesados

4. **Memoria:**
   - Carga bajo demanda de datos
   - Cache LRU para jugadores frecuentes
   - Compresión de datos históricos

---

## **10. ROADMAP DE DESARROLLO**

### **Fase 1: Core (Meses 1-3)**
- [ ] Arquitectura Tauri + Rust + React
- [ ] Esquema de base de datos completo
- [ ] Generador procedural de jugadores/clubes
- [ ] Sistema de tiempo básico
- [ ] Motor de partido 2D simple

### **Fase 2: Gestión (Meses 4-6)**
- [ ] Sistema de fichajes y contratos
- [ ] Entrenamientos y progresión
- [ ] Sistema de ojeo
- [ ] Finanzas de club
- [ ] Interfaz completa de gestión

### **Fase 3: Simulación (Meses 7-9)**
- [ ] Motor de partido avanzado con físicas
- [ ] IA táctica de equipos
- [ ] Sistema de moral y felicidad
- [ ] Lesiones y sanciones
- [ ] Cantera y juveniles

### **Fase 4: Pulido (Meses 10-12)**
- [ ] Competiciones internacionales
- [ ] Noticias y medios
- [ ] Logros y estadísticas históricas
- [ ] Optimización de rendimiento
- [ ] Beta testing y bugs

---

## **11. MÉTRICAS DE ÉXITO**

- **Rendimiento:** Simular 1000 partidos/segundo en modo máximo
- **Base de datos:** <100ms en consultas complejas
- **Memoria:** <500MB RAM en uso normal
- **Tamaño:** <200MB instalable
- **Jugabilidad:** 60 FPS estables en visualización de partido

---

## **12. CONSIDERACIONES FINALES**

Este PRD establece las bases para un **Futsal Manager** profesional, realista y técnicamente sólido. La elección de **Rust + Tauri** garantiza rendimiento, mientras que **React** proporciona una interfaz moderna y flexible.

**Puntos clave diferenciadores:**
1. **Autenticidad futsal**: Reglas reales (doble penalti, powerplay, rotaciones)
2. **Profundidad táctica**: Automatísmos entrenables, formaciones específicas
3. **Ojeo limitado**: Sistema realista de información parcial
4. **Cantera**: Desarrollo de jóvenes promesas
5. **Motor 2D**: Visualización clara y ágil de los partidos

