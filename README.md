# FUTSAL MANAGER 27

Simulador de gestión de fútbol sala para PC, inspirado en Football Manager pero 100% futsal. Motor de partido 2D cenital estilo "chapas", 3 ligas simultáneas y simulación ultrarrápida.

> **Stack:** Tauri v2 · Rust · SQLite (WAL) · React 18 + TypeScript · TailwindCSS · Konva.js · Zustand

---

## Estado actual — v0.1 (vertical slice jugable)

| Sistema | Estado |
|---|---|
| Mundo: 3 ligas (España 16, Brasil 16, Portugal 14) · 46 clubes · 552 jugadores · atributos ponderados por posición | ✅ |
| Calendario round-robin doble (662 partidos) | ✅ |
| Motor 2D: 40×20 m, 2×20', faltas acumulativas → doble penalti 10 m, 4 s banda/portero, cambios volantes por fatiga, powerplay, 4 formaciones | ✅ |
| Simulación multi-liga headless (jornadas completas en <1 s) | ✅ |
| Avance de tiempo día/semana, standings con DG y posiciones, eventos | ✅ |
| API Tauri (9 comandos) + persistencia file DB | ✅ |
| Frontend: NewGame, Dashboard, Plantilla, Clasificación, Calendario, **Partido en vivo** con Konva a 30 fps y controles x1/x2/x5 | ✅ |
| Mercado de fichajes, entrenamientos, finanzas, fin de temporada | 🔜 (base DB lista, UI en roadmap) |

---

## Requisitos

- **Rust** 1.77+ (`rustup`)
- **Node** 20+ y **npm**
- Windows 10/11 con WebView2 (incluido en Windows 11)

## Instalación y ejecución

```bash
# 1. Clonar
git clone https://github.com/fpons73/Futsal-Manager-27.git
cd Futsal-Manager-27

# 2. Dependencias frontend
npm install

# 3. Desarrollo (Vite + Tauri, hot-reload)
npm run tauri dev
#  → abre la ventana nativa. El primer arranque compila Rust (3-5 min).

# 4. Build de producción
npm run build          # frontend
npm run tauri build    # instalador .msi / .exe en src-tauri/target/release/bundle/
```

### Solo frontend (sin Tauri, en navegador)

```bash
npm run dev
# abre http://localhost:5173 — requiere backend Tauri para datos reales
```

### Tests backend

```bash
cd src-tauri
cargo test             # 12 tests: migraciones, mundo, calendario, motor, simulación
cargo test -- --nocapture --test-threads=1
```

---

## Cómo jugar (v0.1)

1. **Nueva partida** → elige uno de los 46 clubes (agrupados por España/Brasil/Portugal).
2. **Dashboard** → fecha, próximo partido, mini-clasificación, botón *Avanzar 1 día / +7 días*.
3. **Plantilla** → 12 jugadores con CA/PA, salario, condición, atributos (PAS/FIN/REG/ENT/RIT).
4. **Clasificación / Calendario** → tablas completas y jornadas por ronda (30/30/26).
5. **Partido** → *Iniciar próximo partido*: motor en vivo con marcador, faltas, tiros, posesión, eventos y fatiga. Controles pausa/x1/x2/x5. El motor decide pases/tiros con `finishing × composure`, faltas → doble penalti a partir de la 6ª, y powerplay si vas perdiendo en los últimos 3'.

La simulación **headless** resuelve las otras 2 ligas al avanzar días: 662 partidos por temporada.

---

## Arquitectura

```
src-tauri/
  migrations/001_initial.sql   # 20+ tablas, WAL, FK, índices
  src/
    db.rs                      # pools file/memory + migrate!
    world/                     # seed procedural (nombres ES/BR/PT, CA/PA por reputación)
    competition/               # round-robin círculo, doble vuelta
    engine.rs                  # MatchEngine 40×20, 2400 ticks, IA posicional, duelos, stamina
    simulation/                # advance_day/week → engine + standings
    commands/                  # Tauri IPC (game.rs, match_live.rs)
src/
  api.ts · store.ts (Zustand)
  components/
    screens/ NewGame, Dashboard, SquadView, StandingsView, FixturesView, LiveMatch
    FutsalPitch.tsx (Konva 820×420, SCALE 20)
```

**Snapshot del motor** (`MatchSnapshot`) via `invoke("tick_live")` a ~30 fps; el backend avanza `ticks` según velocidad.

---

## Roadmap

- **M10** Mercado: ofertas entrantes/salientes, valoración `CA² × edad × potencial × contrato`, inbox.
- **M11** Entrenamientos: programación semanal, progresión `edad × profesionalidad × instalaciones`, lesiones/sanciones.
- **M12** Finanzas: balance, presupuesto fichajes/salarios, taquilla, premios.
- **M13** Fin de temporada: campeón, pichichi, rollover (envejecimiento, retiradas, regeneración U18).
- **M14** Pulido: ojeo con niebla de guerra, cantera, copas, tests balance, README vídeo.

---

## Créditos

PRD V2 y Prototipo Técnico como especificación. Nombres y escudos ficticios inspirados en LNFS/LNF/Liga Placard.

## Licencia

MIT
