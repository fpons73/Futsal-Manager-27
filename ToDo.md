# FUTSAL MANAGER 27 — Estado del Proyecto

> Este archivo se actualiza con cada hito completado. Cada hito se sube a GitHub.
> Repo: https://github.com/fpons73/Futsal-Manager-27.git

---

## Hitos

- [x] Documentos base (PRD V2 + Prototipo Técnico)
- [x] plan.md creado
- [x] M1: Scaffold Tauri v2 + React + TS compilando — `npm run build` + `cargo check` limpios
- [x] M2: Base de datos SQLite (migraciones + pool) — 001_initial.sql con 20+ tablas, WAL+FK, test `migration_creates_tables` OK
- [ ] M3: Generación procedural del mundo (3 ligas)
- [x] M3: Generación procedural del mundo (3 ligas) — 46 clubes, 552 jugadores, 3 competiciones, test OK
- [x] M4: Competiciones y calendario round-robin — doble robin 662 partidos (240+240+182), balance verificado, 30/30/26 jornadas, tests OK
- [ ] M5: Motor de partido 2D con reglas futsal
- [x] M5: Motor de partido 2D con reglas futsal — ECS-lite, fatiga, faltas/6ª doble-penalti, powerplay, cambios volantes, tests DB OK
- [x] M6: Procesador de tiempo y simulación multi-liga — advance_day con engine, standings y posiciones, full season 662 partidos, bugfix `current_date`→`game_date` (keyword SQLite) y FK club_id, 12 tests OK
- [ ] M7: API de comandos Tauri completa
- [ ] M8: Frontend gestión (dashboard, plantilla, tácticas, ligas)
- [ ] M9: Partido en vivo con campo Konva
- [ ] M10: Mercado de fichajes + bandeja de entrada
- [ ] M11: Entrenamientos, progresión, lesiones y sanciones
- [ ] M12: Finanzas del club
- [ ] M13: Fin de temporada y rollover
- [ ] M14: Pulido, tests, clippy/tsc limpios, README

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
