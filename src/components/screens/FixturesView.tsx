import { useEffect, useState } from "react";
import { api, type FixtureRow } from "../../api";
import { useStore } from "../../store";

export default function FixturesView() {
  const { competitions, selectedComp, setSelectedComp } = useStore();
  const [rows, setRows] = useState<FixtureRow[]>([]);
  const [kind, setKind] = useState<"clubs" | "selecciones">("clubs");
  const opts = kind === "clubs" ? competitions.filter((c)=>c.kind === "club") : competitions.filter((c)=>c.kind === "national_team");
  const sel = selectedComp && opts.some((c)=>c.id===selectedComp) ? selectedComp : (opts[0]?.id ?? 1);
  useEffect(()=>{ if(sel) api.getFixtures(sel).then(setRows).catch(()=>{}); },[sel, kind]);

  const byRound = rows.reduce<Record<number, FixtureRow[]>>((acc, r)=>{ (acc[r.round]??=[]).push(r); return acc; },{});

  return (
    <div className="mx-auto max-w-5xl p-6">
      <div className="mb-4 flex flex-wrap items-center justify-between gap-3">
        <h2 className="text-xl font-black">Calendario</h2>
        <div className="flex items-center gap-2">
          <div className="inline-flex rounded-full border border-fm-border bg-fm-panel p-0.5">
            <button onClick={()=>setKind("clubs")} className={`rounded-full px-3 py-1 text-xs font-bold ${kind==="clubs" ? "bg-fm-accent text-black" : "text-fm-dim"}`}>Clubes</button>
            <button onClick={()=>setKind("selecciones")} className={`rounded-full px-3 py-1 text-xs font-bold ${kind==="selecciones" ? "bg-fm-accent text-black" : "text-fm-dim"}`}>Selecciones</button>
          </div>
          <select value={sel} onChange={(e)=>setSelectedComp(Number(e.target.value))} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-1.5 text-sm">
            {opts.map((c)=><option key={c.id} value={c.id}>{c.name}</option>)}
          </select>
        </div>
      </div>
      <div className="space-y-4">
        {Object.entries(byRound).map(([round, fixtures])=>(
          <div key={round} className="rounded-xl border border-fm-border bg-fm-panel p-3">
            <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Jornada {round} · {fixtures[0]?.date}</div>
            <div className="grid gap-2 sm:grid-cols-2">
              {fixtures.map((f)=>(
                <div key={f.id} className="flex items-center justify-between rounded-lg bg-fm-bg px-3 py-2 text-sm">
                  <span className="font-semibold">{f.home_short}</span>
                  {f.status==="finished" ? <span className="rounded bg-fm-panel px-2 py-0.5 font-mono font-bold">{f.home_score}-{f.away_score}</span> : <span className="text-xs text-fm-dim">{f.date}</span>}
                  <span className="font-semibold">{f.away_short}</span>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
