# FUTSAL MANAGER 27 — Plan de Desarrollo

Simulador de gestión de fútbol sala para PC inspirado en Football Manager, con motor de
partido 2D cenital estilo "chapas", basado en el PRD V2 y el Prototipo Técnico.

---

## 1. Stack Tecnológico

| Capa | Tecnología |
|---|---|
| Backend | Rust + Tauri v2 |
| Base de datos | SQLite (WAL) vía SQLx, migraciones embebidas |
| Frontend | React 18 + TypeScript + Vite |
| Estilos | TailwindCSS 3.4 (tema oscuro tipo FM) |
| Estado cliente | Zustand |
| Campo 2D | Konva.js / react-konva |

## 2. Alcance del primer hito jugable (v0.1)

- **3 ligas nacionales simultáneas**: España (Primera División, 16), Brasil (LNF, 16),
  Portugal (Liga Placard, 14). ~46 clubes reales ficticios.
- **~650 jugadores generados proceduralmente** con atributos técnicos/mentales/físicos/
  portero coherentes por posición y nivel de liga, contratos y valoración.
- **Calendario round-robin** ida/vuelta entrelazado por liga.
- **Motor de partido 2D** en Rust con reglas futsal reales:
  - 2×20 min, descanso, tiempo muerto por equipo y parte.
  - Faltas acumulativas: desde la 6ª falta → doble penalti (10 m).
  - Saques de banda con límite de 4 s, posesión del portero 4 s.
  - Cambios volantes ilimitados con gestión de fatiga.
  - Powerplay (portero-jugador) en finales de parte si el equipo pierde.
  - Formaciones: 3-1, 4-0, 2-2, 5-0. Sliders: tempo, presión, línea defensiva, amplitud.
  - Tarjetas amarillas/rojas y sanciones por acumulación.
- **Partido en vivo** visualizable (campo Konva 40×20 m) o simulación instantánea.
- **Gestión**: plantilla, pizarra táctica, mercado de fichajes (ofertas entrantes y
  salientes con IA), entrenamientos semanales, progresión de jugadores, moral,
  lesiones, finanzas (salarios, taquilla, premios, presupuestos).
- **Fin de temporada**: campeón, goleadores, rollover a nueva temporada
  (envejecimiento, retiradas, regeneración de juveniles, mercado de verano).
- **Guardado/carga** en archivo `.db` por partida + autoguardado semanal.

## 3. Arquitectura

```
FutsalManager_Open/
├── src-tauri/
│   ├── migrations/            # SQLx migrations (schema)
│   └── src/
│       ├── main.rs            # Entry point Tauri
│       ├── db.rs              # Pool SQLite + migraciones
│       ├── commands/          # API IPC (game, squad, tactics, league,
│       │                      # match, transfer, training, finance, inbox)
│       ├── world/             # Generación procedural del mundo
│       ├── competition/       # Ligas, calendario, clasificación, temporada
│       ├── engine/            # Motor de partido 2D
│       │   ├── types.rs       #   entidades y componentes ECS-lite
│       │   ├── engine.rs      #   bucle tick + estado
│       │   ├── ai.rs          #   posicionamiento y decisiones
│       │   ├── resolution.rs  #   duelos, probabilidad de gol, fatiga
│       │   ├── rules.rs       #   constantes reglamento futsal
│       │   └── quicksim.rs    #   simulación headless para partidos IA
│       └── management/        # fichajes IA, entrenamientos, progresión,
│                              # moral, lesiones/sanciones, finanzas
└── src/
    ├── components/screens/    # pantallas de gestión
    ├── components/match/      # campo Konva y HUD de partido
    ├── store.ts               # Zustand
    └── api.ts                 # wrappers tipados de invoke()
```

### Decisiones técnicas clave

1. **Snapshots del motor**: el frontend llama `advance_match(ticks)` a ~30 Hz según la
   velocidad elegida; el motor corre íntegro en Rust (evita 60 IPC/s).
2. **Quicksim headless**: los partidos IA-vs-IA usan el mismo motor sin render a máxima
   velocidad para simular jornadas completas de las 3 ligas en segundos.
3. **Schema pragmático**: ~18 tablas funcionales (evolucionables al esquema completo del
   PRD). Atributos del jugador agrupados en una sola tabla.
4. **Un proceso = un mundo**: el estado vivo vive en memoria durante la sesión y se
   persiste a SQLite al avanzar tiempo / acciones clave (robusto y rápido).

## 4. Hitos

| # | Hito | Entregable |
|---|---|---|
| M1 | Scaffold | Proyecto Tauri v2 + React + TS compilando (`tauri dev`) |
| M2 | Base de datos | Migraciones SQLx + pool + acceso tipado |
| M3 | Mundo procedural | 3 ligas, clubes, jugadores, staff, finanzas iniciales |
| M4 | Competiciones | Calendarios round-robin, standings, goleadores |
| M5 | Motor de partido | Tick loop, IA posicional, duelos, reglas futsal, eventos |
| M6 | Tiempo | Avance día/semana, simulación multi-liga headless |
| M7 | API Tauri | Commands completos y tipados |
| M8 | Frontend gestión | Dashboard, plantilla, tácticas, clasificaciones |
| M9 | Partido en vivo | Campo Konva + controles + cambios + tiempos muertos |
| M10 | Mercado | Ofertas bidireccionales, valoración, inbox |
| M11 | Entrenamientos | Programación semanal, progresión, lesiones, sanciones |
| M12 | Finanzas | Salarios, taquilla, premios, presupuesto junta |
| M13 | Temporada | Rollover completo a nueva temporada |
| M14 | Pulido | Tests, clippy/tsc limpios, README, balance |

## 5. Fuera de alcance v0.1 (fases futuras)

Ojeo con niebla de guerra, cantera U-x completa, copas nacionales, competiciones
internacionales, selecciones, playoffs, noticias de medios, logros históricos.

## 6. Verificación continua

- Tests Rust: validez de calendarios, sanidad estadística del motor (100 sims),
  progresión de jugadores, evaluación de ofertas.
- `cargo clippy` sin warnings, `tsc --noEmit` limpio, build de producción.
- Ejecución manual final: nueva partida → gestionar → ver partido en vivo → fin de
  temporada → rollover.

## 7. Flujo de trabajo

Cada hito se compromete a git y se sube a GitHub
(`https://github.com/fpons73/Futsal-Manager-27.git`) actualizando antes `ToDo.md`.
