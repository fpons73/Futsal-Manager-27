CREATE TABLE IF NOT EXISTS training_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    intensity INTEGER DEFAULT 50,
    attributes TEXT
);

INSERT OR IGNORE INTO training_types (id, name, category, intensity, attributes) VALUES
(1, 'Técnica Individual', 'technical', 70, '["dribbling","ball_control","technique","first_touch"]'),
(2, 'Táctica 3-1', 'tactical', 75, '["positioning","decisions","teamwork"]'),
(3, 'Táctica 4-0', 'tactical', 75, '["positioning","vision","teamwork"]'),
(4, 'Físico - Resistencia', 'physical', 65, '["stamina","strength","acceleration"]'),
(5, 'Físico - Velocidad', 'physical', 70, '["pace","agility","acceleration"]'),
(6, 'Porteros', 'goalkeeper', 60, '["reflexes","handling","positioning_gk"]'),
(7, 'Finalización', 'technical', 65, '["finishing","long_shots","penalty_taking"]'),
(8, 'Defensa', 'tactical', 70, '["tackling","marking","interception"]');

CREATE TABLE IF NOT EXISTS training_schedule (
    id INTEGER PRIMARY KEY,
    club_id INTEGER NOT NULL REFERENCES clubs(id),
    day_of_week INTEGER NOT NULL,
    training_type_id INTEGER NOT NULL REFERENCES training_types(id),
    intensity INTEGER DEFAULT 50,
    UNIQUE(club_id, day_of_week)
);

CREATE TABLE IF NOT EXISTS training_history (
    id INTEGER PRIMARY KEY,
    club_id INTEGER NOT NULL REFERENCES clubs(id),
    date TEXT NOT NULL,
    training_type_id INTEGER NOT NULL REFERENCES training_types(id),
    effectiveness INTEGER DEFAULT 50
);

CREATE TABLE IF NOT EXISTS youth_academy (
    id INTEGER PRIMARY KEY,
    club_id INTEGER NOT NULL UNIQUE REFERENCES clubs(id),
    level INTEGER DEFAULT 50,
    scouting_budget REAL DEFAULT 20000
);
