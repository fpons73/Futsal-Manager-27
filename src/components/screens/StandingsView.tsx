import { useEffect, useState } from "react";
import { api, type StandingRow } from "../../api";
import { useStore } from "../../store";

export default function StandingsView() {
  const { competitions, selectedComp, setSelectedComp, userClubId } = useStore();
  const [rows, setRows] = useState<StandingRow[]>([]);
  const sel = selectedComp ?? competitions[0]?.id ?? 1;

  useEffect(() => { if (sel) api.getStandings(sel).then(setRows).catch(()=>{}); }, [sel]);

  return (
    <div className="mx-auto max-w-5xl p-6">
      <div className="mb-4 flex flex-wrap items-center justify-between gap-3">
        <h2 className="text-xl font-black">Clasificación</h2>
        <select value={sel} onChange={(e) => setSelectedComp(Number(e.target.value))} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-1.5 text-sm">
          {competitions.map((c) => <option key={c.id} value={c.id}>{c.name} · {c.nation}</option>)}
        </select>
      </div>
      <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
        <table className="w-full text-sm">
          <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim">
            <tr><th className="px-3 py-2 text-left">#</th><th className="px-2 py-2 text-left">Club</th><th className="px-2 py-2">PJ</th><th className="px-2 py-2">G</th><th className="px-2 py-2">E</th><th className="px-2 py-2">P</th><th className="px-2 py-2">GF</th><th className="px-2 py-2">GC</th><th className="px-2 py-2">DG</th><th className="px-2 py-2">Pts</th></tr>
          </thead>
          <tbody>
            {rows.map((r) => (
              <tr key={r.club_id} className={`border-t border-fm-border ${r.club_id===userClubId ? "bg-fm-accent/10 font-bold" : "hover:bg-fm-panel2"}`}>
                <td className="px-3 py-2 font-mono">{r.position}</td>
                <td className="px-2 py-2">{r.club_name} <span className="text-fm-dim">({r.short_name})</span></td>
                <td className="px-2 py-2 text-center">{r.played}</td>
                <td className="px-2 py-2 text-center text-emerald-400">{r.won}</td>
                <td className="px-2 py-2 text-center text-amber-400">{r.drawn}</td>
                <td className="px-2 py-2 text-center text-red-400">{r.lost}</td>
                <td className="px-2 py-2 text-center">{r.goals_for}</td>
                <td className="px-2 py-2 text-center">{r.goals_against}</td>
                <td className="px-2 py-2 text-center font-mono">{r.goal_difference > 0 ? `+${r.goal_difference}` : r.goal_difference}</td>
                <td className="px-2 py-2 text-center font-black">{r.points}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
