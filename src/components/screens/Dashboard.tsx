import { useEffect, useState } from "react";
import { api, type FixtureRow, type StandingRow } from "../../api";
import { useStore } from "../../store";

export default function Dashboard() {
  const { gameState, userClubId, clubs, competitions, selectedComp } = useStore();
  const [next, setNext] = useState<FixtureRow | null>(null);
  const [standings, setStandings] = useState<StandingRow[]>([]);
  const [advancing, setAdvancing] = useState(false);
  const [events, setEvents] = useState<string[]>([]);

  const myClub = clubs.find((c) => c.id === userClubId);
  const myComp = competitions.find((c) => c.id === selectedComp) ?? competitions[0];

  useEffect(() => {
    if (!userClubId) return;
    api.getNextFixture(userClubId).then(setNext).catch(()=>{});
  }, [userClubId, gameState?.game_date]);

  useEffect(() => {
    if (!myComp) return;
    api.getStandings(myComp.id).then(setStandings).catch(()=>{});
  }, [myComp, gameState?.game_date]);

  const advance = async (days: number) => {
    setAdvancing(true);
    try {
      if (days === 1) {
        const r = await api.advanceDay();
        setEvents((e) => [...r.results, ...e].slice(0, 12));
        const st = await api.getGameState(); useStore.getState().setGameState(st);
      } else {
        const rs = await api.advanceWeek();
        const all = rs.flatMap((r) => r.results);
        setEvents((e) => [...all, ...e].slice(0, 12));
        const st = await api.getGameState(); useStore.getState().setGameState(st);
      }
    } catch (e) { alert(String(e)); }
    finally { setAdvancing(false); }
  };

  if (!gameState || !myClub) return <div className="p-8 text-center text-fm-dim">Cargando…</div>;

  const myStanding = standings.find((s) => s.club_id === userClubId);

  return (
    <div className="mx-auto max-w-6xl space-y-6 p-6">
      <div className="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-fm-border bg-fm-panel p-4">
        <div>
          <div className="text-xs uppercase tracking-widest text-fm-dim">{gameState.season} · Jornada {next?.round ?? "—"}</div>
          <div className="text-2xl font-black">{myClub.name} <span className="font-normal text-fm-dim">({myClub.short_name})</span></div>
        </div>
        <div className="flex items-center gap-3">
          <div className="rounded-lg bg-fm-bg px-4 py-2 text-center">
            <div className="text-xs text-fm-dim">Fecha</div>
            <div className="font-mono font-bold">{gameState.game_date}</div>
          </div>
          <button onClick={() => advance(1)} disabled={advancing} className="rounded-lg bg-fm-accent px-4 py-2.5 text-sm font-bold text-black hover:brightness-110 disabled:opacity-50">Avanzar 1 día</button>
          <button onClick={() => advance(7)} disabled={advancing} className="rounded-lg border border-fm-border bg-fm-panel2 px-4 py-2.5 text-sm font-semibold hover:bg-fm-border disabled:opacity-50">+7 días</button>
        </div>
      </div>

      <div className="grid gap-4 lg:grid-cols-3">
        <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
          <h3 className="mb-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Próximo partido</h3>
          {next ? (
            <div className="space-y-2">
              <div className="text-xs text-fm-dim">{next.date} · J {next.round}</div>
              <div className="flex items-center justify-between rounded-lg bg-fm-bg p-3">
                <span className={`font-bold ${next.home_id===userClubId ? "text-fm-accent" : ""}`}>{next.home_short}</span>
                <span className="text-fm-dim">vs</span>
                <span className={`font-bold ${next.away_id===userClubId ? "text-fm-accent" : ""}`}>{next.away_short}</span>
              </div>
              <div className="text-xs text-fm-dim">{next.home_name} — {next.away_name}</div>
            </div>
          ) : <div className="text-sm text-fm-dim">Sin partidos pendientes</div>}
        </div>

        <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
          <h3 className="mb-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Clasificación · {myComp?.name ?? ""}</h3>
          <div className="space-y-1 text-sm">
            {standings.slice(0, 6).map((s) => (
              <div key={s.club_id} className={`flex items-center justify-between rounded px-2 py-1 ${s.club_id===userClubId ? "bg-fm-accent/15 font-bold" : "hover:bg-fm-bg"}`}>
                <span className="flex items-center gap-2"><span className="w-5 text-fm-dim">{s.position}.</span> {s.short_name}</span>
                <span className="font-mono">{s.points} pts <span className="text-fm-dim">({s.played} PJ)</span></span>
              </div>
            ))}
            {myStanding && myStanding.position > 6 && (
              <div className="flex items-center justify-between rounded bg-fm-accent/15 px-2 py-1 font-bold">
                <span className="flex items-center gap-2"><span className="w-5 text-fm-dim">{myStanding.position}.</span> {myStanding.short_name} (tú)</span>
                <span className="font-mono">{myStanding.points} pts</span>
              </div>
            )}
          </div>
        </div>

        <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
          <h3 className="mb-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Últimos resultados</h3>
          {events.length === 0 ? <div className="text-sm text-fm-dim">Avanza días para ver resultados.</div> : (
            <div className="space-y-1.5">
              {events.map((ev, i) => <div key={i} className="rounded bg-fm-bg px-2 py-1.5 font-mono text-xs">{ev}</div>)}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
