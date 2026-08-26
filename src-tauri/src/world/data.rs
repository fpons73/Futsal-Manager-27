pub struct ClubDef {
    pub name: &'static str,
    pub short: &'static str,
    pub city: &'static str,
    pub reputation: i64,
    pub color: &'static str,
    pub color2: &'static str,
    pub stadium: &'static str,
    pub capacity: i64,
}

pub const SPAIN_CLUBS: &[ClubDef] = &[
    ClubDef { name: "FC Barcelona Futsal", short: "BAR", city: "Barcelona", reputation: 920, color: "#A50044", color2: "#004D98", stadium: "Palau Blaugrana", capacity: 7588 },
    ClubDef { name: "Inter FS", short: "INT", city: "Torrejon de Ardoz", reputation: 900, color: "#00A651", color2: "#000000", stadium: "Jorge Garbajosa", capacity: 3450 },
    ClubDef { name: "ElPozo Murcia", short: "EPO", city: "Murcia", reputation: 880, color: "#E30613", color2: "#FFFFFF", stadium: "Palacio de Deportes", capacity: 7450 },
    ClubDef { name: "Palma Futsal", short: "PAL", city: "Palma de Mallorca", reputation: 870, color: "#00A859", color2: "#000000", stadium: "Palau Municipal", capacity: 5120 },
    ClubDef { name: "Jaen Paraiso Interior", short: "JAE", city: "Jaen", reputation: 820, color: "#FFD500", color2: "#6B1D5E", stadium: "Olivo Arena", capacity: 6589 },
    ClubDef { name: "Valdepenas FS", short: "VAL", city: "Valdepenas", reputation: 780, color: "#0057A8", color2: "#FFFFFF", stadium: "Virgen de la Cabeza", capacity: 2000 },
    ClubDef { name: "Osasuna Magna Xota", short: "OSA", city: "Pamplona", reputation: 760, color: "#0A4A8A", color2: "#C8102E", stadium: "Anaitasuna", capacity: 3000 },
    ClubDef { name: "Levante UD FS", short: "LEV", city: "Valencia", reputation: 750, color: "#0D2A54", color2: "#C8102E", stadium: "Paterna", capacity: 1600 },
    ClubDef { name: "Cordoba Patrimonio", short: "COR", city: "Cordoba", reputation: 700, color: "#00693E", color2: "#FFFFFF", stadium: "Vista Alegre", capacity: 3800 },
    ClubDef { name: "Manacor FS", short: "MAN", city: "Manacor", reputation: 690, color: "#009639", color2: "#FFFFFF", stadium: "Miquel Angel Nadal", capacity: 1500 },
    ClubDef { name: "Quesos El Hidalgo Manzanares", short: "MANZ", city: "Manzanares", reputation: 670, color: "#003DA5", color2: "#FFFFFF", stadium: "Manzanares Arena", capacity: 2000 },
    ClubDef { name: "Noia Portus Apostoli", short: "NOI", city: "Noia", reputation: 650, color: "#00A651", color2: "#FFFFFF", stadium: "Agustin Mouris", capacity: 2000 },
    ClubDef { name: "Ribera Navarra FS", short: "RIB", city: "Tudela", reputation: 640, color: "#FF6600", color2: "#004B8D", stadium: "Ciudad de Tudela", capacity: 1200 },
    ClubDef { name: "AD Sala 10 Zaragoza", short: "ZAR", city: "Zaragoza", reputation: 630, color: "#E30613", color2: "#000000", stadium: "Siglo XXI", capacity: 2850 },
    ClubDef { name: "UMA Antequera", short: "ANT", city: "Antequera", reputation: 620, color: "#006633", color2: "#FFFFFF", stadium: "Fernando Argüelles", capacity: 2575 },
    ClubDef { name: "Peníscola FS", short: "PEN", city: "Peniscola", reputation: 600, color: "#FFCC00", color2: "#000000", stadium: "Pabellon Municipal", capacity: 1500 },
];

