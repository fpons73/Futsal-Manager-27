# FUTSAL MANAGER 27

Simulador de gestión de fútbol sala para PC, inspirado en Football Manager pero 100% futsal. Motor de partido 2D cenital estilo "chapas", 3 ligas simultáneas y simulación ultrarrápida.

> **Stack:** Tauri v2 · Rust · SQLite (WAL) · React 18 + TypeScript · TailwindCSS · Konva.js · Zustand

---

## Estado actual — v1.0 (completo jugable)

| Sistema | Estado |
|---|---|
| Mundo: 43 competiciones PRD (1ª/2ª/2ªB, copas y selecciones) · 26 naciones · 370 clubes · +4400 jugadores · atributos ponderados por posición | ✅ |
| Calendario round-robin doble (662 partidos) | ✅ |
| Motor 2D: 40×20 m, 2×20', faltas 6ª→doble penalti, powerplay, cambios por fatiga | ✅ |
| Simulación multi-liga headless + avance día/semana | ✅ |
| API Tauri (18 comandos) + persistencia file DB | ✅ |
| Frontend: 10 pantallas (NewGame, Dashboard, Plantilla, Clasificación, Calendario, Partido en vivo, Mercado, Inbox, Entrenamientos, Finanzas) | ✅ |
| Mercado: valoración `CA^1.8×edad×potencial`, ofertas AI, negociación, inbox | ✅ |
| Entrenamientos: 8 tipos, schedule L-V, progreso semanal `edad×gap×prof`, lesiones 0.8% | ✅ |
| Finanzas: balance, presupuestos, taquilla 65-90%×€12, patrocinio, alerta negativo | ✅ |
| Fin de temporada: premios, retiradas 33-36a, cantera 17a, nuevo calendario | ✅ |

---

## Requisitos

- **Rust** 1.77+ (`rustup`)
- **Node** 20+ y **pnpm** 9+ (`npm i -g pnpm`)
- Windows 10/11 con WebView2 (incluido en Windows 11)

## Instalación y ejecución

```bash
# 1. Clonar
git clone https://github.com/fpons73/Futsal-Manager-27.git
cd Futsal-Manager-27

# 2. Dependencias frontend
pnpm install

# 3. Desarrollo (Vite + Tauri, hot-reload)
pnpm tauri dev
#  → abre la ventana nativa. El primer arranque compila Rust (3-5 min).

# 4. Build de producción
pnpm run build          # frontend
pnpm tauri build    # instalador .msi / .exe en src-tauri/target/release/bundle/
```

### Solo frontend (sin Tauri, en navegador)

```bash
pnpm run dev
# abre http://localhost:5173 — requiere backend Tauri para datos reales
```

### Tests backend

```bash
cd src-tauri
cargo test             # 12 tests: migraciones, mundo, calendario, motor, simulación
cargo test -- --nocapture --test-threads=1
```

---

## Cómo jugar (v1.0)

1. **Nueva partida** → elige uno de los 370 clubes (ES/BR/PT + resto del mundo, incl. divisiones inferiores), o abre el **Editor** para crear/editar/eliminar países, clubes, jugadores y competiciones.
2. **Dashboard** → fecha, próximo partido, clasificación, *Avanzar 1 día / +7 días*, alerta fin de temporada → *Rollover*.
3. **Plantilla** → 12 jugadores con CA/PA, salario, condición, atributos.
4. **Clasificación / Calendario** → tablas y jornadas (30/30/26).
5. **Partido** → *Ver en vivo*: motor Rust con faltas→doble penalti, powerplay, cambios; controles pausa/x1/x2/x5.
6. **Mercado** → 20 jugadores aleatorios, oferta €, negociación AI (≥85% acepta, 60-85% negocia), inbox con ofertas entrantes.
7. **Entrenamientos** → schedule L-V (técnica/táctica/físico), progreso semanal, lesiones.
8. **Finanzas** → balance, presupuestos, taquilla y patrocinio semanal.
9. **Buzón** → mensajes board/staff (fichajes, lesiones, financiera, fin de temporada).

La simulación headless resuelve jornadas de todas las ligas al avanzar: **+660 partidos/temporada**.

---

## Arquitectura

```
src-tauri/
  migrations/ 001_initial.sql (20+ tablas) + 002_training.sql (8 tipos)
  src/
    db.rs, world/, competition/, engine.rs (2400 ticks, IA, duelos)
    simulation/ (advance_day + taquilla + training/finance/inbox semanal)
    transfer/, training/, finance/, season/ (rollover)
    commands/ (game, match_live, transfer, training, finance, season, inbox)
src/
  api.ts (18 invokes) · store.ts (Zustand)
  components/
    screens/ NewGame, Dashboard, SquadView, StandingsView, FixturesView,
             LiveMatch, MarketView, InboxView, TrainingView, FinanceView
    FutsalPitch.tsx (Konva 820×420)
```

**Snapshot del motor** (`MatchSnapshot`) via `invoke("tick_live")` a ~30 fps; el backend avanza `ticks` según velocidad.

---

## Roadmap futuro

- Ojeo con niebla de guerra (scouting), cantera U18 completa, copas nacionales, competiciones internacionales, playoffs, editor de base de datos.

---

## Créditos

PRD V2 y Prototipo Técnico como especificación. Nombres y escudos ficticios inspirados en LNFS/LNF/Liga Placard.

## Licencia

MIT
