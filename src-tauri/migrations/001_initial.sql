PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- =========================================================
-- Mundo y geografia
-- =========================================================
CREATE TABLE IF NOT EXISTS confederations (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  short_name TEXT NOT NULL,
  reputation INTEGER DEFAULT 1000
);

CREATE TABLE IF NOT EXISTS nations (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  confederation_id INTEGER NOT NULL REFERENCES confederations(id),
  reputation INTEGER DEFAULT 500,
  futsal_level INTEGER DEFAULT 50
);

CREATE TABLE IF NOT EXISTS cities (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  population INTEGER
);

CREATE TABLE IF NOT EXISTS stadiums (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  city_id INTEGER REFERENCES cities(id),
  capacity INTEGER NOT NULL DEFAULT 2000,
  pitch_type TEXT DEFAULT 'parquet'
);

-- =========================================================
-- Competiciones
-- =========================================================
CREATE TABLE IF NOT EXISTS competitions (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  nation_id INTEGER REFERENCES nations(id),
  tier INTEGER,
  total_teams INTEGER,
  season TEXT NOT NULL DEFAULT '2026/2027',
  format TEXT DEFAULT 'Round Robin'
);

CREATE TABLE IF NOT EXISTS clubs (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  short_name TEXT,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  city_id INTEGER REFERENCES cities(id),
  stadium_id INTEGER REFERENCES stadiums(id),
  reputation INTEGER DEFAULT 100,
  primary_color TEXT DEFAULT '#0f4c3a',
  secondary_color TEXT DEFAULT '#ffffff'
);

CREATE TABLE IF NOT EXISTS club_finances (
  club_id INTEGER PRIMARY KEY REFERENCES clubs(id),
  balance REAL DEFAULT 500000,
  transfer_budget REAL DEFAULT 100000,
  wage_budget REAL DEFAULT 8000,
  total_wages REAL DEFAULT 0,
  sponsorship_income REAL DEFAULT 0,
  ticket_income REAL DEFAULT 0,
  prize_money REAL DEFAULT 0
);

