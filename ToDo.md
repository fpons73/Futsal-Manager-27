# FUTSAL MANAGER 27 — Estado del Proyecto

> Este archivo se actualiza con cada hito completado. Cada hito se sube a GitHub.
> Repo: https://github.com/fpons73/Futsal-Manager-27.git

---

## Hitos

- [x] Documentos base (PRD V2 + Prototipo Técnico)
- [x] plan.md creado
- [x] M1: Scaffold Tauri v2 + React + TS compilando — `npm run build` + `cargo check` limpios
- [x] M2: Base de datos SQLite (migraciones + pool) — 001_initial.sql con 20+ tablas, WAL+FK, test `migration_creates_tables` OK
- [x] M3: Generación procedural del mundo (3 ligas) — 46 clubes, 552 jugadores, 3 competiciones, test OK
- [x] M4: Competiciones y calendario round-robin — doble robin 662 partidos (240+240+182), balance verificado, 30/30/26 jornadas, tests OK
- [x] M5: Motor de partido 2D con reglas futsal — ECS-lite, fatiga, faltas/6ª doble-penalti, powerplay, cambios volantes, tests DB OK
- [x] M6: Procesador de tiempo y simulación multi-liga — advance_day con engine, standings y posiciones, full season 662 partidos, bugfix `current_date`→`game_date` y FK, 12 tests OK
- [x] M7: API de comandos Tauri completa — new_game, game_state, advance_day/week, standings, fixtures, squad, competitions, next_fixture
- [x] M8: Frontend gestión — NewGame, Dashboard, Plantilla, Clasificación, Calendario, shell navegación, `api.ts`/`store.ts`
- [x] M9: Partido en vivo — MatchEngine en vivo vía `live_match` en AppState, `start_live_match`/`tick_live`/`get_live_snapshot`, FutsalPitch reactivo, controles pausa/x1/x2/x5
- [x] M10: Mercado de fichajes + bandeja de entrada — valoración CA², ofertas AI con negociación, inbox con mensajes board/staff, generación automática de ofertas
- [x] M11: Entrenamientos, progresión, lesiones y sanciones — tipos, schedule semanal, progreso por edad/potencial/profesionalidad, lesiones aleatorias 0.8%
- [x] M12: Finanzas del club — balance, presupuestos, taquilla (65-90% aforo × €12), patrocinio semanal, alerta balance negativo
- [x] M13: Fin de temporada y rollover — premios top3, retiradas 33-36 años, regeneración juvenil 17 años, nuevos contratos, calendario siguiente temporada
- [x] M14: Pulido — `cargo check`/`tsc`/`vite build` limpios, 12 tests OK, README, plan/ToDo al día
- [x] M15: Mundo PRD completo — 37 competiciones, 26 naciones, 6 confederaciones, 222 clubes, +2600 jugadores (names ES/BR/PT + genéricos por país), tests actualizados
- [x] M16: Editor BD — CRUD países/clubes/jugadores/competiciones (backend `editor/mod.rs` + `editor_cmd.rs`, frontend `EditorView`), auto-init `editor_init` (auto-seed), navegación "Editor"
- [x] M17: Editor estilo FM — escudo por club (subida local base64→fichero + asset protocol), entrenador (staff role 'coach'), cuerpo técnico (staff: coach/assistant/scout/physio), gestión de plantilla (fichar/liberar jugadores por club)
- [x] M18: Buscadores en todo el editor + editores completos — jugadores (foto + 46 atributos agrupados), staff (foto + atributos), países (bandera + escudo de federación), confederaciones (escudo)
- [x] M19: 2ª divisiones y más bajas + discriminador Clubes/Selecciones — migración 005 (competitions.kind), Segunda División ES/BR/PT + Vysshaya/A2/FR-D2/AR-2ª, etiquetado kind, toggle Clubes/Selecciones en inicio/clasificación/calendario/editor
- [x] M20: Clubes reales por división — el seed genera clubes distintos por nación (suma de equipos por división) y asigna cada liga su propio grupo: España 92 clubes (Primera 16 + Segunda 16 + 6×Segunda B 60), sin solapamiento entre divisiones. 370 clubes / +4400 jugadores en total
- [x] M21: Agrupación por división en selección de club — `ClubRow` incluye division/tier (ligas) y NewGame agrupa los clubes por país y luego por división (1ª/2ª/3ª/Sin liga) con cabecera de equipos por grupo
- [x] M22: Acceso directo desde equipo a jugador — en `ClubEditor`, clic en un jugador de la plantilla abre `PlayerEditor` inline; al guardar/cerrar vuelve al editor del equipo

