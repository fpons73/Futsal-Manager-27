import { useEffect, useState, useRef } from "react";
import { api, type MatchSnapshot } from "../../api";
import { useStore } from "../../store";
import FutsalPitch from "../FutsalPitch";

export default function LiveMatch({ initial, onBackToSetup }: { initial?: MatchSnapshot | null; onBackToSetup?: () => void }) {
  const { userClubId } = useStore();
  const [snap, setSnap] = useState<MatchSnapshot | null>(initial ?? null);
  const [running, setRunning] = useState(Boolean(initial));
  const [speed, setSpeed] = useState<1 | 2 | 5>(1);
  const intervalRef = useRef<number | null>(null);

  const start = async () => {
    if (!userClubId) return;
    const nxt = await api.getNextFixture(userClubId);
    if (!nxt) { alert("No hay partido pendiente"); return; }
    const s = await api.startLive(nxt.id);
    setSnap(s);
    setRunning(true);
  };

  useEffect(() => {
    if (!running) { if (intervalRef.current) window.clearInterval(intervalRef.current); return; }
    const ms = speed === 1 ? 500 : speed === 2 ? 250 : 120;
    intervalRef.current = window.setInterval(async () => {
      try {
        const s = await api.tickLive(speed * 2);
        setSnap(s);
        if (s.state === "Finished") setRunning(false);
      } catch { setRunning(false); }
    }, ms);
    return () => { if (intervalRef.current) window.clearInterval(intervalRef.current); };
  }, [running, speed]);

  const fmt = (sec: number) => `${String(Math.floor(sec/60)).padStart(2,"0")}:${String(sec%60).padStart(2,"0")}`;

  if (!snap) {
    return (
      <div className="mx-auto max-w-6xl p-6 text-center">
        <h2 className="mb-4 text-xl font-black">Partido en vivo</h2>
        <p className="mb-4 text-sm text-fm-dim">Simulación 2D con motor Rust: faltas, doble penalti, powerplay y cambios volantes.</p>
        {onBackToSetup ? (
          <button onClick={onBackToSetup} className="rounded-lg bg-fm-accent px-6 py-3 font-bold text-black">Configurar tácticas</button>
        ) : (
          <button onClick={start} className="rounded-lg bg-fm-accent px-6 py-3 font-bold text-black">Iniciar próximo partido</button>
        )}
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-6xl space-y-4 p-6">
      <div className="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-fm-border bg-fm-panel p-3">
        <div className="flex items-center gap-4">
          <span className="font-mono text-lg font-black">{snap.score[0]} - {snap.score[1]}</span>
          <span className="rounded bg-fm-bg px-2 py-1 font-mono text-sm">{fmt(snap.time_seconds)} · {snap.state}</span>
          <span className="text-xs text-fm-dim">Faltas {snap.fouls[0]}-{snap.fouls[1]} · Tiros {snap.shots[0]}-{snap.shots[1]} · Pos {snap.possession[0]}%/{snap.possession[1]}%</span>
        </div>
        <div className="flex items-center gap-2">
          <button onClick={() => setRunning(!running)} className="rounded-lg bg-fm-accent px-4 py-1.5 text-sm font-bold text-black">{running ? "Pausar" : "Reanudar"}</button>
          <select value={speed} onChange={(e)=>setSpeed(Number(e.target.value) as any)} className="rounded-lg border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
            <option value={1}>x1</option>
            <option value={2}>x2</option>
            <option value={5}>x5</option>
          </select>
        </div>
      </div>

      <FutsalPitch snap={snap} />

      <div className="grid gap-4 lg:grid-cols-2">
        <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
          <h3 className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Eventos</h3>
          <div className="max-h-48 space-y-1 overflow-auto">
            {snap.events.slice().reverse().slice(0, 12).map((e,i)=>(
              <div key={i} className="rounded bg-fm-bg px-2 py-1 font-mono text-xs">{String(e.minute).padStart(2,"0")}' {e.kind}: {e.description}</div>
            ))}
          </div>
        </div>
        <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
          <h3 className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Jugadores en pista</h3>
          <div className="grid grid-cols-2 gap-1 text-xs">
            {snap.players.filter((p)=>p.on_pitch).map((p)=>(
              <div key={p.id} className="flex items-center justify-between rounded bg-fm-bg px-2 py-1">
                <span>#{p.shirt} {p.role} {p.stamina < 40 ? "🔴" : p.stamina < 60 ? "🟡" : "🟢"}</span>
                <span className="font-mono">{Math.round(p.stamina)}%</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