-- =========================================================
-- Jugadores
-- =========================================================
CREATE TABLE IF NOT EXISTS players (
  id INTEGER PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  common_name TEXT,
  date_of_birth TEXT NOT NULL,
  nation_id INTEGER NOT NULL REFERENCES nations(id),
  second_nation_id INTEGER REFERENCES nations(id),
  preferred_foot TEXT DEFAULT 'right',
  height_cm INTEGER DEFAULT 175,
  weight_kg INTEGER DEFAULT 75,
  is_retired INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS player_positions (
  player_id INTEGER PRIMARY KEY REFERENCES players(id),
  por_natural INTEGER DEFAULT 0,
  cie_natural INTEGER DEFAULT 0,
  ala_natural INTEGER DEFAULT 0,
  piv_natural INTEGER DEFAULT 0,
  uni_natural INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS player_states (
  player_id INTEGER PRIMARY KEY REFERENCES players(id),
  current_ability INTEGER DEFAULT 80,
  potential_ability INTEGER DEFAULT 120,
  condition_val INTEGER DEFAULT 100,
  match_fitness INTEGER DEFAULT 100,
  morale INTEGER DEFAULT 70,
  sharpness INTEGER DEFAULT 50,
  happiness INTEGER DEFAULT 70
);

CREATE TABLE IF NOT EXISTS player_attributes (
  player_id INTEGER PRIMARY KEY REFERENCES players(id),
  first_touch INTEGER DEFAULT 10,
  dribbling INTEGER DEFAULT 10,
  ball_control INTEGER DEFAULT 10,
  technique INTEGER DEFAULT 10,
  passing INTEGER DEFAULT 10,
  vision INTEGER DEFAULT 10,
  crossing INTEGER DEFAULT 10,
  long_shots INTEGER DEFAULT 10,
  finishing INTEGER DEFAULT 10,
  heading INTEGER DEFAULT 10,
  penalty_taking INTEGER DEFAULT 10,
  tackling INTEGER DEFAULT 10,
  marking INTEGER DEFAULT 10,
  interception INTEGER DEFAULT 10,
  blocking INTEGER DEFAULT 10,
  anticipation INTEGER DEFAULT 10,
  decisions INTEGER DEFAULT 10,
  positioning INTEGER DEFAULT 10,
  off_the_ball INTEGER DEFAULT 10,
  work_rate INTEGER DEFAULT 10,
  composure INTEGER DEFAULT 10,
  concentration INTEGER DEFAULT 10,
  determination INTEGER DEFAULT 10,
  bravery INTEGER DEFAULT 10,
  aggression INTEGER DEFAULT 10,
  leadership INTEGER DEFAULT 10,
  teamwork INTEGER DEFAULT 10,
  flair INTEGER DEFAULT 10,
  acceleration INTEGER DEFAULT 10,
  pace INTEGER DEFAULT 10,
  agility INTEGER DEFAULT 10,
  balance INTEGER DEFAULT 10,
  stamina INTEGER DEFAULT 10,
  strength INTEGER DEFAULT 10,
  jumping INTEGER DEFAULT 10,
  reflexes INTEGER DEFAULT 10,
  handling INTEGER DEFAULT 10,
  one_on_ones INTEGER DEFAULT 10,
  positioning_gk INTEGER DEFAULT 10,
  rushing_out INTEGER DEFAULT 10,
  throwing INTEGER DEFAULT 10,
  kicking INTEGER DEFAULT 10,
  professionalism INTEGER DEFAULT 10,
  consistency INTEGER DEFAULT 10,
  important_matches INTEGER DEFAULT 10,
  injury_proneness INTEGER DEFAULT 10
);

CREATE TABLE IF NOT EXISTS contracts (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id),
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  wage_weekly REAL NOT NULL DEFAULT 500,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  release_clause REAL,
  is_active INTEGER DEFAULT 1
);

-- =========================================================
-- Tactica
-- =========================================================
CREATE TABLE IF NOT EXISTS tactics (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL UNIQUE REFERENCES clubs(id),
  formation TEXT NOT NULL DEFAULT '3-1',
  tempo INTEGER DEFAULT 50,
  pressing INTEGER DEFAULT 50,
  defensive_line INTEGER DEFAULT 50,
  width INTEGER DEFAULT 50,
  playing_style TEXT DEFAULT 'balanced',
  powerplay_enabled INTEGER DEFAULT 1
);

-- =========================================================
-- Partidos y competicion
-- =========================================================
CREATE TABLE IF NOT EXISTS matches (
  id INTEGER PRIMARY KEY,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  season TEXT NOT NULL,
  round INTEGER,
  date TEXT NOT NULL,
  home_club_id INTEGER NOT NULL REFERENCES clubs(id),
  away_club_id INTEGER NOT NULL REFERENCES clubs(id),
  stadium_id INTEGER REFERENCES stadiums(id),
  status TEXT DEFAULT 'scheduled',
  home_score INTEGER DEFAULT 0,
  away_score INTEGER DEFAULT 0,
  home_possession INTEGER,
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
  away_red_cards INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS match_events (
  id INTEGER PRIMARY KEY,
  match_id INTEGER NOT NULL REFERENCES matches(id),
  minute INTEGER NOT NULL,
  second INTEGER DEFAULT 0,
  event_type TEXT NOT NULL,
  player_id INTEGER REFERENCES players(id),
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  description TEXT,
  x REAL,
  y REAL
);

CREATE TABLE IF NOT EXISTS match_player_stats (
  id INTEGER PRIMARY KEY,
  match_id INTEGER NOT NULL REFERENCES matches(id),
  player_id INTEGER NOT NULL REFERENCES players(id),
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  started INTEGER DEFAULT 0,
  minutes_played INTEGER DEFAULT 0,
  goals INTEGER DEFAULT 0,
  assists INTEGER DEFAULT 0,
  shots INTEGER DEFAULT 0,
  shots_on_target INTEGER DEFAULT 0,
  fouls_committed INTEGER DEFAULT 0,
  yellow_cards INTEGER DEFAULT 0,
  red_cards INTEGER DEFAULT 0,
  rating REAL DEFAULT 6.0
);

CREATE TABLE IF NOT EXISTS league_standings (
  id INTEGER PRIMARY KEY,
  competition_id INTEGER NOT NULL REFERENCES competitions(id),
  season TEXT NOT NULL,
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  position INTEGER,
  played INTEGER DEFAULT 0,
  won INTEGER DEFAULT 0,
  drawn INTEGER DEFAULT 0,
  lost INTEGER DEFAULT 0,
  goals_for INTEGER DEFAULT 0,
  goals_against INTEGER DEFAULT 0,
  goal_difference INTEGER DEFAULT 0,
  points INTEGER DEFAULT 0,
  form_last_5 TEXT DEFAULT '',
  UNIQUE(competition_id, season, club_id)
);

-- =========================================================
-- Transferencias
-- =========================================================
CREATE TABLE IF NOT EXISTS transfer_offers (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id),
  from_club_id INTEGER NOT NULL REFERENCES clubs(id),
  to_club_id INTEGER NOT NULL REFERENCES clubs(id),
  offered_fee REAL NOT NULL,
  wage_offered REAL,
  status TEXT DEFAULT 'pending',
  date_offered TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS transfer_history (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id),
  from_club_id INTEGER REFERENCES clubs(id),
  to_club_id INTEGER NOT NULL REFERENCES clubs(id),
  transfer_date TEXT NOT NULL,
  transfer_fee REAL DEFAULT 0,
  transfer_type TEXT DEFAULT 'permanent'
);

-- =========================================================
-- Lesiones, sanciones, mensajes, entrenamientos
-- =========================================================
CREATE TABLE IF NOT EXISTS injuries (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id),
  injury_type TEXT NOT NULL,
  severity INTEGER NOT NULL,
  expected_return_date TEXT,
  injury_date TEXT NOT NULL,
  is_active INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS suspensions (
  id INTEGER PRIMARY KEY,
  player_id INTEGER NOT NULL REFERENCES players(id),
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  reason TEXT NOT NULL,
  matches_banned INTEGER NOT NULL,
  matches_served INTEGER DEFAULT 0,
  start_date TEXT NOT NULL,
  is_active INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS inbox_messages (
  id INTEGER PRIMARY KEY,
  club_id INTEGER NOT NULL REFERENCES clubs(id),
  sender_type TEXT DEFAULT 'system',
  subject TEXT NOT NULL,
  body TEXT NOT NULL,
  date_sent TEXT NOT NULL,
  is_read INTEGER DEFAULT 0,
  is_important INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS game_state (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  game_date TEXT NOT NULL,
  season TEXT NOT NULL,
  user_club_id INTEGER REFERENCES clubs(id),
  game_speed TEXT DEFAULT 'normal'
);

-- =========================================================
-- Indices
-- =========================================================
CREATE INDEX IF NOT EXISTS idx_players_nation ON players(nation_id);
CREATE INDEX IF NOT EXISTS idx_contracts_club ON contracts(club_id);
CREATE INDEX IF NOT EXISTS idx_contracts_player ON contracts(player_id);
CREATE INDEX IF NOT EXISTS idx_matches_competition ON matches(competition_id);
CREATE INDEX IF NOT EXISTS idx_matches_date ON matches(date);
CREATE INDEX IF NOT EXISTS idx_matches_clubs ON matches(home_club_id, away_club_id);
CREATE INDEX IF NOT EXISTS idx_match_events_match ON match_events(match_id);
CREATE INDEX IF NOT EXISTS idx_standings_comp_season ON league_standings(competition_id, season);
CREATE INDEX IF NOT EXISTS idx_transfer_offers_player ON transfer_offers(player_id);
CREATE INDEX IF NOT EXISTS idx_inbox_club ON inbox_messages(club_id);