pub const BRAZIL_CLUBS: &[ClubDef] = &[
    ClubDef { name: "Magnus Futsal", short: "MAG", city: "Sorocaba", reputation: 920, color: "#FF6600", color2: "#000000", stadium: "Arena Sorocaba", capacity: 5000 },
    ClubDef { name: "Atlantico Futsal", short: "ATL", city: "Erechim", reputation: 860, color: "#006633", color2: "#FFFFFF", stadium: "Caldeirao", capacity: 3500 },
    ClubDef { name: "Joinville Futsal", short: "JOI", city: "Joinville", reputation: 850, color: "#000000", color2: "#FFFFFF", stadium: "Centreventos Cau Hansen", capacity: 3500 },
    ClubDef { name: "Carlos Barbosa (ACBF)", short: "ACBF", city: "Carlos Barbosa", reputation: 900, color: "#FF6600", color2: "#FFFFFF", stadium: "Centro Municipal", capacity: 6500 },
    ClubDef { name: "Corinthians Futsal", short: "SCCP", city: "Sao Paulo", reputation: 870, color: "#000000", color2: "#FFFFFF", stadium: "Gin. Wlamir Marques", capacity: 7000 },
    ClubDef { name: "Cascavel Futsal", short: "CAS", city: "Cascavel", reputation: 820, color: "#0033A0", color2: "#FFCC00", stadium: "Neva", capacity: 3000 },
    ClubDef { name: "Jaragua Futsal", short: "JAR", city: "Jaragua do Sul", reputation: 840, color: "#FFCC00", color2: "#000000", stadium: "Arena Jaragua", capacity: 8600 },
    ClubDef { name: "Umuarama Futsal", short: "UMU", city: "Umuarama", reputation: 680, color: "#006633", color2: "#FFFFFF", stadium: "Amario Vieira", capacity: 4000 },
    ClubDef { name: "Pato Futsal", short: "PAT", city: "Pato Branco", reputation: 760, color: "#004B8D", color2: "#FFFFFF", stadium: "Dolivar Lavarda", capacity: 2000 },
    ClubDef { name: "Marechal Futsal", short: "MAR", city: "Marechal Candido Rondon", reputation: 670, color: "#006633", color2: "#FFFFFF", stadium: "Ney Braga", capacity: 2800 },
    ClubDef { name: "Minas Tenis Clube", short: "MIN", city: "Belo Horizonte", reputation: 750, color: "#004B8D", color2: "#FFFFFF", stadium: "Arena Minas", capacity: 4000 },
    ClubDef { name: "Campo Mourao Futsal", short: "CMO", city: "Campo Mourao", reputation: 640, color: "#000000", color2: "#FFCC00", stadium: "UTFPR", capacity: 2000 },
    ClubDef { name: "Foz Cataratas", short: "FOZ", city: "Foz do Iguacu", reputation: 700, color: "#004B8D", color2: "#FFFFFF", stadium: "Costa Cavalcanti", capacity: 3000 },
    ClubDef { name: "Sao Jose Futsal", short: "SJO", city: "Sao Jose dos Campos", reputation: 660, color: "#FFCC00", color2: "#000000", stadium: "Tenis Clube", capacity: 2500 },
    ClubDef { name: "Blumenau Futsal", short: "BLU", city: "Blumenau", reputation: 620, color: "#E30613", color2: "#FFFFFF", stadium: "SESI", capacity: 2000 },
    ClubDef { name: "Tubarao Futsal", short: "TUB", city: "Tubarao", reputation: 600, color: "#004B8D", color2: "#FFFFFF", stadium: "Estener Soratto", capacity: 3000 },
];

pub const PORTUGAL_CLUBS: &[ClubDef] = &[
    ClubDef { name: "Sporting CP Futsal", short: "SCP", city: "Lisboa", reputation: 940, color: "#008057", color2: "#FFFFFF", stadium: "Pavilhao Joao Rocha", capacity: 3000 },
    ClubDef { name: "SL Benfica Futsal", short: "BEN", city: "Lisboa", reputation: 930, color: "#EF0000", color2: "#FFFFFF", stadium: "Pavilhao Fidelidade", capacity: 2400 },
    ClubDef { name: "SC Braga Futsal", short: "BRA", city: "Braga", reputation: 800, color: "#E30613", color2: "#FFFFFF", stadium: "Amelia Morais", capacity: 2000 },
    ClubDef { name: "AD Fundao", short: "FUN", city: "Fundao", reputation: 680, color: "#6B1D5E", color2: "#FFFFFF", stadium: "Pavilhao Municipal", capacity: 1200 },
    ClubDef { name: "Leoes Porto Salvo", short: "LEO", city: "Oeiras", reputation: 660, color: "#FF0000", color2: "#FFFFFF", stadium: "Leoes Porto Salvo", capacity: 800 },
    ClubDef { name: "Ferreira do Zezere", short: "FER", city: "Ferreira do Zezere", reputation: 600, color: "#FFCC00", color2: "#000000", stadium: "Alfredo Bento Calado", capacity: 800 },
    ClubDef { name: "Quinta dos Lombos", short: "LOM", city: "Carcavelos", reputation: 640, color: "#004B8D", color2: "#FFFFFF", stadium: "Quinta dos Lombos", capacity: 800 },
    ClubDef { name: "ELC Belenenses", short: "BEL", city: "Lisboa", reputation: 620, color: "#004B8D", color2: "#FFFFFF", stadium: "Acacio Rosa", capacity: 1800 },
    ClubDef { name: "SC Ferreira", short: "SCF", city: "Ferreira do Zezere", reputation: 580, color: "#008057", color2: "#FFFFFF", stadium: "Municipal Ferreira", capacity: 800 },
    ClubDef { name: "Viseu 2001", short: "VIS", city: "Viseu", reputation: 560, color: "#000000", color2: "#FFCC00", stadium: "Cidade de Viseu", capacity: 1200 },
    ClubDef { name: "Candoso SC", short: "CAN", city: "Guimaraes", reputation: 540, color: "#FF0000", color2: "#000000", stadium: "Candoso", capacity: 600 },
    ClubDef { name: "SC Rio Ave", short: "RIO", city: "Vila do Conde", reputation: 550, color: "#008057", color2: "#FFFFFF", stadium: "Rio Ave", capacity: 1000 },
    ClubDef { name: "Boavista FC Futsal", short: "BOA", city: "Porto", reputation: 530, color: "#000000", color2: "#FFFFFF", stadium: "Boavista", capacity: 1000 },
    ClubDef { name: "CR Candoso", short: "CRC", city: "Vila Nova de Famalicao", reputation: 520, color: "#FF0000", color2: "#FFFFFF", stadium: "Candoso Famalicao", capacity: 600 },
];