---

## Registro de avances

| Fecha | Hito | Notas |
|---|---|---|
| 2026-08-26 | Docs | Plan de desarrollo definido y aprobado |
| 2026-08-26 | M1 | Scaffold Tauri v2 + React + TS + Tailwind + Konva/Zustand. `cargo check` limpio. |
| 2026-08-26 | M2 | SQLite WAL, 20+ tablas, indices, test migración OK (23 tablas). |
| 2026-08-26 | M3 | Mundo: 3 confederaciones, 3 naciones, ciudades, 46 estadios/clubes/finanzas/tácticas, 552 jugadores con atributos/contratos, test_counts OK. |
| 2026-08-26 | M4 | Calendarios round-robin doble: 662 partidos generados, algoritmo círculo, idempotente, balance y tests OK. |
| 2026-08-26 | M5 | Motor 2D: movimiento táctico, duelos, cálculo de gol por distancia/ángulo, faltas acumulativas, doble-penalti, powerplay, cambios por fatiga, simulate_clubs desde DB. 5 tests OK. |
| 2026-08-26 | M6 | Simulación: `advance_day` headless multi-liga, actualización de clasificaciones y posiciones, eventos a DB. Corrección crítica: `current_date` era keyword SQLite (reenamed a `game_date`), FK `club_id` en eventos. Full season 662/662 OK. |
| 2026-08-26 | M7 | API Tauri: 9 comandos tipados, pool persistente en AppState, new_game recrea file DB, fix `current_date`→`game_date` en todos los queries. |
| 2026-08-26 | M8 | Frontend: 5 pantallas (NewGame, Dashboard, Squad, Standings, Fixtures) + FutsalPitch Konva placeholder, Zustand + api.ts, build 457 kB OK. |
| 2026-08-26 | M9 | Partido en vivo interactivo: snapshot 30 fps, polling tick_live, render Konva con colores de club, fatiga y posesión. |
| 2026-08-26 | M10 | Mercado + Inbox: 20 jugadores aleatorios, valoración, ofertas pending/accepted/rejected, generación automática cada día (15%). |
| 2026-08-26 | M11 | Entrenamientos: 8 tipos, schedule L-V, proceso semanal con mejora `edad×potencial×profesionalidad`, lesiones 0.8%/jugador/semana. |
| 2026-08-26 | M12 | Finanzas: salarios semanales, taquilla por partido casa, patrocinio €15k+rep, inbox alerta si balance <0. |
| 2026-08-26 | M13 | Rollover: campeones con premios €150k/80k/40k, retiradas y cantera 17 años, nuevo calendario 2027/28 y game_date 2027-07-10. |
| 2026-08-26 | M14 | README + `pnpm run build` 479 kB y `cargo check` verdes, 12 tests, push a main. |
| 2026-08-26 | M15 | Mundo PRD: 37 competiciones (19 ligas + 7 internacionales + 11 extras), 26 naciones (6 confed), 222 clubes, +2600 jugadores. Fix crash memoria: referencias dangling por `unsafe transmute` en `owned_defs`. |
| 2026-08-26 | M16 | Editor: CRUD naciones/clubes/jugadores/competiciones, `editor_init` auto-seed, pestaña Editor en nav, botón NewGame. |
| 2026-08-26 | M17 | Editor FM: migración 003 (staff + clubs.crest_path/coach_id), subida escudo local, entrenador/staff por club, gestión plantilla (fichar/liberar), asset protocol para escudos. |
| 2026-08-26 | M18 | Migración 004 (fotos/banderas: players.photo_path, staff.photo_path, nations.flag_path, confederations.crest_path) + ImagePicker, PlayerEditor (atributos), StaffEditor, NationEditor (bandera+federación), buscador por pestaña. |
| 2026-08-26 | M19 | Migración 005 (competitions.kind) + 2ªs divisiones (ES/BR/PT, Vysshaya, A2, FR-D2, AR-2ª) -> 43 comps; toggle Clubes/Selecciones en inicio y clasificación/calendario. |
| 2026-08-26 | M20 | Seed con pirámide real: clubes por nación = suma de equipos de sus divisiones; cada división con su propio grupo de clubes (España 92). Test verifica 16/16/60 sin solapamiento. |
| 2026-08-26 | M21 | ClubRow con division/tier; NewGame agrupa por país y, dentro, por división (1ª/2ª/3ª/Sin liga). |
| 2026-08-26 | M22 | ClubEditor -> clic en jugador de plantilla abre PlayerEditor; al cerrar vuelve al equipo. |
