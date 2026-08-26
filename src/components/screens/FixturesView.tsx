import { useEffect, useState } from "react";
import { api, type FixtureRow } from "../../api";
import { useStore } from "../../store";

export default function FixturesView() {
  const { competitions, selectedComp, setSelectedComp } = useStore();
  const [rows, setRows] = useState<FixtureRow[]>([]);
  const sel = selectedComp ?? competitions[0]?.id ?? 1;
  useEffect(()=>{ if(sel) api.getFixtures(sel).then(setRows).catch(()=>{}); },[sel]);

  const byRound = rows.reduce<Record<number, FixtureRow[]>>((acc, r)=>{ (acc[r.round]??=[]).push(r); return acc; },{});

  return (
    <div className="mx-auto max-w-5xl p-6">
      <div className="mb-4 flex items-center justify-between">
        <h2 className="text-xl font-black">Calendario</h2>
        <select value={sel} onChange={(e)=>setSelectedComp(Number(e.target.value))} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-1.5 text-sm">
          {competitions.map((c)=><option key={c.id} value={c.id}>{c.name}</option>)}
        </select>
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