pub const SPAIN_FIRST: &[&str] = &["Sergio","Adolfo","Carlos","Javier","Miguel","Antonio","Jose","Juan","Raul","Ferran","Marc","Dani","Jesus","Alvaro","Pol","Ruben","Mario","Pablo","Diego","Andres","Victor","Jorge","Borja","Chemi","Iker","Hugo","Leo","Nico","Ivan","Gonzalo"];
pub const SPAIN_LAST: &[&str] = &["Lozano","Fernandez","Garcia","Lopez","Martinez","Gonzalez","Rodriguez","Perez","Sanchez","Valera","Ortiz","Ruiz","Moreno","Jimenez","Gomez","Hernandez","Diaz","Alvarez","Molina","Navarro","Torres","Ramirez","Serrano","Gil","Vidal","Ramos","Ortas","Rivillos","Esquerdinha","Bebe"];

pub const BRAZIL_FIRST: &[&str] = &["Ferrao","Pito","Gadeia","Dyego","Filipe","Arthur","Marlon","Leandro","Rafael","Bruno","Tiago","Cecilio","Marcel","Rodrigo","Lucas","Gabriel","Matheus","Diego","Caio","Felipe","Gustavo","Willian","Douglas","Vinicius","Henrique","Wesley","Igor","Alan","Xuxa","Elison"];
pub const BRAZIL_LAST: &[&str] = &["Silva","Santos","Oliveira","Souza","Rodrigues","Ferreira","Alves","Pereira","Lima","Gomes","Costa","Ribeiro","Martins","Carvalho","Almeida","Soares","Fernandes","Rocha","Barbosa","Mendes","Freitas","Araujo","Cavalcanti","Xavier","Reis","Goncalves","Moura","Cardoso","Nascimento","Moreira"];

pub const PORTUGAL_FIRST: &[&str] = &["Ricardinho","Joao","Bruno","Andre","Tiago","Miguel","Rui","Pedro","Nuno","Paulo","Fabio","Hugo","Diogo","Francisco","Antonio","Carlos","Jose","Fernando","Vitor","Nelson","Edgar","Taynan","Pauleta","Erick","Zicky","Tomas","Rodrigo","Alex","Luis","Goncalo"];
pub const PORTUGAL_LAST: &[&str] = &["Silva","Santos","Ferreira","Pereira","Oliveira","Costa","Rodrigues","Martins","Jesus","Sousa","Fernandes","Goncalves","Gomes","Lopes","Soares","Almeida","Ribeiro","Carvalho","Alves","Pinto","Matos","Coelho","Moreira","Cardoso","Correia","Mendes","Reis","Cavaco","Teixeira","Varela"];

pub fn pick_first(nation: &str, rng: &mut impl rand::Rng) -> &'static str {
    let pool = match nation {
        "Portugal" => PORTUGAL_FIRST,
        "Brasil" => BRAZIL_FIRST,
        _ => SPAIN_FIRST,
    };
    pool[rng.gen_range(0..pool.len())]
}
pub fn pick_last(nation: &str, rng: &mut impl rand::Rng) -> &'static str {
    let pool = match nation {
        "Portugal" => PORTUGAL_LAST,
        "Brasil" => BRAZIL_LAST,
        _ => SPAIN_LAST,
    };
    pool[rng.gen_range(0..pool.len())]
}
